use log::{debug, error};
use resalt_config::ResaltConfig;
use resalt_models::*;
use resalt_storage_files::StorageFiles;
use resalt_storage_mysql::StorageMySQL;
use resalt_storage_redis::StorageRedis;
use serde_json::Value;

#[derive(Clone)]
pub struct Storage {
    storage: Box<dyn StorageImpl>,
}

impl Storage {
    pub async fn init_db() -> Storage {
        let db_type = &ResaltConfig::DATABASE_TYPE.clone();
        let db_type = db_type.as_str();
        debug!("Database type: \"{}\"", db_type);
        let storage: Box<dyn StorageImpl> = match db_type {
            "files" => {
                let path: String = ResaltConfig::DATABASE_HOST.clone();
                debug!("Database path: \"{}\"", path);
                Box::new(
                    StorageFiles::connect(&path)
                        .unwrap_or_else(|_| panic!("Error connecting to {}", &path)),
                )
            }
            "mysql" => {
                let database_url = format!(
                    "mysql://{}:{}@{}:{}/{}",
                    *ResaltConfig::DATABASE_USERNAME,
                    *ResaltConfig::DATABASE_PASSWORD,
                    *ResaltConfig::DATABASE_HOST,
                    *ResaltConfig::DATABASE_PORT,
                    *ResaltConfig::DATABASE_DATABASE
                );
                debug!("Database URL: \"{}\"", database_url);
                Box::new(
                    StorageMySQL::connect(&database_url)
                        .await
                        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url)),
                )
            }
            "redis" => {
                let database_url = format!(
                    "redis://{}:{}@{}:{}/{}",
                    *ResaltConfig::DATABASE_USERNAME,
                    *ResaltConfig::DATABASE_PASSWORD,
                    *ResaltConfig::DATABASE_HOST,
                    *ResaltConfig::DATABASE_PORT,
                    *ResaltConfig::DATABASE_DATABASE
                );
                debug!("Database URL: \"{}\"", database_url);
                Box::new(
                    StorageRedis::connect(&database_url)
                        .await
                        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url)),
                )
            }
            _ => panic!(),
        };
        Storage { storage }
    }

    pub fn prune_minions_without_key(&self, keys: &Vec<SaltMinionKey>) -> Result<(), String> {
        let mut minions: Vec<Minion> = match self.list_minions(Vec::new(), None, Paginate::None) {
            Ok(minions) => minions,
            Err(e) => {
                error!("{:?}", e);
                return Err(e);
            }
        };
        let mut minions_to_delete: Vec<Minion> = Vec::new();
        for minion in minions.iter_mut() {
            let mut found = false;
            for key in keys {
                if minion.id == key.id {
                    found = true;
                    break;
                }
            }
            if !found {
                minions_to_delete.push(minion.clone());
            }
        }
        for minion in minions_to_delete {
            match self.delete_minion(minion.id) {
                Ok(_) => (),
                Err(e) => {
                    error!("{:?}", e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub fn refresh_user_permissions(&self, user_id: &str) -> Result<(), String> {
        let groups = match self.list_permission_groups_by_user_id(user_id) {
            Ok(groups) => groups,
            Err(e) => {
                error!("{:?}", e);
                return Err(e);
            }
        };
        let mut perms: Vec<Value> = Vec::new();
        for group in groups {
            // Parse group.perms as json array
            let serdegroup: serde_json::Value = match serde_json::from_str(&group.perms) {
                Ok(serdegroup) => serdegroup,
                Err(e) => {
                    error!("{:?}", e);
                    return Err(e.to_string());
                }
            };
            let group_perms = match serdegroup.as_array() {
                Some(group_perms) => group_perms,
                None => continue,
            };
            for group_perm in group_perms {
                perms.push(group_perm.clone());
            }
        }
        let perms = Value::Array(perms);
        let perms = serde_json::to_string(&perms).unwrap();
        let mut user: User = match self.get_user_by_id(user_id) {
            Ok(Some(user)) => user,
            Ok(None) => {
                error!("User not found");
                return Err("User not found".to_string());
            }
            Err(e) => {
                error!("{:?}", e);
                return Err(e);
            }
        };
        user.perms = perms;
        match self.update_user(&user) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{:?}", e);
                Err(e)
            }
        }
    }
}

impl StorageImpl for Storage {
    fn clone(&self) -> Box<dyn StorageImpl> {
        Box::new(Clone::clone(self))
    }

    fn get_status(&self) -> Result<StorageStatus, String> {
        self.storage.get_status()
    }

    fn create_user_hashed(
        &self,
        id: Option<String>,
        username: String,
        password: Option<String>,
        perms: String,
        last_login: Option<ResaltTime>,
        email: Option<String>,
    ) -> Result<User, String> {
        self.storage
            .create_user_hashed(id, username, password, perms, last_login, email)
    }

    fn list_users(&self, paginate: Paginate) -> Result<Vec<User>, String> {
        self.storage.list_users(paginate)
    }

    fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        self.storage.get_user_by_id(id)
    }

    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        self.storage.get_user_by_username(username)
    }

    fn update_user(&self, user: &User) -> Result<(), String> {
        self.storage.update_user(user)
    }

    fn delete_user(&self, id: &str) -> Result<(), String> {
        self.storage.delete_user(id)
    }

    fn create_authtoken(&self, user_id: String) -> Result<AuthToken, String> {
        self.storage.create_authtoken(user_id)
    }

    fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        self.storage.get_authtoken_by_id(id)
    }

    fn update_authtoken_salttoken(
        &self,
        auth_token: &str,
        salt_token: Option<&SaltToken>,
    ) -> Result<(), String> {
        self.storage
            .update_authtoken_salttoken(auth_token, salt_token)
    }

    fn list_minions(
        &self,
        filters: Vec<Filter>,
        sort: Option<MinionSort>,
        paginate: Paginate,
    ) -> Result<Vec<Minion>, String> {
        self.storage.list_minions(filters, sort, paginate)
    }

    fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String> {
        self.storage.get_minion_by_id(id)
    }

    fn update_minion(&self, minion: Minion) -> Result<(), String> {
        self.storage.update_minion(minion)
    }

    fn update_minion_last_seen(&self, minion_id: String, time: ResaltTime) -> Result<(), String> {
        self.storage.update_minion_last_seen(minion_id, time)
    }

    fn update_minion_grains(
        &self,
        minion_id: String,
        time: ResaltTime,
        grains: String,
        os_type: String,
    ) -> Result<(), String> {
        self.storage
            .update_minion_grains(minion_id, time, grains, os_type)
    }

    fn update_minion_pillars(
        &self,
        minion_id: String,
        time: ResaltTime,
        pillars: String,
    ) -> Result<(), String> {
        self.storage.update_minion_pillars(minion_id, time, pillars)
    }

    fn update_minion_pkgs(
        &self,
        minion_id: String,
        time: ResaltTime,
        pkgs: String,
    ) -> Result<(), String> {
        self.storage.update_minion_pkgs(minion_id, time, pkgs)
    }

    fn update_minion_conformity(
        &self,
        minion_id: String,
        time: ResaltTime,
        conformity: String,
        success: i32,
        incorrect: i32,
        error: i32,
    ) -> Result<(), String> {
        self.storage
            .update_minion_conformity(minion_id, time, conformity, success, incorrect, error)
    }

    fn delete_minion(&self, id: String) -> Result<(), String> {
        self.storage.delete_minion(id)
    }

    fn insert_event(
        &self,
        tag: String,
        data: String,
        timestamp: ResaltTime,
    ) -> Result<String, String> {
        self.storage.insert_event(tag, data, timestamp)
    }

    fn list_events(&self, paginate: Paginate) -> Result<Vec<Event>, String> {
        self.storage.list_events(paginate)
    }

    fn get_event_by_id(&self, id: &str) -> Result<Option<Event>, String> {
        self.storage.get_event_by_id(id)
    }

    fn insert_job(
        &self,
        jid: String,
        user: Option<String>,
        event_id: Option<String>,
        timestamp: ResaltTime,
    ) -> Result<(), String> {
        self.storage.insert_job(jid, user, event_id, timestamp)
    }

    fn list_jobs(&self, sort: Option<JobSort>, paginate: Paginate) -> Result<Vec<Job>, String> {
        self.storage.list_jobs(sort, paginate)
    }

    fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String> {
        self.storage.get_job_by_jid(jid)
    }

    fn insert_job_return(
        &self,
        jid: String,
        job_id: String,
        event_id: String,
        minion_id: String,
        timestamp: ResaltTime,
    ) -> Result<(), String> {
        self.storage
            .insert_job_return(jid, job_id, event_id, minion_id, timestamp)
    }

    fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<JobReturn>, String> {
        self.storage.get_job_returns_by_job(job)
    }

    fn create_permission_group(
        &self,
        id: Option<String>,
        name: &str,
        perms: Option<String>,
    ) -> Result<String, String> {
        self.storage.create_permission_group(id, name, perms)
    }

    fn list_permission_groups(&self, paginate: Paginate) -> Result<Vec<PermissionGroup>, String> {
        self.storage.list_permission_groups(paginate)
    }

    fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String> {
        self.storage.get_permission_group_by_id(id)
    }

    fn is_user_member_of_group(&self, user_id: &str, group_id: &str) -> Result<bool, String> {
        self.storage.is_user_member_of_group(user_id, group_id)
    }

    fn update_permission_group(&self, permission_group: &PermissionGroup) -> Result<(), String> {
        self.storage.update_permission_group(permission_group)
    }

    fn delete_permission_group(&self, id: &str) -> Result<(), String> {
        self.storage.delete_permission_group(id)
    }

    fn insert_permission_group_user(&self, user_id: &str, group_id: &str) -> Result<(), String> {
        self.storage.insert_permission_group_user(user_id, group_id)
    }

    fn list_permission_groups_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<PermissionGroup>, String> {
        self.storage.list_permission_groups_by_user_id(user_id)
    }

    fn list_users_by_permission_group_id(&self, group_id: &str) -> Result<Vec<User>, String> {
        self.storage.list_users_by_permission_group_id(group_id)
    }

    fn delete_permission_group_user(&self, user_id: &str, group_id: &str) -> Result<(), String> {
        self.storage.delete_permission_group_user(user_id, group_id)
    }

    fn insert_minion_preset(
        &self,
        id: Option<String>,
        name: &str,
        filter: &str,
    ) -> Result<String, String> {
        self.storage.insert_minion_preset(id, name, filter)
    }

    fn list_minion_presets(&self) -> Result<Vec<MinionPreset>, String> {
        self.storage.list_minion_presets()
    }

    fn get_minion_preset_by_id(&self, id: &str) -> Result<Option<MinionPreset>, String> {
        self.storage.get_minion_preset_by_id(id)
    }

    fn update_minion_preset(&self, minion_preset: &MinionPreset) -> Result<(), String> {
        self.storage.update_minion_preset(minion_preset)
    }

    fn delete_minion_preset(&self, id: &str) -> Result<(), String> {
        self.storage.delete_minion_preset(id)
    }
}
