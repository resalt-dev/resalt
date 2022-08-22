use super::SaltAPI;
use crate::prelude::*;
use crate::storage::SConfig;
use futures::{pin_mut, StreamExt};
use log::*;
use regex::Regex;
use serde_json::Value;

pub const RESALT_SALT_SYSTEM_SERVICE_USERNAME: &str = "$superadmin/svc/resalt$";

lazy_static::lazy_static! {
    static ref REGEX_JOB_NEW: Regex = Regex::new("salt/job/([0-9]+)/new").unwrap();
    static ref REGEX_JOB_RETURN: Regex = Regex::new("salt/job/([0-9]+)/ret/(.+)").unwrap();
}

pub struct SaltEventListener {
    api: SaltAPI,
    pipeline: PipelineServer,
    storage: Storage,
}

impl SaltEventListener {
    pub fn new(pipeline: PipelineServer, storage: Storage) -> Self {
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

            // Unwrap string to JSON structure
            let data: Value = serde_json::from_str(&event.data).unwrap();
            let data = data.get("data").unwrap().as_object().unwrap();

            // Unpack timestamp
            let time = data["_stamp"].as_str().unwrap();
            let time = chrono::NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%S.%f").unwrap();

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
                let jid = capture.get(1).unwrap().as_str().to_string();
                let user = data["user"].as_str().map(|s| s.to_string());

                // Insert job into database
                match self.storage.insert_job(jid, user, Some(event_id), time) {
                    Ok(_) => (),
                    Err(err) => error!("Failed to insert job: {:?}", err),
                }
            } else if let Some(capture) = REGEX_JOB_RETURN.captures(&event.tag) {
                // Assumed always present
                let jid = capture.get(1).unwrap().as_str().to_string();
                let minion_id = capture.get(2).unwrap().as_str().to_string();
                let fun = data["fun"].as_str().unwrap();
                let fun_args = data["fun_args"].as_array().unwrap();

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
                        let minion_id = data["id"].as_str().unwrap().to_string();
                        let grains = data.get("return").unwrap().as_object().unwrap();
                        let grains = serde_json::to_string(grains).unwrap();
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
                        let minion_id = data["id"].as_str().unwrap().to_string();
                        let pillar = data.get("return").unwrap().as_object().unwrap();
                        let pillar = serde_json::to_string(pillar).unwrap();
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
                        let minion_id = data["id"].as_str().unwrap().to_string();
                        let pkgs = data.get("return").unwrap().as_object().unwrap();
                        let pkgs = serde_json::to_string(pkgs).unwrap();
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
                        let is_highstate = fun_args.is_empty()
                            || (fun_args.len() == 1
                                && ((fun_args[0].is_string()
                                    && fun_args[0].as_str().unwrap() == "test=True")
                                    || (fun_args[0].is_object()
                                        && fun_args[0]
                                            .as_object()
                                            .unwrap()
                                            .get("test")
                                            .unwrap()
                                            .as_str()
                                            .unwrap()
                                            .to_lowercase()
                                            == "true")));
                        if !is_highstate {
                            continue;
                        }
                        let retcode = data["retcode"].as_i64().unwrap();
                        if retcode == 1 {
                            continue;
                        }

                        let minion_id = data["id"].as_str().unwrap().to_string();

                        // Loop over return's and count success/incorrect/error
                        let mut success = 0;
                        let mut incorrect = 0;
                        let mut error = 0;

                        let ret = data.get("return").unwrap().as_object().unwrap();
                        for item in ret.values() {
                            let item = match item.as_object() {
                                Some(item) => item,
                                None => continue,
                            };
                            match item.get("result").unwrap().as_bool() {
                                Some(true) => success += 1,
                                Some(false) => error += 1,
                                None => incorrect += 1, // test=True mode, result will be Null
                            };
                        }

                        match self.storage.update_minion_conformity(
                            minion_id.clone(),
                            time,
                            data.get("return").unwrap().to_string(),
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
                let result = data.get("result").unwrap().as_bool().unwrap();
                if !result {
                    continue;
                }

                let minion_id = data["id"].as_str().unwrap().to_string();
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
