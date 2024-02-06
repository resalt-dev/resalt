use r2d2::{Pool, PooledConnection};
use redis::{Client, Commands, Iter, JsonCommands};
use resalt_models::*;
use serde_json;

#[derive(Clone)]
pub struct StorageRedis {
    pool: Pool<Client>,
}

impl StorageRedis {
    pub async fn connect(database_url: &str) -> Result<Self, String> {
        let client = Client::open(database_url).unwrap();
        let pool = Pool::builder().build(client);

        match pool {
            Ok(pool) => Ok(Self { pool }),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    fn create_connection(&self) -> Result<PooledConnection<Client>, String> {
        match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

impl StorageImpl for StorageRedis {
    fn clone(&self) -> Box<dyn StorageImpl> {
        Box::new(Clone::clone(self))
    }

    fn get_status(&self) -> Result<StorageStatus, String> {
        //        let mut connection = self.create_connection()?;

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
        preferences: UserPreferences,
    ) -> Result<User, String> {
        let mut connection = self.create_connection()?;
        let id = id.unwrap_or(format!("usr_{}", uuid::Uuid::new_v4()));
        let user = User {
            id: id.clone(),
            username: username.clone(),
            password,
            perms,
            last_login,
            email: email.clone(),
            preferences: preferences.clone(),
        };

        let user_json = serde_json::to_string(&user).map_err(|e| format!("{:?}", e))?;
        connection
            .set(format!("user:{}", id), &user_json)
            .map_err(|e| format!("{:?}", e))?;

        Ok(user)
    }

    fn list_users(&self, paginate: Paginate) -> Result<Vec<User>, String> {
        let mut connection = self.create_connection()?;
        let mut users: Vec<User> = Vec::new();

        // Loop over user:*, which are JSON strings
        let mut keys: Vec<String> = connection.keys("user:*").map_err(|e| format!("{:?}", e))?;
        keys.sort();

        // Pagination
        if let Some((limit, offset)) = paginate {
            keys = keys
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect();
        }

        for key in keys {
            // Fields are stored as JSON strings
            let user_json: String = connection
                .get(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let user: User =
                serde_json::from_str(user_json.as_str()).map_err(|e| format!("{:?}", e))?;
            users.push(user);
        }

        Ok(users)
    }

    fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let mut connection = self.create_connection()?;
        // If user doesn't exist, return None
        if !connection
            .exists(format!("user:{}", id).as_str())
            .map_err(|e| format!("{:?}", e))?
        {
            return Ok(None);
        }
        let user_json: String = connection
            .get(format!("user:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        let user: User =
            serde_json::from_str(user_json.as_str()).map_err(|e| format!("{:?}", e))?;
        Ok(Some(user))
    }

    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let mut conn_iter = self.create_connection()?;
        let mut conn_lookup = self.create_connection()?;
        // If user doesn't exist, return None

        let keys: Vec<String> = conn_iter
            .scan_match("user:*")
            .map_err(|e| format!("{:?}", e))?
            .collect();

        let user_jsons: Vec<String> = conn_lookup
            .mget(keys.as_slice())
            .map_err(|e| format!("{:?}", e))?;

        for user_json in user_jsons {
            let user: User =
                serde_json::from_str(user_json.as_str()).map_err(|e| format!("{:?}", e))?;

            if user.username == username {
                return Ok(Some(user));
            }
        }

        Ok(None)
    }

    fn update_user(&self, user: &User) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let user_json = serde_json::to_string(user).map_err(|e| format!("{:?}", e))?;
        connection
            .set(format!("user:{}", user.id), &user_json)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn update_user_preferences(
        &self,
        user_id: &str,
        preferences: &UserPreferences,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection
            .json_set(format!("user:{}", user_id), ".preferences", &preferences)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn delete_user(&self, id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection
            .del(format!("user:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    ///////////////////
    /// Auth tokens ///
    ///////////////////

    fn create_authtoken(&self, user_id: String) -> Result<AuthToken, String> {
        let mut connection = self.create_connection()?;
        let id = format!("auth_{}", uuid::Uuid::new_v4());
        let authtoken = AuthToken {
            id: id.clone(),
            user_id: user_id.clone(),
            timestamp: ResaltTime::now(),
            salt_token: None,
        };

        // Insert auth token
        let auth_token_json = serde_json::to_string(&authtoken).map_err(|e| format!("{:?}", e))?;
        connection
            .set(format!("authtoken:{}", id), &auth_token_json)
            .map_err(|e| format!("{:?}", e))?;

        // Update user's last_login
        connection
            .json_set(
                format!("user:{}", user_id),
                ".last_login",
                &authtoken.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            )
            .map_err(|e| format!("{:?}", e))?;

        Ok(authtoken)
    }

    fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        let mut connection = self.create_connection()?;
        // If authtoken doesn't exist, return None
        if !connection
            .exists(format!("authtoken:{}", id).as_str())
            .map_err(|e| format!("{:?}", e))?
        {
            return Ok(None);
        }

        let authtoken_json: String = connection
            .get(format!("authtoken:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        let authtoken: AuthToken =
            serde_json::from_str(authtoken_json.as_str()).map_err(|e| format!("{:?}", e))?;

        Ok(Some(authtoken))
    }

    fn update_authtoken_salttoken(
        &self,
        auth_token: &str,
        salt_token: Option<&SaltToken>,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;

        let salt_token_str = salt_token
            .as_ref()
            .map(|st| serde_json::to_string(st).unwrap());

        // Update authtoken with salt_token
        connection
            .json_set(
                format!("authtoken:{}", auth_token),
                ".salt_token",
                &salt_token_str,
            )
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    ///////////////
    /// Minions ///
    ///////////////

    fn list_minions(
        &self,
        filters: Vec<Filter>,
        sort: Option<MinionSort>,
        paginate: Paginate,
    ) -> Result<Vec<Minion>, String> {
        let mut connection = self.create_connection()?;
        let mut minions: Vec<Minion> = Vec::new();

        // Loop over minion:*, which are HashMaps
        let mut keys: Vec<String> = connection
            .keys("minion:*")
            .map_err(|e| format!("{:?}", e))?;
        keys.sort();

        // QUICK PAGINATION (Skip offset & Limit)
        if filters.is_empty() {
            // Pagination
            if let Some((limit, offset)) = paginate {
                keys = keys
                    .into_iter()
                    .skip(offset as usize)
                    .take(limit as usize)
                    .collect();
            }
        }

        for key in &keys {
            let minion_json: String = connection
                .get(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let minion: Minion =
                serde_json::from_str(minion_json.as_str()).map_err(|e| format!("{:?}", e))?;
            minions.push(minion);
        }

        // Filtering
        filter_minions(&mut minions, &filters);

        // Sorting
        if let Some(sort) = sort {
            sort_minions(&mut minions, &sort);
        }

        // SLOW PAGINATION (Skip offset & Limit)
        if !filters.is_empty() {
            // Pagination
            if let Some((limit, offset)) = paginate {
                minions = minions
                    .into_iter()
                    .skip(offset as usize)
                    .take(limit as usize)
                    .collect();
            }
        }

        Ok(minions)
    }

    fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String> {
        let mut connection = self.create_connection()?;
        // If minion doesn't exist, return None
        if !connection
            .exists(format!("minion:{}", id).as_str())
            .map_err(|e| format!("{:?}", e))?
        {
            return Ok(None);
        }

        let minion_json: String = connection
            .get(format!("minion:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        let minion: Minion =
            serde_json::from_str(minion_json.as_str()).map_err(|e| format!("{:?}", e))?;

        Ok(Some(minion))
    }

    fn upsert_minion(&self, minion: Minion) -> Result<(), String> {
        let mut connection = self.create_connection()?;

        // Update if it exists, insert if it doesn't
        let minion_json = serde_json::to_string(&minion).map_err(|e| format!("{:?}", e))?;
        connection
            .set(format!("minion:{}", minion.id), &minion_json)
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    fn upsert_minion_last_seen(&self, minion_id: &str, time: ResaltTime) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection
            .json_set(
                format!("minion:{}", minion_id),
                ".last_seen",
                &time.to_string(),
            )
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn upsert_minion_grains(
        &self,
        minion_id: &str,
        time: ResaltTime,
        grains: String,
        os_type: String,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection
            .json_set(
                format!("minion:{}", minion_id),
                ".last_updated_grains",
                &time.to_string(),
            )
            .map_err(|e| format!("{:?}", e))?;
        connection
            .json_set(format!("minion:{}", minion_id), ".grains", &grains)
            .map_err(|e| format!("{:?}", e))?;
        connection
            .json_set(format!("minion:{}", minion_id), ".os_type", &os_type)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn upsert_minion_pillars(
        &self,
        minion_id: &str,
        time: ResaltTime,
        pillars: String,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection
            .json_set(
                format!("minion:{}", minion_id),
                ".last_updated_pillars",
                &time.to_string(),
            )
            .map_err(|e| format!("{:?}", e))?;
        connection
            .json_set(format!("minion:{}", minion_id), ".pillars", &pillars)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn upsert_minion_pkgs(
        &self,
        minion_id: &str,
        time: ResaltTime,
        pkgs: String,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection
            .json_set(
                format!("minion:{}", minion_id),
                ".last_updated_pkgs",
                &time.to_string(),
            )
            .map_err(|e| format!("{:?}", e))?;
        connection
            .json_set(format!("minion:{}", minion_id), ".pkgs", &pkgs)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn upsert_minion_conformity(
        &self,
        minion_id: &str,
        time: ResaltTime,
        conformity: String,
        success: i32,
        incorrect: i32,
        error: i32,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection
            .json_set(
                format!("minion:{}", minion_id),
                ".last_updated_conformity",
                &time.to_string(),
            )
            .map_err(|e| format!("{:?}", e))?;
        connection
            .json_set(format!("minion:{}", minion_id), ".conformity", &conformity)
            .map_err(|e| format!("{:?}", e))?;
        connection
            .json_set(
                format!("minion:{}", minion_id),
                ".conformity_success",
                &success,
            )
            .map_err(|e| format!("{:?}", e))?;
        connection
            .json_set(
                format!("minion:{}", minion_id),
                ".conformity_incorrect",
                &incorrect,
            )
            .map_err(|e| format!("{:?}", e))?;
        connection
            .json_set(format!("minion:{}", minion_id), ".conformity_error", &error)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    // Delete minion
    fn delete_minion(&self, id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection
            .del(format!("minion:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    //////////////
    /// Events ///
    //////////////

    fn insert_event(
        &self,
        tag: String,
        data: String,
        timestamp: ResaltTime,
    ) -> Result<String, String> {
        let mut connection = self.create_connection()?;
        let id = format!("evnt_{}", uuid::Uuid::new_v4());
        let event = Event {
            id: id.clone(),
            timestamp,
            tag,
            data,
        };

        let event_json = serde_json::to_string(&event).map_err(|e| format!("{:?}", e))?;
        connection
            .set(format!("event:{}", id), &event_json)
            .map_err(|e| format!("{:?}", e))?;
        Ok(id)
    }

    fn list_events(&self, paginate: Paginate) -> Result<Vec<Event>, String> {
        let mut connection = self.create_connection()?;
        let mut events: Vec<Event> = Vec::new();

        // Loop over event:*, which are HashMaps
        let mut keys: Vec<String> = connection.keys("event:*").map_err(|e| format!("{:?}", e))?;
        keys.sort();

        // Pagination
        if let Some((limit, offset)) = paginate {
            keys = keys
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect();
        }

        for key in keys {
            let event_json: String = connection
                .get(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let event: Event =
                serde_json::from_str(event_json.as_str()).map_err(|e| format!("{:?}", e))?;
            events.push(event);
        }

        Ok(events)
    }

    fn get_event_by_id(&self, id: &str) -> Result<Option<Event>, String> {
        let mut connection = self.create_connection()?;
        // If event doesn't exist, return None
        if !connection
            .exists(format!("event:{}", id).as_str())
            .map_err(|e| format!("{:?}", e))?
        {
            return Ok(None);
        }

        let event_json: String = connection
            .get(format!("event:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        let event: Event =
            serde_json::from_str(event_json.as_str()).map_err(|e| format!("{:?}", e))?;
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
        timestamp: ResaltTime,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let job = Job {
            id: jid.clone(),
            timestamp,
            jid: jid.clone(), // TODO: remove, id = jid
            user,
            event_id,
        };

        let job_json = serde_json::to_string(&job).map_err(|e| format!("{:?}", e))?;
        connection
            .set(format!("job:{}", jid), &job_json)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn list_jobs(&self, sort: Option<JobSort>, paginate: Paginate) -> Result<Vec<Job>, String> {
        let mut connection = self.create_connection()?;
        let mut jobs: Vec<Job> = Vec::new();

        // Loop over job:*, which are HashMaps
        let mut keys: Vec<String> = connection.keys("job:*").map_err(|e| format!("{:?}", e))?;
        keys.sort();

        // Pagination
        if let Some((limit, offset)) = paginate {
            keys = keys
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect();
        }

        for key in keys {
            let job_json: String = connection
                .get(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let job: Job =
                serde_json::from_str(job_json.as_str()).map_err(|e| format!("{:?}", e))?;
            jobs.push(job);
        }

        // Sorting
        if let Some(sort) = sort {
            sort_jobs(&mut jobs, &sort);
        }

        Ok(jobs)
    }

    fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String> {
        let mut connection = self.create_connection()?;
        // If job doesn't exist, return None
        if !connection
            .exists(format!("job:{}", jid).as_str())
            .map_err(|e| format!("{:?}", e))?
        {
            return Ok(None);
        }

        let job_json: String = connection
            .get(format!("job:{}", jid))
            .map_err(|e| format!("{:?}", e))?;
        let job: Job = serde_json::from_str(job_json.as_str()).map_err(|e| format!("{:?}", e))?;
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
        timestamp: ResaltTime,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let id = format!("jret_{}", uuid::Uuid::new_v4());
        let job_return = JobReturn {
            id: "".to_string(),
            timestamp,
            jid: jid.clone(),
            job_id,
            event_id,
            minion_id,
        };

        let job_return_json = serde_json::to_string(&job_return).map_err(|e| format!("{:?}", e))?;
        connection
            .set(format!("job_return:{}", id), &job_return_json)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<JobReturn>, String> {
        let mut connection = self.create_connection()?;
        let mut job_returns: Vec<JobReturn> = Vec::new();

        // Loop over job_return:<jid>:*
        let mut keys: Vec<String> = connection
            .keys(format!("job_return:{}:*", job.id))
            .map_err(|e| format!("{:?}", e))?;
        keys.sort();

        for key in keys {
            let job_return_json: String = connection
                .get(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let job_return: JobReturn =
                serde_json::from_str(job_return_json.as_str()).map_err(|e| format!("{:?}", e))?;
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
        let mut connection = self.create_connection()?;
        let id = id.unwrap_or(format!("pg_{}", uuid::Uuid::new_v4()));
        let permission_group = PermissionGroup {
            id: id.clone(),
            name: name.to_owned(),
            perms: perms.unwrap_or("[]".to_string()),
        };

        let permission_group_json =
            serde_json::to_string(&permission_group).map_err(|e| format!("{:?}", e))?;
        connection
            .set(format!("permission_group:{}", id), &permission_group_json)
            .map_err(|e| format!("{:?}", e))?;
        Ok(id)
    }

    fn list_permission_groups(&self, paginate: Paginate) -> Result<Vec<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        let mut permission_groups: Vec<PermissionGroup> = Vec::new();

        // Loop over permission_group:*, which are HashMaps
        let mut keys: Vec<String> = connection
            .keys("permission_group:*")
            .map_err(|e| format!("{:?}", e))?;
        keys.sort();

        // Pagination
        if let Some((limit, offset)) = paginate {
            keys = keys
                .into_iter()
                .skip(offset as usize)
                .take(limit as usize)
                .collect();
        }

        for key in keys {
            let permission_group_json: String = connection
                .get(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let permission_group: PermissionGroup =
                serde_json::from_str(permission_group_json.as_str())
                    .map_err(|e| format!("{:?}", e))?;
            permission_groups.push(permission_group);
        }

        Ok(permission_groups)
    }

    fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        // If permission_group doesn't exist, return None
        if !connection
            .exists(format!("permission_group:{}", id).as_str())
            .map_err(|e| format!("{:?}", e))?
        {
            return Ok(None);
        }

        let permission_group_json: String = connection
            .get(format!("permission_group:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        let permission_group: PermissionGroup =
            serde_json::from_str(permission_group_json.as_str()).map_err(|e| format!("{:?}", e))?;
        Ok(Some(permission_group))
    }

    fn is_user_member_of_group(&self, user_id: &str, group_id: &str) -> Result<bool, String> {
        let mut connection = self.create_connection()?;

        // Search for existance of permission_group_user:<user_id>:<group_id>
        let key = format!("permission_group_user:{}:{}", user_id, group_id);
        let exists: bool = connection
            .exists(key.as_str())
            .map_err(|e| format!("{:?}", e))?;

        Ok(exists)
    }

    fn update_permission_group(&self, permission_group: &PermissionGroup) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let permission_group_json =
            serde_json::to_string(permission_group).map_err(|e| format!("{:?}", e))?;
        connection
            .set(
                format!("permission_group:{}", permission_group.id),
                &permission_group_json,
            )
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn delete_permission_group(&self, id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection
            .del(format!("permission_group:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn insert_permission_group_user(&self, user_id: &str, group_id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let key = format!("permission_group_user:{}:{}", user_id, group_id);
        connection
            .set(key.as_str(), "1")
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn list_permission_groups_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<PermissionGroup>, String> {
        let mut conn_iter = self.create_connection()?;
        let mut conn_lookup = self.create_connection()?;
        let mut permission_groups: Vec<PermissionGroup> = Vec::new();

        // Search for Hashmap where permission_group_user:<user_id>:* exists
        let keys: Iter<'_, String> = conn_iter
            .scan_match(format!("permission_group_user:{}:*", user_id).as_str())
            .map_err(|e| format!("{:?}", e))?;

        for key in keys {
            let permission_group_json: String = conn_lookup
                .get(format!(
                    "permission_group:{}",
                    key.split(':').last().unwrap()
                ))
                .map_err(|e| format!("{:?}", e))?;
            let permission_group: PermissionGroup =
                serde_json::from_str(permission_group_json.as_str())
                    .map_err(|e| format!("{:?}", e))?;
            permission_groups.push(permission_group);
        }

        Ok(permission_groups)
    }

    fn list_users_by_permission_group_id(&self, group_id: &str) -> Result<Vec<User>, String> {
        let mut conn_iter = self.create_connection()?;
        let mut conn_lookup = self.create_connection()?;
        let mut users: Vec<User> = Vec::new();

        // Search for Hashmap where permission_group_user:*:<group_id> exists
        let keys: Iter<'_, String> = conn_iter
            .scan_match(format!("permission_group_user:*:{}", group_id).as_str())
            .map_err(|e| format!("{:?}", e))?;

        for key in keys {
            let user_json: String = conn_lookup
                .get(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let user: User =
                serde_json::from_str(user_json.as_str()).map_err(|e| format!("{:?}", e))?;
            users.push(user);
        }

        Ok(users)
    }

    fn delete_permission_group_user(&self, user_id: &str, group_id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let key = format!("permission_group_user:{}:{}", user_id, group_id);
        connection
            .del(key.as_str())
            .map_err(|e| format!("{:?}", e))?;
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
        let mut connection = self.create_connection()?;
        let id = id.unwrap_or(format!("pre_{}", uuid::Uuid::new_v4()));
        let minion_preset = MinionPreset {
            id: id.clone(),
            name: name.to_string(),
            filter: filter.to_string(),
        };

        let minion_preset_json =
            serde_json::to_string(&minion_preset).map_err(|e| format!("{:?}", e))?;
        connection
            .set(format!("minion_preset:{}", id), &minion_preset_json)
            .map_err(|e| format!("{:?}", e))?;

        Ok(id)
    }

    fn list_minion_presets(&self) -> Result<Vec<MinionPreset>, String> {
        let mut connection = self.create_connection()?;
        let mut minion_presets: Vec<MinionPreset> = Vec::new();

        // Loop over minion_preset:*, which are HashMaps
        let mut keys: Vec<String> = connection
            .keys("minion_preset:*")
            .map_err(|e| format!("{:?}", e))?;
        keys.sort();

        for key in keys {
            let minion_preset_json: String = connection
                .get(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let minion_preset: MinionPreset = serde_json::from_str(minion_preset_json.as_str())
                .map_err(|e| format!("{:?}", e))?;
            minion_presets.push(minion_preset);
        }

        Ok(minion_presets)
    }

    fn get_minion_preset_by_id(&self, id: &str) -> Result<Option<MinionPreset>, String> {
        let mut connection = self.create_connection()?;
        // If minion_preset doesn't exist, return None
        if !connection
            .exists(format!("minion_preset:{}", id).as_str())
            .map_err(|e| format!("{:?}", e))?
        {
            return Ok(None);
        }

        let minion_preset_json: String = connection
            .get(format!("minion_preset:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        let minion_preset: MinionPreset =
            serde_json::from_str(minion_preset_json.as_str()).map_err(|e| format!("{:?}", e))?;
        Ok(Some(minion_preset))
    }

    fn update_minion_preset(&self, minion_preset: &MinionPreset) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let minion_preset_json =
            serde_json::to_string(minion_preset).map_err(|e| format!("{:?}", e))?;
        connection
            .set(
                format!("minion_preset:{}", minion_preset.id),
                &minion_preset_json,
            )
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn delete_minion_preset(&self, id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        connection
            .del(format!("minion_preset:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }
}
