use crate::{
    AuthToken, Event, Filter, Job, JobReturn, JobSort, Minion, MinionPreset, MinionSort, Paginate,
    PermissionGroup, ResaltTime, SaltToken, User,
};

pub trait StorageImpl: Send + Sync {
    fn clone(&self) -> Box<dyn StorageImpl>;

    fn clone_self(&self) -> Box<dyn StorageImpl> {
        self.clone()
    }

    fn get_status(&self) -> Result<StorageStatus, String>;

    /// Create a new user.
    ///
    /// If `id` is `None`, a new UUID will be generated according to the format `usr_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`.
    /// If `password` is `None`, the account will be created without a password. THE PASSWORD MUST BE HASHED BEFORE CALLING THIS FUNCTION.
    /// If `last_login` is `None`, the account will be created without a last login time.
    /// If `email` is `None`, the account will be created without an email address.
    ///
    /// Username MUST be unique.
    /// Perms SHOULD be a valid JSON string, however this is not enforced by the database layer.
    /// Email SHOULD be a valid email address, however this is not enforced by the database layer.
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
        sort: Option<MinionSort>,
        paginate: Paginate,
    ) -> Result<Vec<Minion>, String>;

    fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String>;

    fn upsert_minion(&self, minion: Minion) -> Result<(), String>;

    fn upsert_minion_last_seen(&self, minion_id: String, time: ResaltTime) -> Result<(), String>;

    fn upsert_minion_grains(
        &self,
        minion_id: String,
        time: ResaltTime,
        grains: String,
        os_type: String,
    ) -> Result<(), String>;

    fn upsert_minion_pillars(
        &self,
        minion_id: String,
        time: ResaltTime,
        pillars: String,
    ) -> Result<(), String>;

    fn upsert_minion_pkgs(
        &self,
        minion_id: String,
        time: ResaltTime,
        pkgs: String,
    ) -> Result<(), String>;

    fn upsert_minion_conformity(
        &self,
        minion_id: String,
        time: ResaltTime,
        conformity: String,
        success: i32,
        incorrect: i32,
        error: i32,
    ) -> Result<(), String>;

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

    fn list_jobs(&self, sort: Option<JobSort>, paginate: Paginate) -> Result<Vec<Job>, String>;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorageStatus {
    pub auth_tokens_total: i64,
    pub auth_tokens_active: i64,
    pub events_total: i64,
    pub job_returns_total: i64,
    pub jobs_total: i64,
    pub minions_total: i64,
    pub minions_success: i64,
    pub minions_incorrect: i64,
    pub minions_error: i64,
    pub minions_unknown: i64,
    pub permission_group_users_total: i64,
    pub permission_groups_total: i64,
    pub users_total: i64,
}

//
// Test cases
//

pub fn test_storage_impl_users(data: &dyn StorageImpl) {
    // Check DB is empty
    let total = data.list_users(Paginate::None).unwrap();
    assert_eq!(total.len(), 0);

    // Create user
    let user = data
        .create_user_hashed(
            None,
            "testuser".to_string(),
            Some("testpass".to_string()),
            "testperms".to_string(),
            None,
            None,
        )
        .unwrap();
    assert_eq!(user.id.len(), 40);
    assert_eq!(user.username, "testuser");
    assert_eq!(user.password, Some("testpass".to_string()));
    assert_eq!(user.perms, "testperms");
    assert_eq!(user.last_login, None);
    assert_eq!(user.email, None);

    // Check DB has one user
    let total = data.list_users(Paginate::None).unwrap();
    assert_eq!(total.len(), 1);

    // Get user by id
    let user = data.get_user_by_id(&user.id).unwrap().unwrap();
    assert_eq!(user.id.len(), 40);
    assert_eq!(user.username, "testuser");
    assert_eq!(user.password, Some("testpass".to_string()));
    assert_eq!(user.perms, "testperms");
    assert_eq!(user.last_login, None);
    assert_eq!(user.email, None);

    // Get user by username
    let user = data.get_user_by_username(&user.username).unwrap().unwrap();
    assert_eq!(user.id.len(), 40);
    assert_eq!(user.username, "testuser");
    assert_eq!(user.password, Some("testpass".to_string()));
    assert_eq!(user.perms, "testperms");

    // Update user
    let mut user = data.get_user_by_id(&user.id).unwrap().unwrap();
    user.username = "testuser2".to_string();

    data.update_user(&user).unwrap();

    let user = data.get_user_by_id(&user.id).unwrap().unwrap();
    assert_eq!(user.id.len(), 40);
    assert_eq!(user.username, "testuser2");
    assert_eq!(user.password, Some("testpass".to_string()));
    assert_eq!(user.perms, "testperms");

    // Delete user
    data.delete_user(&user.id).unwrap();

    // Check DB is empty
    let total = data.list_users(Paginate::None).unwrap();
    assert_eq!(total.len(), 0);
}
