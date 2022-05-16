use super::SaltAPI;
use crate::prelude::*;
use crate::storage::SConfig;
use futures::{pin_mut, StreamExt};
use log::*;
use regex::Regex;
use serde_json::Value;

pub const HIBIKE_SALT_SYSTEM_SERVICE_USERNAME: &str = "$superadmin/svc/hibike$";

lazy_static::lazy_static! {
    //static ref REGEX_JOB_NEW: Regex = Regex::new("salt/job/([0-9]+)/new").unwrap();
    static ref REGEX_JOB_RETURN: Regex = Regex::new("salt/job/([0-9]+)/ret/(.+)").unwrap();
}

pub struct SaltEventListener {
    api: SaltAPI,
    storage: Storage,
}

impl SaltEventListener {
    pub fn new(storage: Storage) -> Self {
        Self {
            api: SaltAPI::new(),
            storage,
        }
    }

    async fn refresh_token(&self) -> Option<SaltToken> {
        match self
            .api
            .login(
                HIBIKE_SALT_SYSTEM_SERVICE_USERNAME,
                &SConfig::salt_api_system_service_token(),
            )
            .await
        {
            Ok(token) => Some(token),
            Err(err) => {
                error!("failed to refresh token: {:?}", err);
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
            // debug!("{:?}", event);

            let data: Value = serde_json::from_str(&event.data).unwrap();
            let data = data.get("data").unwrap().as_object().unwrap();

            if let Some(_job_id) = REGEX_JOB_RETURN.captures(&event.tag) {
                //let job_id = job_id.get(1).unwrap().as_str().to_string();

                // Assumed always present, everything else is optional
                let time = data["_stamp"].as_str().unwrap();
                let time =
                    chrono::NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%S.%f").unwrap();
                let fun = data["fun"].as_str().unwrap();
                let fun_args = data["fun_args"].as_array().unwrap();

                debug!("salt event job fun: {:?}", fun);
                match fun {
                    "grains.items" => {
                        let minion_id = data["id"].as_str().unwrap();
                        let grains = data.get("return").unwrap().as_object().unwrap();
                        let grains = serde_json::to_string(grains).unwrap();
                        match self
                            .storage
                            .update_minion_grains(minion_id, time, &grains)
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => error!("Failed updating minion grains {:?}", e),
                        }
                    }
                    "pillar.items" => {
                        let minion_id = data["id"].as_str().unwrap();
                        let pillar = data.get("return").unwrap().as_object().unwrap();
                        let pillar = serde_json::to_string(pillar).unwrap();
                        match self
                            .storage
                            .update_minion_pillars(minion_id, time, &pillar)
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => error!("Failed updating minion pillar {:?}", e),
                        }
                    }
                    "pkg.list_pkgs" => {
                        let minion_id = data["id"].as_str().unwrap();
                        let pkgs = data.get("return").unwrap().as_object().unwrap();
                        let pkgs = serde_json::to_string(pkgs).unwrap();
                        match self
                            .storage
                            .update_minion_pkgs(minion_id, time, &pkgs)
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => error!("Failed updating minion pkgs {:?}", e),
                        }
                    }
                    "state.apply" | "state.highstate" => {
                        // Check if empty args, or if empty args but test=True is only argument.
                        // If so, then we can assume this is a highstate run.
                        let is_highstate = fun_args.len() == 0
                            || (fun_args.len() == 1
                                && fun_args[0].as_str().unwrap().to_lowercase() == "test=true");
                        if !is_highstate {
                            continue;
                        }
                        let retcode = data["retcode"].as_i64().unwrap();
                        if retcode == 1 {
                            continue;
                        }

                        let minion_id = data["id"].as_str().unwrap();

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

                        match self
                            .storage
                            .update_minion_conformity(
                                minion_id,
                                time,
                                &data.get("return").unwrap().to_string(),
                                success,
                                incorrect,
                                error,
                            )
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => error!("Failed updating minion conformity {:?}", e),
                        }
                    }
                    _ => {}
                }
            } else if event.tag == "salt/auth" {
                let time = data["_stamp"].as_str().unwrap();
                let time =
                    chrono::NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%S.%f").unwrap();

                let result = data.get("result").unwrap().as_bool().unwrap();
                if !result {
                    continue;
                }

                let minion_id = data["id"].as_str().unwrap();
                match self.storage.update_minion_last_seen(minion_id, time).await {
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
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
