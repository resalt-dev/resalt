use crate::{
    AuthToken, Event, Filter, Job, JobReturn, JobSort, Minion, MinionPreset, MinionSort, Paginate,
    PermissionGroup, Preferences, ResaltTime, SaltToken, User,
};

pub trait StorageImpl: Send + Sync {
    fn clone(&self) -> Box<dyn StorageImpl>;

    fn clone_self(&self) -> Box<dyn StorageImpl> {
        self.clone()
    }

    // Extremely generic KV operations

    // fn get(&self, key: &str) -> Result<Option<String>, String>;

    // fn set(&self, key: &str, value: &str) -> Result<(), String>;

    // fn delete(&self, key: &str) -> Result<(), String>;

    // fn list(&self, prefix: &str) -> Result<Vec<String>, String>;

    // fn list_prefix(&self, prefix: &str) -> Result<Vec<String>, String>;

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

    fn upsert_preferences(&self, user_id: &str, preferences: &Preferences) -> Result<(), String>;

    fn get_preferences(&self, user_id: &str) -> Result<Option<Preferences>, String>;

    /// Create a new auth token.
    ///
    /// If user does not exist, this function will return an error.
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

    fn upsert_minion_last_seen(&self, minion_id: &str, time: ResaltTime) -> Result<(), String>;

    fn upsert_minion_grains(
        &self,
        minion_id: &str,
        time: ResaltTime,
        grains: String,
        os_type: String,
    ) -> Result<(), String>;

    fn upsert_minion_pillars(
        &self,
        minion_id: &str,
        time: ResaltTime,
        pillars: String,
    ) -> Result<(), String>;

    fn upsert_minion_pkgs(
        &self,
        minion_id: &str,
        time: ResaltTime,
        pkgs: String,
    ) -> Result<(), String>;

    fn upsert_minion_conformity(
        &self,
        minion_id: &str,
        time: ResaltTime,
        conformity: String,
        success: i32,
        incorrect: i32,
        error: i32,
    ) -> Result<(), String>;

