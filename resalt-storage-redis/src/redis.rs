use r2d2::{Pool, PooledConnection};
use redis::{Client, Commands, Iter};
use resalt_models::*;
use resalt_storage::StorageImpl;

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
        };

        let values = user.hash();

        connection
            .hset_multiple(format!("user:{}", id), values.as_slice())
            .map_err(|e| format!("{:?}", e))?;

        Ok(user)
    }

    fn list_users(&self, paginate: Paginate) -> Result<Vec<User>, String> {
        let mut connection = self.create_connection()?;
        let mut users: Vec<User> = Vec::new();

        // Loop over user:*, which are HashMaps
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
            let id: String = key.split(':').last().unwrap().to_string();
            // Fields are stored as HashMap
            let values = connection
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let user: User = User::dehash(id, values);
            users.push(user);
        }

        Ok(users)
    }

    fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let mut connection = self.create_connection()?;
        // If user doesn't exist, return None
        let values: Vec<(String, String)> = connection
            .hgetall(format!("user:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        if values.is_empty() {
            return Ok(None);
        }

        let user: User = User::dehash(id.to_string(), values);

        Ok(Some(user))
    }

    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let mut conn_iter = self.create_connection()?;
        let mut conn_lookup = self.create_connection()?;
        // If user doesn't exist, return None

        // Search for Hashmap where user:*.username == username
        let keys: Iter<'_, String> = conn_iter
            .scan_match("user:*")
            .map_err(|e| format!("{:?}", e))?;

        for key in keys {
            let id: String = key.split(':').last().unwrap().to_string();
            let values: Vec<(String, String)> = conn_lookup
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            if values.is_empty() {
                continue;
            }

            let user: User = User::dehash(id, values);

            if user.username == username {
                return Ok(Some(user));
            }
        }

        Ok(None)
    }

    fn update_user(&self, user: &User) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let values = user.hash();
        connection
            .hset_multiple(format!("user:{}", user.id), values.as_slice())
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
            id,
            user_id: user_id.clone(),
            timestamp: ResaltTime::now(),
            salt_token: None,
        };

        let values = authtoken.hash();

        // Insert auth token
        connection
            .hset_multiple(format!("authtoken:{}", authtoken.id), values.as_slice())
            .map_err(|e| format!("{:?}", e))?;

        // Update user's last_login
        connection
            .hset(
                format!("user:{}", user_id),
                "last_login",
                authtoken.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            )
            .map_err(|e| format!("{:?}", e))?;

        Ok(authtoken)
    }

    fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        let mut connection = self.create_connection()?;
        // If authtoken doesn't exist, return None
        let values: Vec<(String, String)> = connection
            .hgetall(format!("authtoken:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        if values.is_empty() {
            return Ok(None);
        }

        let authtoken: AuthToken = AuthToken::dehash(id.to_string(), values);

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
            .hset(
                format!("authtoken:{}", auth_token),
                "salt_token",
                salt_token_str,
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
            let id: String = key.split(':').last().unwrap().to_string();
            // Fields are stored as HashMap
            let values = connection
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let minion: Minion = Minion::dehash(id, values);
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
        let values: Vec<(String, String)> = connection
            .hgetall(format!("minion:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        if values.is_empty() {
            return Ok(None);
        }

        let minion: Minion = Minion::dehash(id.to_string(), values);

        Ok(Some(minion))
    }

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
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;

        let last_updated_grains = grains.as_ref().map(|_| last_updated_grains.unwrap_or(time));
        let last_updated_pillars = pillars
            .as_ref()
            .map(|_| last_updated_pillars.unwrap_or(time));
        let last_updated_pkgs = pkgs.as_ref().map(|_| last_updated_pkgs.unwrap_or(time));
        let last_updated_conformity = conformity
            .as_ref()
            .map(|_| last_updated_conformity.unwrap_or(time));

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
            last_seen: time,
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

        let values = minion.hash();

        // Update if it exists, insert if it doesn't
        connection
            .hset_multiple(format!("minion:{}", minion_id), values.as_slice())
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    // Delete minion
    fn delete_minion(&self, id: String) -> Result<(), String> {
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

        let values = event.hash();

        connection
            .hset_multiple(format!("event:{}", id), values.as_slice())
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
            let id: String = key.split(':').last().unwrap().to_string();
            // Fields are stored as HashMap
            let values = connection
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let event: Event = Event::dehash(id, values);
            events.push(event);
        }

        Ok(events)
    }

    fn get_event_by_id(&self, id: &str) -> Result<Option<Event>, String> {
        let mut connection = self.create_connection()?;
        // If event doesn't exist, return None
        let values: Vec<(String, String)> = connection
            .hgetall(format!("event:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        if values.is_empty() {
            return Ok(None);
        }

        let event: Event = Event::dehash(id.to_string(), values);

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

        let values = job.hash();

        connection
            .hset_multiple(format!("job:{}", jid), values.as_slice())
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
            let id: String = key.split(':').last().unwrap().to_string();
            // Fields are stored as HashMap
            let values = connection
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let job: Job = Job::dehash(id, values);
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
        let values: Vec<(String, String)> = connection
            .hgetall(format!("job:{}", jid))
            .map_err(|e| format!("{:?}", e))?;
        if values.is_empty() {
            return Ok(None);
        }

        let job: Job = Job::dehash(jid.to_string(), values);

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

        let values = job_return.hash();

        connection
            .hset_multiple(format!("job_return:{}:{}", jid, id), values.as_slice())
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
            let id: String = key.split(':').last().unwrap().to_string();
            // Fields are stored as HashMap
            let values = connection
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let job_return: JobReturn = JobReturn::dehash(id, values);
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

        let values = permission_group.hash();

        connection
            .hset_multiple(format!("permission_group:{}", id), values.as_slice())
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
            let id: String = key.split(':').last().unwrap().to_string();
            // Fields are stored as HashMap
            let values = connection
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let permission_group: PermissionGroup = PermissionGroup::dehash(id, values);
            permission_groups.push(permission_group);
        }

        Ok(permission_groups)
    }

    fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        // If permission_group doesn't exist, return None
        let values: Vec<(String, String)> = connection
            .hgetall(format!("permission_group:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        if values.is_empty() {
            return Ok(None);
        }

        let permission_group: PermissionGroup = PermissionGroup::dehash(id.to_string(), values);

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
        let values = permission_group.hash();
        connection
            .hset_multiple(
                format!("permission_group:{}", permission_group.id),
                values.as_slice(),
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
            let group_id: String = key.split(':').last().unwrap().to_string();
            let values: Vec<(String, String)> = conn_lookup
                .hgetall(format!("permission_group:{}", group_id))
                .map_err(|e| format!("{:?}", e))?;
            if values.is_empty() {
                continue;
            }

            let permission_group: PermissionGroup = PermissionGroup::dehash(group_id, values);
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
            let user_id: String = key.split(':').nth(1).unwrap().to_string();
            let values: Vec<(String, String)> = conn_lookup
                .hgetall(format!("user:{}", user_id))
                .map_err(|e| format!("{:?}", e))?;
            if values.is_empty() {
                continue;
            }

            let user: User = User::dehash(user_id, values);
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

        let values = minion_preset.hash();

        connection
            .hset_multiple(format!("minion_preset:{}", id), values.as_slice())
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
            let id: String = key.split(':').last().unwrap().to_string();
            // Fields are stored as HashMap
            let values = connection
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let minion_preset: MinionPreset = MinionPreset::dehash(id, values);
            minion_presets.push(minion_preset);
        }

        Ok(minion_presets)
    }

    fn get_minion_preset_by_id(&self, id: &str) -> Result<Option<MinionPreset>, String> {
        let mut connection = self.create_connection()?;
        // If minion_preset doesn't exist, return None
        let values: Vec<(String, String)> = connection
            .hgetall(format!("minion_preset:{}", id))
            .map_err(|e| format!("{:?}", e))?;
        if values.is_empty() {
            return Ok(None);
        }

        let minion_preset: MinionPreset = MinionPreset::dehash(id.to_string(), values);

        Ok(Some(minion_preset))
    }

    fn update_minion_preset(&self, minion_preset: &MinionPreset) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let values = minion_preset.hash();
        connection
            .hset_multiple(
                format!("minion_preset:{}", minion_preset.id),
                values.as_slice(),
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
