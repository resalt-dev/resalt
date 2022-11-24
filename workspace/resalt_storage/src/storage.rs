use chrono::*;
use resalt_models::*;

pub trait Storage {
    fn create_user(
        &self,
        username: String,
        password: Option<String>,
        email: Option<String>,
        ldap_sync: Option<String>,
    );
    fn list_users(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, String>;
    fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String>;
    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String>;
    fn update_user(&self, user: &User) -> Result<(), String>;
    fn create_authtoken(&self, user_id: String) -> Result<AuthToken, String>;
    fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String>;
    fn update_authtoken_salttoken(&self, auth_token: &str, salt_token: &Option<SaltToken>);
    fn list_minions(
        &self,
        filters: Vec<Filter>,
        sort: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    );
    fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String>;
    fn update_minion_last_seen(&self, minion_id: String, time: chrono::NaiveDateTime);
    fn update_minion_grains(&self, minion_id: String, time: chrono::NaiveDateTime, grains: String);
    fn update_minion_pillars(
        &self,
        minion_id: String,
        time: chrono::NaiveDateTime,
        pillars: String,
    );
    fn update_minion_pkgs(&self, minion_id: String, time: NaiveDateTime, pkgs: String);
    fn update_minion_conformity(
        &self,
        minion_id: String,
        time: NaiveDateTime,
        conformity: String,
        success: i32,
        incorrect: i32,
        error: i32,
    );
    fn prune_minions(&self, ids: Vec<String>) -> Result<(), String>;
    fn insert_event(&self, tag: String, data: String, timestamp: NaiveDateTime);
    fn list_events(&self, limit: Option<i64>, offset: Option<i64>);
    fn insert_job(
        &self,
        jid: String,
        user: Option<String>,
        event_id: Option<String>,
        timestamp: NaiveDateTime,
    );
    fn list_jobs(&self, sort: Option<String>, limit: Option<i64>, offset: Option<i64>);
    fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String>;
    fn insert_job_return(
        &self,
        jid: String,
        job_id: String,
        event_id: String,
        minion_id: String,
        timestamp: NaiveDateTime,
    );
    fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<Event>, String>;
    fn get_metric_results(&self) -> Result<Vec<MetricResult>, String>;
    fn create_permission_group(&self, name: &str) -> Result<String, String>;
    fn list_permission_groups(&self, limit: Option<i64>, offset: Option<i64>);
    fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String>;
    fn get_permission_group_by_name(&self, name: &str);
    fn is_user_member_of_group(&self, user_id: &str, group_id: &str) -> Result<bool, String>;
    fn update_permission_group(&self, permission_group: &PermissionGroup);
    fn delete_permission_group(&self, id: &str) -> Result<(), String>;
    fn insert_permission_group_user(&self, user_id: &str, group_id: &str);
    fn list_permission_groups_by_user_id(&self, user_id: &str);
    fn list_users_by_permission_group_id(&self, group_id: &str) -> Result<Vec<User>, String>;
    fn delete_permission_group_user(&self, user_id: &str, group_id: &str);
}
