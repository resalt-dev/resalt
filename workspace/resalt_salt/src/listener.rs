use super::SaltAPI;
use chrono::NaiveDateTime;
use futures::{pin_mut, StreamExt};
use log::*;
use regex::Regex;
use resalt_config::SConfig;
use resalt_models::SaltToken;
use resalt_pipeline::PipelineServer;
use resalt_storage::StorageImpl;
use serde_json::Value;

pub const RESALT_SALT_SYSTEM_SERVICE_USERNAME: &str = "$superadmin/svc/resalt$";

lazy_static::lazy_static! {
    static ref REGEX_JOB_NEW: Regex = Regex::new("salt/job/([0-9]+)/new").unwrap();
    static ref REGEX_JOB_RETURN: Regex = Regex::new("salt/job/([0-9]+)/ret/(.+)").unwrap();
}

const TIME_FMT: &str = "%Y-%m-%dT%H:%M:%S.%f";

pub struct SaltEventListener {
    api: SaltAPI,
    pipeline: PipelineServer,
    storage: Box<dyn StorageImpl>,
}

impl SaltEventListener {
    pub fn new(pipeline: PipelineServer, storage: Box<dyn StorageImpl>) -> Self {
        Self {
            api: SaltAPI::new(),
            pipeline,
            storage,
        }
    }

    async fn refresh_token(&self) -> Option<SaltToken> {
        match self
            .api
            .login(
                RESALT_SALT_SYSTEM_SERVICE_USERNAME,
                &SConfig::salt_api_system_service_token(),
            )
            .await
        {
            Ok(token) => Some(token),
            Err(err) => {
                error!("Failed to refresh token: {:?}", err);
                None
            }
        }
    }

