use std::collections::HashMap;

use log::{debug, error};
use resalt_config::ResaltConfig;
use resalt_models::*;
use resalt_storage_files::StorageFiles;
use resalt_storage_redis::StorageRedis;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub struct Storage {
    s: Box<dyn StorageImpl>,
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
        Storage { s: storage }
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
            match self.delete_minion(&minion.id) {
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
        let perms = match serde_json::to_string(&perms) {
            Ok(perms) => perms,
            Err(e) => {
                error!("{:?}", e);
                return Err(e.to_string());
            }
        };
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
        match self.set_user(&user) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{:?}", e);
                Err(e)
            }
        }
    }

    fn obj_to_map(&self, obj: &impl Serialize) -> Result<HashMap<String, String>, String> {
        let obj = match serde_json::to_value(obj) {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        };
        let obj = match obj.as_object() {
            Some(obj) => obj,
            None => return Err("Object is not a map".to_string()),
        };
        let mut map: HashMap<String, String> = HashMap::new();
        for (key, value) in obj.iter() {
            map.insert(key.clone(), value.to_string());
        }
        Ok(map)
    }

    fn save_object(&self, prefix: &str, obj: &impl Serialize) -> Result<(), String> {
        let map = match self.obj_to_map(obj) {
            Ok(map) => map,
            Err(e) => return Err(e),
        };
        for (key, value) in map.iter() {
            match self.s.set(&format!("{}:{}", prefix, key), value) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn read_map(&self, prefix: &str) -> Result<Option<HashMap<String, String>>, String> {
        let keys = match self.s.keys(&format!("{}:*", prefix)) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };
        if keys.is_empty() {
            return Ok(None);
        }
        let mut map: HashMap<String, String> = HashMap::new();
        for key in keys {
            let value = match self.s.get(&key) {
                Ok(value) => value,
                Err(e) => return Err(e),
            };
            let value = match value {
                Some(value) => value,
                None => continue,
            };
            let key = key.replace(&format!("{}:", prefix), "");
            map.insert(key, value);
        }
        Ok(Some(map))
    }

    fn read_object<T: serde::de::DeserializeOwned>(
        &self,
        prefix: &str,
    ) -> Result<Option<T>, String> {
        let map = match self.read_map(prefix) {
            Ok(map) => map,
            Err(e) => return Err(e),
        };
        let map = match map {
            Some(map) => map,
            None => return Ok(None),
        };
        let obj = match serde_json::to_value(map) {
            Ok(obj) => obj,
            Err(e) => return Err(e.to_string()),
        };
        let obj = match serde_json::from_value(obj) {
            Ok(obj) => obj,
            Err(e) => return Err(e.to_string()),
        };
        Ok(Some(obj))
    }

    fn id(prefix: &str) -> String {
        format!("{}_{}", prefix, uuid::Uuid::new_v4())
    }

    fn keys_depth(&self, prefix: &str, depth: usize) -> Result<Vec<String>, String> {
        let keys = match self.s.keys(&format!("{}:*", prefix)) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };
        let mut result: Vec<String> = Vec::new();
        for key in keys {
            let key = key.replace(&format!("{}:", prefix), "");
            if key.split(':').count() == depth {
                result.push(key);
            }
        }
        Ok(result)
    }

    //
    // User Preferences
    //

    pub fn set_preferences(&self, user_id: &str, preferences: &Preferences) -> Result<(), String> {
        self.save_object(&format!("user:{}:preferences", user_id), preferences)
    }

    pub fn get_preferences(&self, user_id: &str) -> Result<Option<Preferences>, String> {
        self.read_object(&format!("user:{}:preferences", user_id))
    }

    //
    // Events
    //

    pub fn insert_event(
        &self,
        tag: String,
        data: String,
        timestamp: ResaltTime,
    ) -> Result<String, String> {
        let event = Event {
            id: Storage::id("evnt"),
            tag,
            data,
            timestamp,
        };
        self.save_object(&format!("event:{}", event.id), &event)?;
        Ok(event.id)
    }

    pub fn get_event(&self, id: &str) -> Result<Option<Event>, String> {
        self.read_object(&format!("event:{}", id))
    }

    pub fn list_events(&self, paginate: Paginate) -> Result<Vec<Event>, String> {
        let keys = match self.keys_depth("event:*", 2) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };

        // Read events
        let mut events: Vec<Event> = Vec::new();
        for key in keys {
            let event = match self.read_object(&key) {
                Ok(Some(event)) => event,
                Ok(None) => continue,
                Err(e) => return Err(e),
            };
            events.push(event);
        }

        // Sort by timestamp
        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Pagination
        if let Some((limit, offset)) = paginate {
            events = events
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect();
        }

        Ok(events)
    }

    //
    // Jobs
    //

    pub fn insert_job(
        &self,
        jid: String,
        user: Option<String>,
        event_id: Option<String>,
        timestamp: ResaltTime,
    ) -> Result<(), String> {
        let job = Job {
            id: jid.clone(),
            timestamp,
            jid: jid.clone(), // TODO: remove, id = jid
            user,
            event_id,
        };
        self.save_object(&format!("job:{}", job.id), &job)
    }

    pub fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String> {
        self.read_object(&format!("job:{}", jid))
    }

    pub fn list_jobs(&self, sort: Option<JobSort>, paginate: Paginate) -> Result<Vec<Job>, String> {
        let keys = match self.keys_depth("job:*", 2) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };

        // Read jobs
        let mut jobs: Vec<Job> = Vec::new();
        for key in keys {
            let job = match self.read_object(&key) {
                Ok(Some(job)) => job,
                Ok(None) => continue,
                Err(e) => return Err(e),
            };
            jobs.push(job);
        }

        // Sort
        if let Some(sort) = sort {
            sort_jobs(&mut jobs, &sort);
        }

        // Pagination
        if let Some((limit, offset)) = paginate {
            jobs = jobs
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect();
        }

        Ok(jobs)
    }

    //
    // Job Returns
    //

    pub fn insert_job_return(
        &self,
        jid: String,
        job_id: String,
        event_id: String,
        minion_id: String,
        timestamp: ResaltTime,
    ) -> Result<(), String> {
        let job_return = JobReturn {
            id: Storage::id("jret"),
            timestamp,
            jid,
            job_id,
            event_id,
            minion_id,
        };
        self.save_object(&format!("job_return:{}", job_return.id), &job_return)
    }

    pub fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<JobReturn>, String> {
        let keys = match self.keys_depth("job_return:*", 2) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };

        // Read job returns
        let mut job_returns: Vec<JobReturn> = Vec::new();
        for key in keys {
            let job_return = match self.read_object(&key) {
                Ok(Some(job_return)) => job_return,
                Ok(None) => continue,
                Err(e) => return Err(e),
            };
            job_returns.push(job_return);
        }

        // Filter by job_id
        let job_returns: Vec<JobReturn> = job_returns
            .into_iter()
            .filter(|job_return| job_return.job_id == job.id)
            .collect();

        Ok(job_returns)
    }

    //
    // Permission Groups
    //

    pub fn create_permission_group(
        &self,
        id: Option<String>,
        name: &str,
        perms: Option<String>,
    ) -> Result<String, String> {
        let permission_group = PermissionGroup {
            id: id.unwrap_or_else(|| Storage::id("perm")),
            name: name.to_string(),
            perms: perms.unwrap_or_else(|| "[]".to_string()),
        };
        self.save_object(
            &format!("permission_group:{}", permission_group.id),
            &permission_group,
        )?;
        Ok(permission_group.id)
    }

    pub fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String> {
        self.read_object(&format!("permission_group:{}", id))
    }

    pub fn update_permission_group(
        &self,
        permission_group: &PermissionGroup,
    ) -> Result<(), String> {
        self.save_object(
            &format!("permission_group:{}", permission_group.id),
            permission_group,
        )
    }

    pub fn delete_permission_group(&self, id: &str) -> Result<(), String> {
        match self.s.del(&format!("permission_group:{}", id)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn list_permission_groups(
        &self,
        paginate: Paginate,
    ) -> Result<Vec<PermissionGroup>, String> {
        let keys = match self.keys_depth("permission_group:*", 2) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };

        // Read permission groups
        let mut permission_groups: Vec<PermissionGroup> = Vec::new();
        for key in keys {
            let permission_group = match self.read_object(&key) {
                Ok(Some(permission_group)) => permission_group,
                Ok(None) => continue,
                Err(e) => return Err(e),
            };
            permission_groups.push(permission_group);
        }

        // Pagination
        if let Some((limit, offset)) = paginate {
            permission_groups = permission_groups
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect();
        }

        Ok(permission_groups)
    }

    pub fn list_permission_groups_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<PermissionGroup>, String> {
        let keys = match self.keys_depth("permission_group_user:*", 2) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };

        // Read permission groups
        let mut permission_groups: Vec<PermissionGroup> = Vec::new();
        for key in keys {
            let user_id_group_id: Vec<&str> = key.split(':').collect();
            if user_id_group_id.len() != 3 {
                continue;
            }
            if user_id_group_id[1] != user_id {
                continue;
            }
            let permission_group = match self.get_permission_group_by_id(user_id_group_id[2]) {
                Ok(Some(permission_group)) => permission_group,
                Ok(None) => continue,
                Err(e) => return Err(e),
            };
            permission_groups.push(permission_group);
        }

        Ok(permission_groups)
    }

    pub fn is_user_member_of_group(&self, user_id: &str, group_id: &str) -> Result<bool, String> {
        let key = format!("permission_group_user:{}:{}", user_id, group_id);
        match self.s.get(&key) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub fn insert_permission_group_user(
        &self,
        user_id: &str,
        group_id: &str,
    ) -> Result<(), String> {
        let key = format!("permission_group_user:{}:{}", user_id, group_id);
        self.s.set(&key, "")
    }

    pub fn list_users_by_permission_group_id(&self, group_id: &str) -> Result<Vec<User>, String> {
        let keys = match self.keys_depth("permission_group_user:*", 2) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };

        // Read users
        let mut users: Vec<User> = Vec::new();
        for key in keys {
            let user_id_group_id: Vec<&str> = key.split(':').collect();
            if user_id_group_id.len() != 3 {
                continue;
            }
            if user_id_group_id[2] != group_id {
                continue;
            }
            let user = match self.get_user_by_id(user_id_group_id[1]) {
                Ok(Some(user)) => user,
                Ok(None) => continue,
                Err(e) => return Err(e),
            };
            users.push(user);
        }

        Ok(users)
    }

    pub fn delete_permission_group_user(
        &self,
        user_id: &str,
        group_id: &str,
    ) -> Result<(), String> {
        let key = format!("permission_group_user:{}:{}", user_id, group_id);
        match self.s.del(&key) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    //
    // Minions
    //

    pub fn list_minions(
        &self,
        filters: Vec<Filter>,
        sort: Option<MinionSort>,
        paginate: Paginate,
    ) -> Result<Vec<Minion>, String> {
        let keys = match self.keys_depth("minion:*", 2) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };

        // Read minions
        let mut minions: Vec<Minion> = Vec::new();
        for key in keys {
            let minion = match self.read_object(&key) {
                Ok(Some(minion)) => minion,
                Ok(None) => continue,
                Err(e) => return Err(e),
            };
            minions.push(minion);
        }

        // Filter
        filter_minions(&mut minions, &filters);

        // Sort
        if let Some(sort) = sort {
            sort_minions(&mut minions, &sort);
        }

        // Pagination
        if let Some((limit, offset)) = paginate {
            minions = minions
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect();
        }

        Ok(minions)
    }

    pub fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String> {
        self.read_object(&format!("minion:{}", id))
    }

    pub fn set_minion(&self, minion: Minion) -> Result<(), String> {
        self.save_object(&format!("minion:{}", minion.id), &minion)
    }

    pub fn set_minion_last_seen(&self, minion_id: &str, time: ResaltTime) -> Result<(), String> {
        let key = format!("minion:{}:last_seen", minion_id);
        self.s.set(&key, &time.to_string())
    }

    pub fn set_minion_grains(
        &self,
        minion_id: &str,
        time: ResaltTime,
        grains: String,
        os_type: String,
    ) -> Result<(), String> {
        let key = format!("minion:{}:grains", minion_id);
        self.s.set(&key, &grains)?;
        let key = format!("minion:{}:os_type", minion_id);
        self.s.set(&key, &os_type)?;
        let key = format!("minion:{}:grains_time", minion_id);
        self.s.set(&key, &time.to_string())
    }

    pub fn set_minion_pillars(
        &self,
        minion_id: &str,
        time: ResaltTime,
        pillars: String,
    ) -> Result<(), String> {
        let key = format!("minion:{}:pillars", minion_id);
        self.s.set(&key, &pillars)?;
        let key = format!("minion:{}:pillars_time", minion_id);
        self.s.set(&key, &time.to_string())
    }

    pub fn set_minion_pkgs(
        &self,
        minion_id: &str,
        time: ResaltTime,
        pkgs: String,
    ) -> Result<(), String> {
        let key = format!("minion:{}:pkgs", minion_id);
        self.s.set(&key, &pkgs)?;
        let key = format!("minion:{}:pkgs_time", minion_id);
        self.s.set(&key, &time.to_string())
    }

    pub fn set_minion_conformity(
        &self,
        minion_id: &str,
        time: ResaltTime,
        conformity: String,
        success: i32,
        incorrect: i32,
        error: i32,
    ) -> Result<(), String> {
        let key = format!("minion:{}:conformity", minion_id);
        self.s.set(&key, &conformity)?;
        let key = format!("minion:{}:conformity_time", minion_id);
        self.s.set(&key, &time.to_string())?;
        let key = format!("minion:{}:conformity_success", minion_id);
        self.s.set(&key, &success.to_string())?;
        let key = format!("minion:{}:conformity_incorrect", minion_id);
        self.s.set(&key, &incorrect.to_string())?;
        let key = format!("minion:{}:conformity_error", minion_id);
        self.s.set(&key, &error.to_string())
    }

    pub fn delete_minion(&self, id: &str) -> Result<(), String> {
        match self.s.del(&format!("minion:{}", id)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    //
    // Minion Presets
    //

    pub fn insert_minion_preset(
        &self,
        id: Option<String>,
        name: &str,
        filter: &str,
    ) -> Result<String, String> {
        let minion_preset = MinionPreset {
            id: id.unwrap_or_else(|| Storage::id("mnp")),
            name: name.to_string(),
            filter: filter.to_string(),
        };
        self.save_object(
            &format!("minion_preset:{}", minion_preset.id),
            &minion_preset,
        )?;
        Ok(minion_preset.id)
    }

    pub fn get_minion_preset_by_id(&self, id: &str) -> Result<Option<MinionPreset>, String> {
        self.read_object(&format!("minion_preset:{}", id))
    }

    pub fn update_minion_preset(&self, minion_preset: &MinionPreset) -> Result<(), String> {
        self.save_object(
            &format!("minion_preset:{}", minion_preset.id),
            minion_preset,
        )
    }

    pub fn delete_minion_preset(&self, id: &str) -> Result<(), String> {
        match self.s.del(&format!("minion_preset:{}", id)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn list_minion_presets(&self) -> Result<Vec<MinionPreset>, String> {
        let keys = match self.keys_depth("minion_preset:*", 2) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };

        // Read minion presets
        let mut minion_presets: Vec<MinionPreset> = Vec::new();
        for key in keys {
            let minion_preset = match self.read_object(&key) {
                Ok(Some(minion_preset)) => minion_preset,
                Ok(None) => continue,
                Err(e) => return Err(e),
            };
            minion_presets.push(minion_preset);
        }

        Ok(minion_presets)
    }

    //
    // Users
    //

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
    pub fn create_user_hashed(
        &self,
        id: Option<String>,
        username: String,
        password: Option<String>,
        perms: String,
        last_login: Option<ResaltTime>,
        email: Option<String>,
    ) -> Result<User, String> {
        let user = User {
            id: id.unwrap_or_else(|| Storage::id("usr")),
            username,
            password,
            perms,
            last_login,
            email,
        };
        self.save_object(&format!("user:{}", user.id), &user)?;
        Ok(user)
    }

    pub fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        self.read_object(&format!("user:{}", id))
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        // Look for user:<userId>:username
        let keys = match self.s.keys(&format!("user:*:username")) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };
        for key in keys {
            if self.s.get(&key).unwrap_or_else(|_| None) == Some(username.to_string()) {
                let user_id_username: Vec<&str> = key.split(':').collect();
                if user_id_username.len() != 3 {
                    continue;
                }
                return self.get_user_by_id(user_id_username[1]);
            }
        }
        Ok(None)
    }

    pub fn set_user(&self, user: &User) -> Result<(), String> {
        self.save_object(&format!("user:{}", user.id), user)
    }

    pub fn set_user_last_login(&self, user_id: &str, time: ResaltTime) -> Result<(), String> {
        let key = format!("user:{}:last_login", user_id);
        self.s.set(&key, &time.to_string())
    }

    pub fn delete_user(&self, id: &str) -> Result<(), String> {
        match self.s.del(&format!("user:{}", id)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn list_users(&self, paginate: Paginate) -> Result<Vec<User>, String> {
        let keys = match self.keys_depth("user:*", 2) {
            Ok(keys) => keys,
            Err(e) => return Err(e),
        };

        // Read users
        let mut users: Vec<User> = Vec::new();
        for key in keys {
            let user = match self.read_object(&key) {
                Ok(Some(user)) => user,
                Ok(None) => continue,
                Err(e) => return Err(e),
            };
            users.push(user);
        }

        // Pagination
        if let Some((limit, offset)) = paginate {
            users = users
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect();
        }

        Ok(users)
    }

    //
    // Auth tokens
    //

    /// Create a new auth token.
    ///
    /// If user does not exist, this function will return an error.
    pub fn create_authtoken(&self, user_id: String) -> Result<AuthToken, String> {
        let auth_token = AuthToken {
            id: Storage::id("autht"),
            user_id,
            timestamp: ResaltTime::now(),
            salt_token: None,
        };
        self.save_object(&format!("auth_token:{}", auth_token.id), &auth_token)?;

        // Update users's last_login
        self.set_user_last_login(&auth_token.user_id, auth_token.timestamp)?;

        Ok(auth_token)
    }

    pub fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        self.read_object(&format!("auth_token:{}", id))
    }

    pub fn update_authtoken_salttoken(
        &self,
        auth_token: &str,
        salt_token: Option<&SaltToken>,
    ) -> Result<(), String> {
        let salt_token_str = salt_token
            .as_ref()
            .map(|st| serde_json::to_string(st).unwrap());

        // Update authtoken with salt_token
        self.s.set(
            &format!("auth_token:{}:salt_token", auth_token),
            &salt_token_str.unwrap_or_else(|| "".to_string()),
        )
    }
}
