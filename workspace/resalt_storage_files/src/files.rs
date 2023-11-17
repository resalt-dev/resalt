use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

use resalt_models::*;
use resalt_storage::{StorageImpl, StorageStatus};

/// Dev storage which saves everything to filesystem instead of using a database
/// NOTE! NOT MEANT FOR PRODUCTION!!!
///
/// Structure:
/// e.g. PATH=./files
///
/// ./files/users/<user_id>.json
/// ./files/authtokens/<authtoken_id>.json
/// ./files/minions/<minion_id>.json
/// ./files/events/<event_id>.json
/// etc.

#[derive(Clone)]
pub struct StorageFiles {
    path: String, // MUST BE WITHOUT TRAILING SLASH
}

impl StorageFiles {
    pub fn connect(path: &str) -> Result<StorageFiles, String> {
        log::info!("Connecting to files storage at {}", path);
        let path = path.trim_end_matches('/');
        let storage = StorageFiles {
            path: path.to_string(),
        };

        // Create directories if they don't exist
        let dirs = vec![
            "users",
            "authtokens",
            "minions",
            "events",
            "jobs",
            "job_returns",
            "permission_groups",
            "minion_presets",
        ];
        for dir in dirs {
            let path = format!("{}/{}", storage.path, dir);
            std::fs::create_dir_all(path).map_err(|e| format!("{:?}", e))?;
        }

        Ok(storage)
    }

    fn save_file(&self, path: &str, data: &impl Serialize) -> Result<(), String> {
        let path = format!("{}/{}.json", self.path, path);
        let serialized_data = serde_json::to_string(data).map_err(|e| format!("{:?}", e))?;
        let mut file = std::fs::File::create(path).map_err(|e| format!("{:?}", e))?;
        file.write_all(serialized_data.as_bytes())
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn load_file<T>(&self, path: &str) -> Result<T, String>
    where
        for<'de> T: Deserialize<'de>,
    {
        let path = format!("{}/{}.json", self.path, path);
        let mut file = std::fs::File::open(path).map_err(|e| format!("{:?}", e))?;
        let mut serialized_data = String::new();
        file.read_to_string(&mut serialized_data)
            .map_err(|e| format!("{:?}", e))?;
        let data: T = serde_json::from_str(&serialized_data).map_err(|e| format!("{:?}", e))?;
        Ok(data)
    }

    fn check_file_exists(&self, path: &str) -> Result<bool, String> {
        let path = format!("{}/{}.json", self.path, path);
        let exists = std::path::Path::new(&path).exists();
        Ok(exists)
    }

    fn list_file_names(&self, path: &str) -> Result<Vec<String>, String> {
        let path = format!("{}/{}", self.path, path);
        let mut file_names: Vec<String> = Vec::new();
        for entry in std::fs::read_dir(path).map_err(|e| format!("{:?}", e))? {
            let entry = entry.map_err(|e| format!("{:?}", e))?;
            let file_name = entry
                .file_name()
                .into_string()
                .map_err(|e| format!("{:?}", e))?;
            let file_name = file_name.trim_end_matches(".json").to_string();
            file_names.push(file_name);
        }
        Ok(file_names)
    }

    fn delete_file(&self, path: &str) -> Result<(), String> {
        let path = format!("{}/{}.json", self.path, path);
        std::fs::remove_file(path).map_err(|e| format!("{:?}", e))?;
        Ok(())
    }
}

impl StorageImpl for StorageFiles {
    fn clone(&self) -> Box<dyn StorageImpl> {
        Box::new(Clone::clone(self))
    }