    fn delete_minion(&self, id: &str) -> Result<(), String>;

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
    assert!(user.id.starts_with("usr_"));
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

pub fn test_storage_impl_authtoken(data: &dyn StorageImpl) {
    // Create User
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
    data.update_user(&user).unwrap();

    // Create Authtoken
    let authtoken = data.create_authtoken(user.id.clone()).unwrap();
    assert!(authtoken.id.starts_with("auth_"));
    assert_eq!(authtoken.id.len(), 41);
    assert_eq!(authtoken.user_id, user.id);
    assert_eq!(authtoken.salt_token, None);
    assert!(authtoken.timestamp.timestamp() > 100000);

    // Get Authtoken by id
    let authtoken = data.get_authtoken_by_id(&authtoken.id).unwrap().unwrap();
    assert_eq!(authtoken.id.len(), 41);
    assert_eq!(authtoken.user_id, user.id);
    assert_eq!(authtoken.salt_token, None);

    // Update Authtoken
    let salttoken = SaltToken {
        token: "testtoken".to_string(),
        start: 0.0,
        expire: 99.9,
        user: "testuser".to_string(),
        eauth: "unittest".to_string(),
        perms: serde_json::Value::Array(vec![]),
    };
    data.update_authtoken_salttoken(&authtoken.id, Some(&salttoken))
        .unwrap();

    let authtoken = data.get_authtoken_by_id(&authtoken.id).unwrap().unwrap();
    assert_eq!(authtoken.id.len(), 41);
    assert_eq!(authtoken.user_id, user.id);
    assert_eq!(authtoken.salt_token.unwrap(), salttoken);
}

pub fn test_storage_impl_minions(data: &dyn StorageImpl) {
    // Check DB is empty
    let total = data.list_minions(vec![], None, Paginate::None).unwrap();
    assert_eq!(total.len(), 0);

    // Create minion
    let time1 = ResaltTime::now();
    let minion = Minion {
        id: "testminion".to_string(),
        last_seen: time1,
        grains: None,
        pillars: None,
        pkgs: None,
        last_updated_grains: None,
        last_updated_pillars: None,
        last_updated_pkgs: None,
        conformity: None,
        conformity_success: None,
        conformity_incorrect: None,
        conformity_error: None,
        last_updated_conformity: None,
        os_type: None,
    };
    data.upsert_minion(minion.clone()).unwrap();

    // Check DB has one minion
    let total = data.list_minions(vec![], None, Paginate::None).unwrap();
    assert_eq!(total.len(), 1);
    assert_eq!(total[0], minion);

    // Get minion by id
    let minion = data.get_minion_by_id(&minion.id).unwrap().unwrap();
    assert_eq!(minion.id, "testminion");
    assert_eq!(minion.last_seen, time1);

    // Update minion last seen
    let time2 = time1 + chrono::Duration::seconds(1);
    assert!(time2.timestamp() > time1.timestamp());
    data.upsert_minion_last_seen("testminion", time2).unwrap();

    let minion = data.get_minion_by_id(&minion.id).unwrap().unwrap();
    assert_eq!(minion.id, "testminion");
    assert_eq!(minion.last_seen, time2);

    // Update minion grains
    data.upsert_minion_grains(
        "testminion",
        time2,
        "testgrains".to_string(),
        "testos".to_string(),
    )
    .unwrap();

    let minion = data.get_minion_by_id(&minion.id).unwrap().unwrap();
    assert_eq!(minion.id, "testminion");
    assert_eq!(minion.last_updated_grains.unwrap(), time2);
    assert_eq!(minion.grains.unwrap(), "testgrains");

    // Update minion pillars
    data.upsert_minion_pillars("testminion", time2, "testpillars".to_string())
        .unwrap();

    let minion = data.get_minion_by_id(&minion.id).unwrap().unwrap();
    assert_eq!(minion.id, "testminion");
    assert_eq!(minion.last_updated_pillars.unwrap(), time2);
    assert_eq!(minion.pillars.unwrap(), "testpillars");

    // Update minion pkgs
    data.upsert_minion_pkgs("testminion", time2, "testpkgs".to_string())
        .unwrap();

    let minion = data.get_minion_by_id(&minion.id).unwrap().unwrap();
    assert_eq!(minion.id, "testminion");
    assert_eq!(minion.last_updated_pkgs.unwrap(), time2);
    assert_eq!(minion.pkgs.unwrap(), "testpkgs");

    // Update minion conformity
    data.upsert_minion_conformity("testminion", time2, "testconformity".to_string(), 1, 2, 3)
        .unwrap();

    let minion = data.get_minion_by_id(&minion.id).unwrap().unwrap();
    assert_eq!(minion.id, "testminion");
    assert_eq!(minion.last_updated_conformity.unwrap(), time2);
    assert_eq!(minion.conformity.unwrap(), "testconformity");
    assert_eq!(minion.conformity_success.unwrap(), 1);
    assert_eq!(minion.conformity_incorrect.unwrap(), 2);
    assert_eq!(minion.conformity_error.unwrap(), 3);

    // Check grains, pillars and pkgs are still there
    assert_eq!(minion.last_updated_grains.unwrap(), time2);
    assert_eq!(minion.grains.unwrap(), "testgrains");
    assert_eq!(minion.last_updated_pillars.unwrap(), time2);
    assert_eq!(minion.pillars.unwrap(), "testpillars");
    assert_eq!(minion.last_updated_pkgs.unwrap(), time2);
    assert_eq!(minion.pkgs.unwrap(), "testpkgs");

    // Delete minion
    data.delete_minion(&minion.id).unwrap();

    // Check DB is empty
    let total = data.list_minions(vec![], None, Paginate::None).unwrap();
    assert_eq!(total.len(), 0);
}