    async fn listen(&self) {
        let salt_token = match self.refresh_token().await {
            Some(token) => token,
            None => {
                error!("Failed to refresh listener token");
                return;
            }
        };

        let stream = self.api.listen_events(&salt_token);
        pin_mut!(stream);

        while let Some(event) = stream.next().await {
            debug!("{:?}", event);

            // Unpack string to JSON structure
            let data: Value = match serde_json::from_str(&event.data) {
                Ok(data) => data,
                Err(err) => {
                    error!("Failed to parse event data: {:?}", err);
                    continue;
                }
            };
            let data = match data.get("data") {
                Some(data) => data,
                None => {
                    error!("Failed to get data from event data");
                    continue;
                }
            };
            let data = match data.as_object() {
                Some(data) => data,
                None => {
                    error!("Failed to get data as object");
                    continue;
                }
            };

            // Unpack timestamp
            let time = match data.get("_stamp") {
                Some(time) => match time.as_str() {
                    Some(time) => match NaiveDateTime::parse_from_str(time, TIME_FMT) {
                        Ok(time) => time,
                        Err(err) => {
                            error!("Failed to parse timestamp: {:?}", err);
                            continue;
                        }
                    },
                    None => {
                        error!("Failed to get timestamp from event data");
                        continue;
                    }
                },
                None => {
                    error!("Failed to get timestamp from event data");
                    continue;
                }
            };

            // Insert event into database
            let event_id = match self
                .storage
                .insert_event(event.tag.clone(), event.data, time)
            {
                Ok(uuid) => uuid,
                Err(err) => {
                    error!("Failed to insert event: {:?}", err);
                    continue;
                }
            };

            // Check tag type
            if let Some(capture) = REGEX_JOB_NEW.captures(&event.tag) {
                // Assumed always present
                let jid = match capture.get(1) {
                    Some(jid) => jid.as_str().to_string(),
                    None => {
                        error!("Failed to get JID from event tag");
                        continue;
                    }
                };
                let user = match data.get("user") {
                    Some(user) => user.as_str().map(|s| s.to_string()),
                    None => {
                        error!("Failed to get user from event data");
                        continue;
                    }
                };

                // Insert job into database
                match self.storage.insert_job(jid, user, Some(event_id), time) {
                    Ok(_) => (),
                    Err(err) => error!("Failed to insert job: {:?}", err),
                }
            } else if let Some(capture) = REGEX_JOB_RETURN.captures(&event.tag) {
                // Assumed always present
                let jid = match capture.get(1) {
                    Some(jid) => jid.as_str().to_string(),
                    None => {
                        error!("Failed to get JID from event tag");
                        continue;
                    }
                };
                let minion_id = match capture.get(2) {
                    Some(minion_id) => minion_id.as_str().to_string(),
                    None => {
                        error!("Failed to get minion ID from event tag");
                        continue;
                    }
                };
                let fun = match data.get("fun") {
                    Some(fun) => match fun.as_str() {
                        Some(fun) => fun,
                        None => {
                            error!("Failed to get function from event data");
                            continue;
                        }
                    },
                    None => {
                        error!("Failed to get function from event data");
                        continue;
                    }
                };
                let fun_args = match data.get("fun_args") {
                    Some(fun_args) => match fun_args.as_array() {
                        Some(fun_args) => fun_args,
                        None => {
                            error!("Failed to get function arguments from event data");
                            continue;
                        }
                    },
                    None => {
                        error!("Failed to get function arguments from event data");
                        continue;
                    }
                };

                // Insert job return into database
                match self.storage.get_job_by_jid(&jid) {
                    Ok(job) => match job {
                        Some(job) => {
                            let job_id = job.id;
                            match self
                                .storage
                                .insert_job_return(jid, job_id, event_id, minion_id, time)
                            {
                                Ok(_) => (),
                                Err(err) => error!("Failed to insert job return: {:?}", err),
                            }
                        }
                        None => {
                            warn!("Failed to get job by jid: {}", jid);
                            continue;
                        }
                    },
                    Err(err) => {
                        error!("Failed to get job by jid: {:?}", err);
                    }
                };

                debug!("salt event job fun: {:?}", fun);
                match fun {
                    "grains.items" => {
                        let minion_id = match data.get("id") {
                            Some(minion_id) => match minion_id.as_str() {
                                Some(minion_id) => minion_id.to_string(),
                                None => {
                                    error!("Failed to get minion ID from event data");
                                    continue;
                                }
                            },
                            None => {
                                error!("Failed to get minion ID from event data");
                                continue;
                            }
                        };
                        let grains = match data.get("return") {
                            Some(grains) => match grains.as_object() {
                                Some(grains) => match serde_json::to_string(grains) {
                                    Ok(grains) => grains,
                                    Err(err) => {
                                        error!("Failed to serialize grains: {:?}", err);
                                        continue;
                                    }
                                },
                                None => {
                                    error!("Failed to get grains from event data");
                                    continue;
                                }
                            },
                            None => {
                                error!("Failed to get grains from event data");
                                continue;
                            }
                        };
                        match self
                            .storage
                            .update_minion_grains(minion_id.clone(), time, grains)
                        {
                            Ok(_) => {
                                self.pipeline_update_minion(&minion_id).await;
                            }
                            Err(e) => error!("Failed updating minion grains {:?}", e),
                        }
                    }
                    "pillar.items" => {
                        let minion_id = match data.get("id") {
                            Some(minion_id) => match minion_id.as_str() {
                                Some(minion_id) => minion_id.to_string(),
                                None => {
                                    error!("Failed to get minion ID from event data");
                                    continue;
                                }
                            },
                            None => {
                                error!("Failed to get minion ID from event data");
                                continue;
                            }
                        };
                        let pillar = match data.get("return") {
                            Some(pillar) => match pillar.as_object() {
                                Some(pillar) => match serde_json::to_string(pillar) {
                                    Ok(pillar) => pillar,
                                    Err(err) => {
                                        error!("Failed to serialize pillar: {:?}", err);
                                        continue;
                                    }
                                },
                                None => {
                                    error!("Failed to get pillar from event data");
                                    continue;
                                }
                            },
                            None => {
                                error!("Failed to get pillar from event data");
                                continue;
                            }
                        };
                        match self
                            .storage
                            .update_minion_pillars(minion_id.clone(), time, pillar)
                        {
                            Ok(_) => {
                                self.pipeline_update_minion(&minion_id).await;
                            }
                            Err(e) => error!("Failed updating minion pillar {:?}", e),
                        }
                    }
                    "pkg.list_pkgs" => {
                        let minion_id = match data.get("id") {
                            Some(minion_id) => match minion_id.as_str() {
                                Some(minion_id) => minion_id.to_string(),
                                None => {
                                    error!("Failed to get minion ID from event data");
                                    continue;
                                }
                            },
                            None => {
                                error!("Failed to get minion ID from event data");
                                continue;
                            }
                        };
                        let pkgs = match data.get("return") {
                            Some(pkgs) => match pkgs.as_object() {
                                Some(pkgs) => match serde_json::to_string(pkgs) {
                                    Ok(pkgs) => pkgs,
                                    Err(err) => {
                                        error!("Failed to serialize pkgs: {:?}", err);
                                        continue;
                                    }
                                },
                                None => {
                                    error!("Failed to get pkgs from event data");
                                    continue;
                                }
                            },
                            None => {
                                error!("Failed to get pkgs from event data");
                                continue;
                            }
                        };
                        match self
                            .storage
                            .update_minion_pkgs(minion_id.clone(), time, pkgs)
                        {
                            Ok(_) => {
                                self.pipeline_update_minion(&minion_id).await;
                            }
                            Err(e) => error!("Failed updating minion pkgs {:?}", e),
                        }
                    }
                    "state.apply" | "state.highstate" => {
                        // Check if empty args, or if empty args but test=True is only argument.
                        // If so, then we can assume this is a highstate run.
                        let only_arg_is_test_true = match fun_args.get(0) {
                            Some(arg) => match arg.is_string() {
                                // If the arg is a string, check if it's test=True
                                true => match arg.as_str() {
                                    Some(arg) => arg.to_lowercase() == "test=true",
                                    None => false,
                                },
                                // If the arg is an object, check if it's test: True
                                false => match arg.is_object() {
                                    true => match arg.get("test") {
                                        // Value can be both string or bool
                                        Some(test) => match test.is_string() {
                                            true => match test.as_str() {
                                                Some(test) => test.to_lowercase() == "true",
                                                None => false,
                                            },
                                            false => match test.is_boolean() {
                                                true => test.as_bool().unwrap_or(false),
                                                false => false,
                                            },
                                        },
                                        None => false,
                                    },
                                    false => false,
                                },
                            },
                            None => false,
                        };
                        let no_args = fun_args.is_empty();
                        let is_highstate = no_args || only_arg_is_test_true;
                        if !is_highstate {
                            continue;
                        }
                        let retcode = match data.get("retcode") {
                            Some(retcode) => match retcode.as_i64() {
                                Some(retcode) => retcode,
                                None => {
                                    error!("Failed to get retcode from event data");
                                    continue;
                                }
                            },
                            None => {
                                error!("Failed to get retcode from event data");
                                continue;
                            }
                        };
                        if retcode == 1 {
                            continue;
                        }

                        let minion_id = match data.get("id") {
                            Some(minion_id) => match minion_id.as_str() {
                                Some(minion_id) => minion_id.to_string(),
                                None => {
                                    error!("Failed to get minion ID from event data");
                                    continue;
                                }
                            },
                            None => {
                                error!("Failed to get minion ID from event data");
                                continue;
                            }
                        };

                        // Loop over return's and count success/incorrect/error
                        let mut success = 0;
                        let mut incorrect = 0;
                        let mut error = 0;

                        let ret = match data.get("return") {
                            Some(ret) => match ret.as_object() {
                                Some(ret) => ret,
                                None => {
                                    error!("Failed to get return from event data");
                                    continue;
                                }
                            },
                            None => {
                                error!("Failed to get return from event data");
                                continue;
                            }
                        };
                        for item in ret.values() {
                            let item = match item.as_object() {
                                Some(item) => item,
                                None => continue,
                            };
                            match item.get("result") {
                                Some(result) => match result.as_bool() {
                                    Some(true) => success += 1,
                                    Some(false) => error += 1,
                                    None => incorrect += 1, // test=True mode, result will be Null
                                },
                                None => {
                                    error!("Failed to get result from event data");
                                    continue;
                                }
                            }
                        }

                        let confirmity: String = match data.get("return") {
                            Some(confirmity) => match confirmity.as_object() {
                                Some(confirmity) => match serde_json::to_string(confirmity) {
                                    Ok(confirmity) => confirmity,
                                    Err(err) => {
                                        error!("Failed to serialize confirmity: {:?}", err);
                                        continue;
                                    }
                                },
                                None => {
                                    error!("Failed to get confirmity from event data");
                                    continue;
                                }
                            },
                            None => {
                                error!("Failed to get confirmity from event data");
                                continue;
                            }
                        };
                        match self.storage.update_minion_conformity(
                            minion_id.clone(),
                            time,
                            confirmity,
                            success,
                            incorrect,
                            error,
                        ) {
                            Ok(_) => {
                                self.pipeline_update_minion(&minion_id).await;
                            }
                            Err(e) => error!("Failed updating minion conformity {:?}", e),
                        }
                    }
                    _ => {}
                }
            } else if event.tag == "salt/auth" {
                let result = match data.get("result") {
                    Some(result) => match result.as_bool() {
                        Some(result) => result,
                        None => {
                            error!("Failed to get result from event data");
                            continue;
                        }
                    },
                    None => {
                        error!("Failed to get result from event data");
                        continue;
                    }
                };
                if !result {
                    continue;
                }

                let minion_id = match data.get("id") {
                    Some(minion_id) => match minion_id.as_str() {
                        Some(minion_id) => minion_id.to_string(),
                        None => {
                            error!("Failed to get minion ID from event data");
                            continue;
                        }
                    },
                    None => {
                        error!("Failed to get minion ID from event data");
                        continue;
                    }
                };
                match self.storage.update_minion_last_seen(minion_id, time) {
                    Ok(_) => {}
                    Err(e) => error!("Failed updating minion last seen {:?}", e),
                }
            } else {
                //warn!("Unhandled event: {:?}", event);
            }
        }

        warn!("Salt event stream ended! Reconnecting stream...");
    }

    async fn pipeline_update_minion(&self, id: &str) {
        let minion = match self.storage.get_minion_by_id(id) {
            Ok(minion) => match minion {
                Some(minion) => minion,
                None => {
                    error!("Minion not found in storage");
                    return;
                }
            },
            Err(e) => {
                error!("Failed to get minion {:?}", e);
                return;
            }
        };
        self.pipeline.update_minion(minion);
    }

    pub async fn start(&self) {
        loop {
            self.listen().await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
