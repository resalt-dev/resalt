use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use clokwerk::TimeUnits;
use log::*;
use resalt_config::SConfig;
use resalt_ldap::{LdapHandler, LdapUser};
use resalt_models::User;
use resalt_security::sync_ldap_groups;
use resalt_storage::StorageCloneWrapper;
use tokio::task;

use crate::update;

#[derive(Clone)]
pub struct Scheduler {
    scheduler: Arc<Mutex<clokwerk::Scheduler>>,
    data: StorageCloneWrapper,
}

impl Scheduler {
    pub fn new(data: StorageCloneWrapper) -> Self {
        Scheduler {
            scheduler: Arc::new(Mutex::new(clokwerk::Scheduler::new())),
            data,
        }
    }

    pub fn register_system_jobs(&mut self) {
        // self.scheduler.lock().unwrap().every(5.minutes()).run(|| {
        //     println!("system job");
        // });

        // Run update check
        self.scheduler.lock().unwrap().every(1.hour()).run(|| {
            info!("Running update checker");
            let rt = tokio::runtime::Runtime::new().unwrap();
            let ls = task::LocalSet::new();
            ls.block_on(&rt, async {
                update::get_update_cache(true).await;
            });
        });

        if SConfig::auth_ldap_enabled() {
            // Run LDAP sync
            let wrapper = self.data.clone();
            self.scheduler.lock().unwrap().every(1.hour()).run(move || {
                info!("Running LDAP sync");
                let rt = tokio::runtime::Runtime::new().unwrap();
                let ls = task::LocalSet::new();
                let wrapper = wrapper.clone();
                ls.block_on(&rt, async {
                    // Fetch users from DB
                    let mut db_users = match wrapper.storage.list_users(Some(std::i64::MAX), None) {
                        Ok(users) => users,
                        Err(e) => {
                            error!("Failed to get all users from database: {:?}", e);
                            return;
                        }
                    };
                    db_users.retain(|user| user.ldap_sync.is_some());

                    // Fetch users from LDAP
                    let dns = db_users
                        .iter()
                        .map(|user| user.ldap_sync.clone())
                        // filter where dn.is_some and map to String
                        .filter_map(|sync| sync)
                        .collect::<Vec<String>>();
                    let ldap_users = match LdapHandler::lookup_users_by_dn(dns).await {
                        Ok(users) => users,
                        Err(e) => {
                            error!("Failed to get all users from LDAP: {:?}", e);
                            return;
                        }
                    };

                    // Merge lists
                    let mut users: Vec<(User, Option<&LdapUser>)> = Vec::new();
                    for user in db_users {
                        // We can safely do user.ldap_sync.unwrap(), because we have earlier reduced list to only LDAP users
                        let user_ldap_sync = user.ldap_sync.clone().unwrap();

                        let ldap_user = ldap_users
                            .iter()
                            .find(|ldap_user| ldap_user.dn.eq(&user_ldap_sync));
                        users.push((user, ldap_user));
                    }

                    // Update users
                    for (user, ldap_user) in users {
                        match sync_ldap_groups(&wrapper.storage, &user, ldap_user) {
                            Ok(_) => {}
                            Err(e) => {
                                error!(
                                    "Failed to sync LDAP groups for user {}: {:?}",
                                    user.username, e
                                );
                            }
                        }
                    }
                });
            });
        }
    }

    pub fn start(&mut self) {
        info!("Starting scheduler");
        let scheduler = self.scheduler.clone();
        // Start a thread and manually run the scheduler in an event loop
        std::thread::spawn(move || {
            // LOCK AND UNLOCK INSIDE THE LOOP, so we can use the scheduler while the thread is sleeping
            loop {
                {
                    let mut scheduler = scheduler.lock().unwrap();
                    scheduler.run_pending();
                }
                // Sleep 100ms
                std::thread::sleep(Duration::from_millis(100));
            }
        });
    }
}
