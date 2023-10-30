use chrono::NaiveDateTime;
use log::*;
use resalt_models::{
    ApiError, AuthToken, Event, Filter, Job, Minion, MinionPreset, PermissionGroup, ResaltTime,
    SaltToken, User,
};
use serde_json::Value;

use crate::StorageStatus;

pub trait StorageImpl: Send {
    fn clone(&self) -> Box<dyn StorageImpl>;

    fn get_status(&self) -> Result<StorageStatus, String>;

    fn create_user(
        &self,
        username: String,
        password: Option<String>,
        email: Option<String>,
        ldap_sync: Option<String>,
    ) -> Result<User, String> {
        self.create_user_hashed(
            None,
            username,
            password.map(|v| resalt_security::hash_password(&v)),
            "[]".to_string(),
            None,
            email,
            ldap_sync,
        )
    }

    fn create_user_hashed(
        &self,
        id: Option<String>,
        username: String,
        password: Option<String>,
        perms: String,
        last_login: Option<ResaltTime>,
        email: Option<String>,
        ldap_sync: Option<String>,
    ) -> Result<User, String>;

    fn list_users(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, String>;

    fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String>;

    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String>;

    fn update_user(&self, user: &User) -> Result<(), String>;

    fn delete_user(&self, id: &str) -> Result<(), String>;

    #[allow(clippy::borrowed_box)]
    fn refresh_user_permissions(&self, user: &User) -> Result<(), ApiError> {
        let groups = match self.list_permission_groups_by_user_id(&user.id) {
            Ok(groups) => groups,
            Err(e) => {
                error!("{:?}", e);
                return Err(ApiError::DatabaseError);
            }
        };
        let mut perms: Vec<Value> = Vec::new();
        for group in groups {
            // Parse group.perms as json array
            let serdegroup: serde_json::Value = match serde_json::from_str(&group.perms) {
                Ok(serdegroup) => serdegroup,
                Err(e) => {
                    error!("{:?}", e);
                    return Err(ApiError::DatabaseError);
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
        let mut user: User = user.clone();
        user.perms = perms;
        match self.update_user(&user) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{:?}", e);
                Err(ApiError::DatabaseError)
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
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Minion>, String>;

    fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String>;

    #[allow(clippy::too_many_arguments)]
    fn update_minion(
        &self,
        minion_id: String,
        time: chrono::NaiveDateTime,
        grains: Option<String>,
        pillars: Option<String>,
        pkgs: Option<String>,
        conformity: Option<String>,
        conformity_success: Option<i32>,
        conformity_incorrect: Option<i32>,
        conformity_error: Option<i32>,
    ) -> Result<(), String>;

    fn update_minion_last_seen(
        &self,
        minion_id: String,
        time: chrono::NaiveDateTime,
    ) -> Result<(), String> {
        self.update_minion(minion_id, time, None, None, None, None, None, None, None)
    }

    fn update_minion_grains(
        &self,
        minion_id: String,
        time: chrono::NaiveDateTime,
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
        )
    }

    fn update_minion_pillars(
        &self,
        minion_id: String,
        time: chrono::NaiveDateTime,
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
        )
    }

    fn update_minion_pkgs(
        &self,
        minion_id: String,
        time: NaiveDateTime,
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
        )
    }

    fn update_minion_conformity(
        &self,
        minion_id: String,
        time: NaiveDateTime,
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
        )
    }

    fn prune_minions(&self, ids: Vec<String>) -> Result<(), String>;

    fn insert_event(
        &self,
        tag: String,
        data: String,
        timestamp: NaiveDateTime,
    ) -> Result<String, String>;

    fn list_events(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Event>, String>;

    fn insert_job(
        &self,
        jid: String,
        user: Option<String>,
        event_id: Option<String>,
        timestamp: NaiveDateTime,
    ) -> Result<(), String>;

    fn list_jobs(
        &self,
        sort: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Job>, String>;

    fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String>;

    fn insert_job_return(
        &self,
        jid: String,
        job_id: String,
        event_id: String,
        minion_id: String,
        timestamp: NaiveDateTime,
    ) -> Result<(), String>;

    fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<Event>, String>;

    fn create_permission_group(
        &self,
        id: Option<String>,
        name: &str,
        perms: Option<String>,
    ) -> Result<String, String>;

    fn list_permission_groups(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<PermissionGroup>, String>;

    fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String>;

    fn get_permission_group_by_name(&self, name: &str) -> Result<Option<PermissionGroup>, String>;

    fn get_permission_group_by_ldap_sync(
        &self,
        ldap_sync: &str,
    ) -> Result<Option<PermissionGroup>, String>;

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

    fn insert_minion_preset(&self, name: &str, filter: &str) -> Result<String, String>;

    fn list_minion_presets(&self) -> Result<Vec<MinionPreset>, String>;

    fn get_minion_preset_by_id(&self, id: &str) -> Result<Option<MinionPreset>, String>;

    fn update_minion_preset(&self, minion_preset: &MinionPreset) -> Result<(), String>;

    fn delete_minion_preset(&self, id: &str) -> Result<(), String>;
}