    fn get_status(&self) -> Result<resalt_storage::StorageStatus, String> {
        //

        //let lifespan = SConfig::auth_session_lifespan() * 1000;
        // let auth_expiry: NaiveDateTime = match NaiveDateTime::from_timestamp_millis(
        //     Utc::now().timestamp_millis() - (lifespan as i64),
        // ) {
        //     Some(dt) => dt,
        //     None => return Err("Failed to convert timestamp to NaiveDateTime: {:?}".to_string()),
        // };

        let auth_tokens_total = -1;
        let auth_tokens_active = -1;
        let events_total = -1;
        let job_returns_total = -1;
        let jobs_total = -1;

        // last_updated_conformity != null, and conformity_error = 0 and conformity_incorrect = 0
        // let minions_success = minions::table
        //     .filter(minions::last_updated_conformity.is_not_null())
        //     .filter(minions::conformity_error.eq(0))
        //     .filter(minions::conformity_incorrect.eq(0))
        //     .count()
        //     .get_result::<i64>(&mut connection)
        //     .map_err(|e| format!("{:?}", e))?;
        // let minions_incorrect = minions::table
        //     .filter(minions::last_updated_conformity.is_not_null())
        //     .filter(minions::conformity_error.eq(0))
        //     .filter(minions::conformity_incorrect.ne(0))
        //     .count()
        //     .get_result::<i64>(&mut connection)
        //     .map_err(|e| format!("{:?}", e))?;
        // let minions_error = minions::table
        //     .filter(minions::last_updated_conformity.is_not_null())
        //     .filter(minions::conformity_error.ne(0))
        //     .count()
        //     .get_result::<i64>(&mut connection)
        //     .map_err(|e| format!("{:?}", e))?;
        // let minions_unknown = minions::table
        //     .filter(minions::last_updated_conformity.is_null())
        //     .count()
        //     .get_result::<i64>(&mut connection)
        //     .map_err(|e| format!("{:?}", e))?;
        // let minions_total = minions_success + minions_incorrect + minions_error + minions_unknown;

        let permission_group_users_total = -1;
        let permission_groups_total = -1;
        let users_total = -1;
        Ok(StorageStatus {
            auth_tokens_total,
            auth_tokens_active,
            events_total,
            job_returns_total,
            jobs_total,
            minions_total: -1,
            minions_success: -1,
            minions_incorrect: -1,
            minions_error: -1,
            minions_unknown: -1,
            permission_group_users_total,
            permission_groups_total,
            users_total,
        })
    }

    /////////////
    /// Users ///
    /////////////

    fn create_user_hashed(
        &self,
        id: Option<String>,
        username: String,
        password: Option<String>,
        perms: String,
        last_login: Option<ResaltTime>,
        email: Option<String>,
        ldap_sync: Option<String>,
    ) -> Result<User, String> {
        let id = id.unwrap_or(format!("usr_{}", uuid::Uuid::new_v4()));
        let user = User {
            id: id.clone(),
            username: username.clone(),
            password,
            perms,
            last_login,
            email: email.clone(),
            ldap_sync: ldap_sync.clone(),
        };

        let path = format!("users/{}.json", id);
        self.save_file(&path, &user)?;

        Ok(user)
    }

    fn list_users(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, String> {
        let mut users: Vec<User> = Vec::new();

        let mut keys: Vec<String> = self.list_file_names("users")?;
        keys.sort();

        // Skip offset & Limit
        keys = keys
            .into_iter()
            .skip(offset.unwrap_or(0) as usize)
            .take(limit.unwrap_or(100) as usize)
            .collect();

        for key in keys {
            let user = self.get_user_by_id(&key)?;
            match user {
                Some(user) => users.push(user),
                None => continue,
            }
        }

        Ok(users)
    }

    fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        // If user doesn't exist, return None
        let exists = self.check_file_exists(&format!("users/{}", id))?;
        if !exists {
            return Ok(None);
        }

        let user: User = self.load_file(&format!("users/{}", id))?;

        Ok(Some(user))
    }

    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let mut users: Vec<User> = Vec::new();

        let mut keys: Vec<String> = self.list_file_names("users")?;
        keys.sort();

        for key in keys {
            let user = self.get_user_by_id(&key)?;
            match user {
                Some(user) => users.push(user),
                None => continue,
            }
        }

