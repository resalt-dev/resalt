use super::SaltAPI;
use futures::{pin_mut, StreamExt};
use log::*;
use resalt_config::ResaltConfig;
use resalt_models::{ResaltTime, SaltToken, StorageImpl};
use resalt_storage::Storage;
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub const RESALT_SALT_SYSTEM_SERVICE_USERNAME: &str = "$superadmin/svc/resalt$";

const TIME_FMT: &str = "%Y-%m-%dT%H:%M:%S.%f";

#[derive(Debug, Clone)]
pub struct SaltEventListenerStatus {
    pub connected: Arc<Mutex<bool>>,
}

pub struct SaltEventListener {
    api: SaltAPI,
    storage: Storage,
    status: SaltEventListenerStatus,
}

impl SaltEventListener {
    pub fn new(storage: Storage, status: SaltEventListenerStatus) -> Self {
        Self {
            api: SaltAPI::new(),
            storage,
            status,
        }
    }

    async fn refresh_token(&self) -> Option<SaltToken> {
        match self
            .api
            .login(
                RESALT_SALT_SYSTEM_SERVICE_USERNAME,
                &ResaltConfig::SALT_API_SYSTEM_SERVICE_TOKEN,
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

        {
            // Make sure lock is released after setting status
            *self.status.connected.lock().unwrap() = true;
        }

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
                    Some(time) => match ResaltTime::parse_from_str(time, TIME_FMT) {
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
            let tag_parts: Vec<&str> = event.tag.split('/').collect();
            if tag_parts.len() == 4
                && tag_parts[0] == "salt"
                && tag_parts[1] == "job"
                && tag_parts[3] == "new"
            {
                let jid = tag_parts[2].to_string();
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
            } else if tag_parts.len() == 5
                && tag_parts[0] == "salt"
                && tag_parts[1] == "job"
                && tag_parts[3] == "ret"
            {
                let jid = tag_parts[2].to_string();
                let minion_id = tag_parts[4].to_string();
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
                        let (grains, os_type) = match data.get("return") {
                            Some(grains) => match grains.as_object() {
                                Some(grains) => match serde_json::to_string(grains) {
                                    Ok(grains_str) => {
                                        // Parse grains as JSON, and fetch osfullname+osrelease as os_type.
                                        let osfullname = grains
                                            .get("osfullname")
                                            .map(|s| s.as_str().unwrap_or("Unknown"))
                                            .unwrap_or("Unknown");
                                        let osrelease = grains
                                            .get("osrelease")
                                            .map(|s| s.as_str().unwrap_or(""))
                                            .unwrap_or("");
                                        let os_type = format!("{} {}", osfullname, osrelease)
                                            .trim()
                                            .to_string();
                                        (grains_str, os_type)
                                    }
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
                        match self.storage.upsert_minion_grains(
                            minion_id.clone(),
                            time,
                            grains,
                            os_type,
                        ) {
                            Ok(_) => {}
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
                            .upsert_minion_pillars(minion_id.clone(), time, pillar)
                        {
                            Ok(_) => {}
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
                            .upsert_minion_pkgs(minion_id.clone(), time, pkgs)
                        {
                            Ok(_) => {}
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

                        let conformity = match serde_json::to_string(ret) {
                            Ok(conformity) => conformity,
                            Err(err) => {
                                error!("Failed to serialize conformity: {:?}", err);
                                continue;
                            }
                        };
                        match self.storage.upsert_minion_conformity(
                            minion_id.clone(),
                            time,
                            conformity,
                            success,
                            incorrect,
                            error,
                        ) {
                            Ok(_) => {}
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
                match self.storage.upsert_minion_last_seen(minion_id, time) {
                    Ok(_) => {}
                    Err(e) => error!("Failed updating minion last seen {:?}", e),
                }
            } else {
                //warn!("Unhandled event: {:?}", event);
            }
        }

        warn!("Salt event stream ended! Reconnecting stream...");
    }

    pub async fn start(&self) {
        loop {
            self.listen().await;
            {
                // Make sure lock is released after setting status
                *self.status.connected.lock().unwrap() = false;
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
