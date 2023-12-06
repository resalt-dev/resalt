use log::*;
use resalt_models::{
    AuthToken, Event, Filter, Job, JobReturn, Minion, MinionPreset, Paginate, PermissionGroup,
    ResaltTime, SaltToken, StorageStatus, User,
};
use serde_json::Value;

pub trait StorageImpl: Send + Sync {
    fn clone(&self) -> Box<dyn StorageImpl>;

    fn clone_self(&self) -> Box<dyn StorageImpl> {
        self.clone()
    }

    fn get_status(&self) -> Result<StorageStatus, String>;

    fn create_user(
        &self,
        username: String,
        password: Option<String>,
        email: Option<String>,
    ) -> Result<User, String> {
        self.create_user_hashed(
            None,
            username,
            password.map(|v| resalt_security::hash_password(&v)),
            "[]".to_string(),
            None,
            email,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn create_user_hashed(
        &self,
        id: Option<String>,
        username: String,
        password: Option<String>,
        perms: String,
        last_login: Option<ResaltTime>,
        email: Option<String>,
    ) -> Result<User, String>;

    fn list_users(&self, paginate: Paginate) -> Result<Vec<User>, String>;

    fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String>;

    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String>;

    fn update_user(&self, user: &User) -> Result<(), String>;

    fn delete_user(&self, id: &str) -> Result<(), String>;

    fn refresh_user_permissions(&self, user_id: &str) -> Result<(), String> {
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

    fn create_authtoken(&self, user_id: String) -> Result<AuthToken, String>;

    fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String>;

    fn update_authtoken_salttoken(
        &self,
        auth_token: &str,
        salt_token: Option<&SaltToken>,
    ) -> Result<(), String>;

    fn list_minions(
        &self,
        filters: Vec<Filter>,
        sort: Option<String>,
        paginate: Paginate,
    ) -> Result<Vec<Minion>, String>;

    fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String>;

    #[allow(clippy::too_many_arguments)]
    fn update_minion(
        &self,
        minion_id: String,
        time: ResaltTime,
        grains: Option<String>,
        pillars: Option<String>,
        pkgs: Option<String>,
        conformity: Option<String>,
        conformity_success: Option<i32>,
        conformity_incorrect: Option<i32>,
        conformity_error: Option<i32>,
        last_updated_grains: Option<ResaltTime>,
        last_updated_pillars: Option<ResaltTime>,
        last_updated_pkgs: Option<ResaltTime>,
        last_updated_conformity: Option<ResaltTime>,
    ) -> Result<(), String>;

    fn update_minion_last_seen(&self, minion_id: String, time: ResaltTime) -> Result<(), String> {
        self.update_minion(
            minion_id, time, None, None, None, None, None, None, None, None, None, None, None,
        )
    }

    fn update_minion_grains(
        &self,
        minion_id: String,
        time: ResaltTime,
        grains: String,
    ) -> Result<(), String> {
        self.update_minion(
            minion_id,
            time,
            Some(grains),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    fn update_minion_pillars(
        &self,
        minion_id: String,
        time: ResaltTime,
        pillars: String,
    ) -> Result<(), String> {
        self.update_minion(
            minion_id,
            time,
            None,
            Some(pillars),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    fn update_minion_pkgs(
        &self,
        minion_id: String,
        time: ResaltTime,
        pkgs: String,
    ) -> Result<(), String> {
        self.update_minion(
            minion_id,
            time,
            None,
            None,
            Some(pkgs),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
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
        self.update_minion(
            minion_id,
            time,
            None,
            None,
            None,
            Some(conformity),
            Some(success),
            Some(incorrect),
            Some(error),
            None,
            None,
            None,
            None,
        )
    }

    fn delete_minion(&self, id: String) -> Result<(), String>;

    fn insert_event(
        &self,
        tag: String,
        data: String,
        timestamp: ResaltTime,
    ) -> Result<String, String>;

    fn list_events(&self, paginate: Paginate) -> Result<Vec<Event>, String>;

    fn get_event_by_id(&self, id: &str) -> Result<Option<Event>, String>;

    fn insert_job(
        &self,
        jid: String,
        user: Option<String>,
        event_id: Option<String>,
        timestamp: ResaltTime,
    ) -> Result<(), String>;

    fn list_jobs(&self, sort: Option<String>, paginate: Paginate) -> Result<Vec<Job>, String>;

    fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String>;

    fn insert_job_return(
        &self,
        jid: String,
        job_id: String,
        event_id: String,
        minion_id: String,
        timestamp: ResaltTime,
    ) -> Result<(), String>;

    fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<JobReturn>, String>;

    fn create_permission_group(
        &self,
        id: Option<String>,
        name: &str,
        perms: Option<String>,
    ) -> Result<String, String>;

    fn list_permission_groups(&self, paginate: Paginate) -> Result<Vec<PermissionGroup>, String>;

    fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String>;

    fn is_user_member_of_group(&self, user_id: &str, group_id: &str) -> Result<bool, String>;

    fn update_permission_group(&self, permission_group: &PermissionGroup) -> Result<(), String>;

    fn delete_permission_group(&self, id: &str) -> Result<(), String>;

    fn insert_permission_group_user(&self, user_id: &str, group_id: &str) -> Result<(), String>;

    fn list_permission_groups_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<PermissionGroup>, String>;

    fn list_users_by_permission_group_id(&self, group_id: &str) -> Result<Vec<User>, String>;

    fn delete_permission_group_user(&self, user_id: &str, group_id: &str) -> Result<(), String>;

    fn insert_minion_preset(
        &self,
        id: Option<String>,
        name: &str,
        filter: &str,
    ) -> Result<String, String>;

    fn list_minion_presets(&self) -> Result<Vec<MinionPreset>, String>;

    fn get_minion_preset_by_id(&self, id: &str) -> Result<Option<MinionPreset>, String>;

    fn update_minion_preset(&self, minion_preset: &MinionPreset) -> Result<(), String>;

    fn delete_minion_preset(&self, id: &str) -> Result<(), String>;
}

impl Clone for Box<dyn StorageImpl> {
    fn clone(&self) -> Box<dyn StorageImpl> {
        self.clone_self()
    }
}