        let user = users.into_iter().find(|u| u.username == username);

        Ok(user)
    }

    fn update_user(&self, user: &User) -> Result<(), String> {
        let path = format!("users/{}", user.id);
        self.save_file(&path, user)?;
        Ok(())
    }

    fn delete_user(&self, id: &str) -> Result<(), String> {
        let path = format!("users/{}", id);
        self.delete_file(&path)?;
        Ok(())
    }

    ///////////////////
    /// Auth tokens ///
    ///////////////////

    fn create_authtoken(&self, user_id: String) -> Result<AuthToken, String> {
        let id = format!("auth_{}", uuid::Uuid::new_v4());
        let authtoken = AuthToken {
            id: id.clone(),
            user_id: user_id.clone(),
            timestamp: ResaltTime::now(),
            salt_token: None,
        };

        let path = format!("authtokens/{}", id);
        self.save_file(&path, &authtoken)?;

        // Update user's last_login
        let mut user = self.get_user_by_id(&user_id)?.unwrap();
        user.last_login = Some(authtoken.timestamp);
        self.update_user(&user)?;

        Ok(authtoken)
    }

    fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        // If authtoken doesn't exist, return None
        let exists = self.check_file_exists(&format!("authtokens/{}", id))?;
        if !exists {
            return Ok(None);
        }

        let authtoken: AuthToken = self.load_file(&format!("authtokens/{}", id))?;

        Ok(Some(authtoken))
    }

    fn update_authtoken_salttoken(
        &self,
        auth_token: &str,
        salt_token: Option<&SaltToken>,
    ) -> Result<(), String> {
        let salt_token_str = salt_token
            .as_ref()
            .map(|st| serde_json::to_string(st).unwrap());

        // Update authtoken with salt_token
        let mut authtoken = self.get_authtoken_by_id(auth_token)?.unwrap();
        authtoken.salt_token = salt_token_str;
        let path = format!("authtokens/{}", auth_token);
        self.save_file(&path, &authtoken)?;

        Ok(())
    }

    ///////////////
    /// Minions ///
    ///////////////

    fn list_minions(
        &self,
        filters: Vec<Filter>,
        sort: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Minion>, String> {
        let mut minions: Vec<Minion> = Vec::new();

        let mut keys = self.list_file_names("minions")?;
        keys.sort();

        // QUICK PAGINATION (Skip offset & Limit)
        if filters.is_empty() {
            keys = keys
                .into_iter()
                .skip(offset.unwrap_or(0) as usize)
                .take(limit.unwrap_or(100) as usize)
                .collect();
        }

        for key in keys {
            let minion = self.get_minion_by_id(&key)?;
            match minion {
                Some(minion) => minions.push(minion),
                None => continue,
            }
        }

        // Filtering
        resalt_storage::filter_minions(&mut minions, &filters);

        // Sorting
        let sort = sort.unwrap_or("id.asc".to_string());
        resalt_storage::sort_minions(&mut minions, &sort);

        // SLOW PAGINATION (Skip offset & Limit)
        if !filters.is_empty() {
            let offset = offset.unwrap_or(0) as usize;
            let limit = limit.unwrap_or(100) as usize;
            minions = minions.into_iter().skip(offset).take(limit).collect();
        }

        Ok(minions)
    }

    fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String> {
        // If minion doesn't exist, return None
        let exists = self.check_file_exists(&format!("minions/{}", id))?;
        if !exists {
            return Ok(None);
        }

        let minion: Minion = self.load_file(&format!("minions/{}", id))?;

        Ok(Some(minion))
    }

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
        last_updated_grains: Option<ResaltTime>,
        last_updated_pillars: Option<ResaltTime>,
        last_updated_pkgs: Option<ResaltTime>,
        last_updated_conformity: Option<ResaltTime>,
    ) -> Result<(), String> {
        let last_updated_grains = grains
            .as_ref()
            .map(|_| last_updated_grains.unwrap_or(time.into()));
        let last_updated_pillars = pillars
            .as_ref()
            .map(|_| last_updated_pillars.unwrap_or(time.into()));
        let last_updated_pkgs = pkgs
            .as_ref()
            .map(|_| last_updated_pkgs.unwrap_or(time.into()));
        let last_updated_conformity = conformity
            .as_ref()
            .map(|_| last_updated_conformity.unwrap_or(time.into()));

        // Parse grains as JSON, and fetch osfullname+osrelease as os_type.
        let parsed_grains = grains
            .as_ref()
            .map(|grains| serde_json::from_str::<serde_json::Value>(grains).unwrap());
        let os_type = match parsed_grains {
            Some(grains) => {
                let osfullname = grains["osfullname"].as_str().unwrap_or("Unknown");
                let osrelease = grains["osrelease"].as_str().unwrap_or("");
                Some(format!("{} {}", osfullname, osrelease).trim().to_string())
            }
            None => None,
        };

        let minion = Minion {
            id: minion_id.clone(),
            last_seen: time.into(),
            grains,
            pillars,
            pkgs,
            last_updated_grains,
            last_updated_pillars,
            last_updated_pkgs,
            conformity,
            conformity_success,
            conformity_incorrect,
            conformity_error,
            last_updated_conformity,
            os_type,
        };

        // Update if it exists, insert if it doesn't
        let path = format!("minions/{}", minion_id);
        self.save_file(&path, &minion)?;

        Ok(())
    }

    // Delete minion
    fn delete_minion(&self, id: String) -> Result<(), String> {
        let path = format!("minions/{}", id);
        self.delete_file(&path)?;
        Ok(())
    }

    //////////////
    /// Events ///
    //////////////

    fn insert_event(
        &self,
        tag: String,
        data: String,
        timestamp: chrono::NaiveDateTime,
    ) -> Result<String, String> {
        let id = format!("evnt_{}", uuid::Uuid::new_v4());
        let event = Event {
            id: id.clone(),
            timestamp: timestamp.into(),
            tag,
            data,
        };

        let path = format!("events/{}", id);
        self.save_file(&path, &event)?;

        Ok(id)
    }

    fn list_events(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Event>, String> {
        let mut events: Vec<Event> = Vec::new();

        // Loop over event:*, which are HashMaps
        let mut keys = self.list_file_names("events")?;
        keys.sort();

        // Skip offset & Limit
        keys = keys
            .into_iter()
            .skip(offset.unwrap_or(0) as usize)
            .take(limit.unwrap_or(100) as usize)
            .collect();

        for key in keys {
            let event = self.get_event_by_id(&key)?;
            match event {
                Some(event) => events.push(event),
                None => continue,
            }
        }

        Ok(events)
    }

    fn get_event_by_id(&self, id: &str) -> Result<Option<Event>, String> {
        // If event doesn't exist, return None
        let exists = self.check_file_exists(&format!("events/{}", id))?;
        if !exists {
            return Ok(None);
        }

        let event: Event = self.load_file(&format!("events/{}", id))?;

        Ok(Some(event))
    }

    ////////////
    /// Jobs ///
    ////////////

    fn insert_job(
        &self,
        jid: String,
        user: Option<String>,
        event_id: Option<String>,
        timestamp: chrono::NaiveDateTime,
    ) -> Result<(), String> {
        let job = Job {
            id: jid.clone(),
            timestamp: timestamp.into(),
            jid: jid.clone(), // TODO: remove, id = jid
            user,
            event_id,
        };

        let path = format!("jobs/{}", jid);
        self.save_file(&path, &job)?;

        Ok(())
    }

    fn list_jobs(
        &self,
        sort: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Job>, String> {
        let mut jobs: Vec<Job> = Vec::new();

        // Loop over job:*, which are HashMaps
        let mut keys = self.list_file_names("jobs")?;
        keys.sort();

        // Skip offset & Limit
        keys = keys
            .into_iter()
            .skip(offset.unwrap_or(0) as usize)
            .take(limit.unwrap_or(100) as usize)
            .collect();

        for key in keys {
            let job = self.get_job_by_jid(&key)?;
            match job {
                Some(job) => jobs.push(job),
                None => continue,
            }
        }

        // Sorting
        let sort = sort.unwrap_or("id.asc".to_string());
        resalt_storage::sort_jobs(&mut jobs, &sort);

        Ok(jobs)
    }

    fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String> {
        // If job doesn't exist, return None
        let exists = self.check_file_exists(&format!("jobs/{}", jid))?;
        if !exists {
            return Ok(None);
        }

        let job: Job = self.load_file(&format!("jobs/{}", jid))?;

        Ok(Some(job))
    }

    ///////////////////
    /// Job Returns ///
    ///////////////////

    fn insert_job_return(
        &self,
        jid: String,
        job_id: String,
        event_id: String,
        minion_id: String,
        timestamp: chrono::NaiveDateTime,
    ) -> Result<(), String> {
        let id = format!("jret_{}", uuid::Uuid::new_v4());
        let job_return = JobReturn {
            id: "".to_string(),
            timestamp: timestamp.into(),
            jid: jid.clone(),
            job_id,
            event_id,
            minion_id,
        };

        let path = format!("job_returns/{}", id);
        self.save_file(&path, &job_return)?;

        Ok(())
    }

    fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<JobReturn>, String> {
        let mut job_returns: Vec<JobReturn> = Vec::new();

        // Loop over job_return:<jid>:*
        let mut keys = self.list_file_names("job_returns")?;
        keys.sort();

        for key in keys {
            let path = format!("job_returns/{}", key);
            let job_return: JobReturn = self.load_file(&path)?;
            if job_return.job_id != job.id {
                continue;
            }
            job_returns.push(job_return);
        }

        Ok(job_returns)
    }

    /////////////////////////
    /// Permission Groups ///
    /////////////////////////

    fn create_permission_group(
        &self,
        id: Option<String>,
        name: &str,
        perms: Option<String>,
    ) -> Result<String, String> {
        let id = id.unwrap_or(format!("pg_{}", uuid::Uuid::new_v4()));
        let permission_group = PermissionGroup {
            id: id.clone(),
            name: name.to_owned(),
            perms: perms.unwrap_or("[]".to_string()),
            ldap_sync: None,
        };

        let path = format!("permission_groups/{}", id);
        self.save_file(&path, &permission_group)?;

        Ok(id)
    }

    fn list_permission_groups(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<PermissionGroup>, String> {
        let mut permission_groups: Vec<PermissionGroup> = Vec::new();

        let mut keys: Vec<String> = self.list_file_names("permission_groups")?;
        keys.sort();

        // Skip offset & Limit
        keys = keys
            .into_iter()
            .skip(offset.unwrap_or(0) as usize)
            .take(limit.unwrap_or(100) as usize)
            .collect();

        for key in keys {
            let permission_group = self.get_permission_group_by_id(&key)?;
            match permission_group {
                Some(permission_group) => permission_groups.push(permission_group),
                None => continue,
            }
        }

        Ok(permission_groups)
    }

    fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String> {
        // If permission_group doesn't exist, return None
        let exists = self.check_file_exists(&format!("permission_groups/{}", id))?;
        if !exists {
            return Ok(None);
        }

        let permission_group: PermissionGroup =
            self.load_file(&format!("permission_groups/{}", id))?;

        Ok(Some(permission_group))
    }

    /// DEPRECATED
    fn get_permission_group_by_ldap_sync(
        &self,
        _ldap_sync: &str,
    ) -> Result<Option<PermissionGroup>, String> {
        Ok(None)
    }

    fn is_user_member_of_group(&self, user_id: &str, group_id: &str) -> Result<bool, String> {
        let path = format!("permission_group_user/{}/{}", user_id, group_id);
        let exists = self.check_file_exists(&path)?;

        Ok(exists)
    }

    fn update_permission_group(&self, permission_group: &PermissionGroup) -> Result<(), String> {
        let path = format!("permission_groups/{}", permission_group.id);
        self.save_file(&path, permission_group)?;
        Ok(())
    }

    fn delete_permission_group(&self, id: &str) -> Result<(), String> {
        let path = format!("permission_groups/{}", id);
        self.delete_file(&path)?;
        Ok(())
    }

    fn insert_permission_group_user(&self, user_id: &str, group_id: &str) -> Result<(), String> {
        let path = format!("permission_group_user/{}/{}", user_id, group_id);
        self.save_file(&path, &())?;
        Ok(())
    }

    fn list_permission_groups_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<PermissionGroup>, String> {
        let mut permission_groups: Vec<PermissionGroup> = Vec::new();

        // Loop over permission_group_user:<user_id>:*
        let keys = self.list_file_names(&format!("permission_group_user/{}", user_id))?;

        for group_id in keys {
            let permission_group = self.get_permission_group_by_id(&group_id)?;
            match permission_group {
                Some(permission_group) => permission_groups.push(permission_group),
                None => continue,
            }
        }

        Ok(permission_groups)
    }

    fn list_users_by_permission_group_id(&self, group_id: &str) -> Result<Vec<User>, String> {
        let mut users: Vec<User> = Vec::new();

        // Search for Hashmap where permission_group_user:*:<group_id> exists
        let keys: Vec<String> = self.list_file_names("users")?;

        for user_id in keys {
            let key = format!("permission_group_user/{}/{}", user_id, group_id);
            let exists = self.check_file_exists(&key)?;
            if !exists {
                continue;
            }

            let user = self.get_user_by_id(&user_id)?;
            match user {
                Some(user) => users.push(user),
                None => continue,
            }
        }

        Ok(users)
    }

    fn delete_permission_group_user(&self, user_id: &str, group_id: &str) -> Result<(), String> {
        let path = format!("permission_group_user:{}:{}", user_id, group_id);
        self.delete_file(&path)?;
        Ok(())
    }

    //////////////////////
    /// Minion Presets ///
    //////////////////////

    fn insert_minion_preset(
        &self,
        id: Option<String>,
        name: &str,
        filter: &str,
    ) -> Result<String, String> {
        let id = id.unwrap_or(format!("pre_{}", uuid::Uuid::new_v4()));
        let minion_preset = MinionPreset {
            id: id.clone(),
            name: name.to_string(),
            filter: filter.to_string(),
        };

        let path = format!("minion_presets/{}", id);
        self.save_file(&path, &minion_preset)?;

        Ok(id)
    }

    fn list_minion_presets(&self) -> Result<Vec<MinionPreset>, String> {
        let mut minion_presets: Vec<MinionPreset> = Vec::new();

        let mut keys = self.list_file_names("minion_presets")?;
        keys.sort();

        for key in keys {
            let minion_preset = self.get_minion_preset_by_id(&key)?;
            match minion_preset {
                Some(minion_preset) => minion_presets.push(minion_preset),
                None => continue,
            }
        }

        Ok(minion_presets)
    }

    fn get_minion_preset_by_id(&self, id: &str) -> Result<Option<MinionPreset>, String> {
        // If minion_preset doesn't exist, return None
        let exists = self.check_file_exists(&format!("minion_presets/{}", id))?;
        if !exists {
            return Ok(None);
        }

        let minion_preset: MinionPreset = self.load_file(&format!("minion_presets/{}", id))?;

        Ok(Some(minion_preset))
    }

    fn update_minion_preset(&self, minion_preset: &MinionPreset) -> Result<(), String> {
        let path = format!("minion_presets/{}", minion_preset.id);
        self.save_file(&path, minion_preset)?;
        Ok(())
    }

    fn delete_minion_preset(&self, id: &str) -> Result<(), String> {
        let path = format!("minion_presets/{}", id);
        self.delete_file(&path)?;
        Ok(())
    }
}
