extern crate r2d2_redis;

use chrono::NaiveDateTime;
use chrono::Utc;
use r2d2_redis::r2d2::Pool;
use r2d2_redis::r2d2::PooledConnection;
use r2d2_redis::redis::Commands;
use r2d2_redis::redis::ConnectionLike;
use r2d2_redis::redis::Iter;
use r2d2_redis::{r2d2, RedisConnectionManager};
use resalt_config::SConfig;
use resalt_models::*;
use resalt_storage::{StorageImpl, StorageStatus};

#[derive(Clone)]
pub struct StorageRedis {
    pool: Pool<RedisConnectionManager>,
}

impl StorageRedis {
    pub async fn connect(database_url: &str) -> Result<Self, String> {
        let manager = RedisConnectionManager::new(database_url).unwrap();
        let pool = r2d2::Pool::builder().build(manager);

        match pool {
            Ok(pool) => {
                let own = Self { pool };
                own.init();
                Ok(own)
            }
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    fn create_connection(&self) -> Result<PooledConnection<RedisConnectionManager>, String> {
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

    fn get_status(&self) -> Result<resalt_storage::StorageStatus, String> {
        let mut connection = self.create_connection()?;

        let lifespan = SConfig::auth_session_lifespan() * 1000;
        let auth_expiry: NaiveDateTime = match NaiveDateTime::from_timestamp_millis(
            Utc::now().timestamp_millis() - (lifespan as i64),
        ) {
            Some(dt) => dt,
            None => return Err("Failed to convert timestamp to NaiveDateTime: {:?}".to_string()),
        };

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

    fn create_user(
        &self,
        username: String,
        password: Option<String>,
        email: Option<String>,
        ldap_sync: Option<String>,
    ) -> Result<User, String> {
        let mut connection = self.create_connection()?;
        let id = format!("usr_{}", uuid::Uuid::new_v4());
        let user = User {
            id: id.clone(),
            username: username.clone(),
            // TODO: // Move hashing out of connection-specific class
            password: password.map(|v| resalt_security::hash_password(&v)),
            perms: "[]".to_string(),
            last_login: None,
            email: email.clone(),
            ldap_sync: ldap_sync.clone(),
        };

        let values = user.hash();

        connection
            .hset_multiple(format!("user:{}", id), values.as_slice())
            .map_err(|e| format!("{:?}", e))?;

        Ok(user)
    }

    fn list_users(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, String> {
        let mut connection = self.create_connection()?;
        let mut users: Vec<User> = Vec::new();

        // Loop over user:*, which are HashMaps
        let mut keys: Vec<String> = connection.keys("user:*").map_err(|e| format!("{:?}", e))?;
        keys.sort();

        // Skip offset & Limit
        keys = keys
            .into_iter()
            .skip(offset.unwrap_or(0) as usize)
            .take(limit.unwrap_or(100) as usize)
            .collect();

        for key in keys {
            // Fields are stored as HashMap
            let values = connection
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let user: User = User::dehash(values);
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

        let user: User = User::dehash(values);

        Ok(Some(user))
    }

    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let mut connection = self.create_connection()?;
        // If user doesn't exist, return None

        // Search for Hashmap where user:*.username == username
        let keys: Iter<'_, String> = connection
            .scan_match("user:*")
            .map_err(|e| format!("{:?}", e))?;

        for key in keys {
            let values: Vec<(String, String)> = connection
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            if values.is_empty() {
                continue;
            }

            let user: User = User::dehash(values);

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
            timestamp: chrono::Utc::now().naive_utc(),
            salt_token: None,
        };

        let values = authtoken.hash();

        // Insert auth token
        connection
            .hset_multiple(format!("authtoken:{}", id), values.as_slice())
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

        let authtoken: AuthToken = AuthToken::dehash(values);

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
        sort: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Minion>, String> {
        let mut connection = self.create_connection()?;
        let mut minions: Vec<Minion> = Vec::new();

        // Loop over minion:*, which are HashMaps
        let mut keys: Vec<String> = connection
            .keys("minion:*")
            .map_err(|e| format!("{:?}", e))?;
        keys.sort();

        // QUICK PAGINATION (Skip offset & Limit)
        if (filters.len() == 0) {
            keys = keys
                .into_iter()
                .skip(offset.unwrap_or(0) as usize)
                .take(limit.unwrap_or(100) as usize)
                .collect();
        }

        for key in keys {
            // Fields are stored as HashMap
            let values = connection
                .hgetall(key.as_str())
                .map_err(|e| format!("{:?}", e))?;
            let minion: Minion = Minion::dehash(values);
            minions.push(minion);
        }

        // Filtering
        resalt_storage::filter_minions(&mut minions, &filters);

        // Sorting
        let sort = sort.unwrap_or("id.asc".to_string());
        resalt_storage::sort_minions(&mut minions, &sort);

        // SLOW PAGINATION (Skip offset & Limit)
        if filters.len() != 0 {
            let offset = offset.unwrap_or(0) as usize;
            let limit = limit.unwrap_or(100) as usize;
            minions = minions.into_iter().skip(offset).take(limit).collect();
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

        let minion: Minion = Minion::dehash(values);

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
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;

        let last_updated_grains = grains.as_ref().map(|_| time);
        let last_updated_pillars = pillars.as_ref().map(|_| time);
        let last_updated_pkgs = pkgs.as_ref().map(|_| time);
        let last_updated_conformity = conformity.as_ref().map(|_| time);

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
        }
        .into();

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
        timestamp: chrono::NaiveDateTime,
    ) -> Result<String, String> {
        let mut connection = self.create_connection()?;
        let id = format!("evnt_{}", uuid::Uuid::new_v4());
        let event: SQLEvent = Event {
            id: id.clone(),
            timestamp,
            tag,
            data,
        }
        .into();
        diesel::insert_into(events::table)
            .values(&event)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(id)
    }

    fn list_events(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Event>, String> {
        let mut connection = self.create_connection()?;
        let mut query = events::table.into_boxed();
        query = query.order(events::timestamp.desc());

        // Filtering

        // Pagination
        query = query.limit(limit.unwrap_or(100));
        query = query.offset(offset.unwrap_or(0));

        // Query
        query
            .load::<SQLEvent>(&mut connection)
            .map_err(|e| format!("{:?}", e))
            .map(|sql_events| {
                sql_events
                    .into_iter()
                    .map(|sql_event| sql_event.into())
                    .collect()
            })
    }

    fn insert_job(
        &self,
        jid: String,
        user: Option<String>,
        event_id: Option<String>,
        timestamp: chrono::NaiveDateTime,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let id = format!("job_{}", uuid::Uuid::new_v4());
        let job: SQLJob = Job {
            id,
            timestamp,
            jid,
            user,
            event_id,
        }
        .into();
        diesel::insert_into(jobs::table)
            .values(&job)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn list_jobs(
        &self,
        sort: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Job>, String> {
        let mut connection = self.create_connection()?;
        let mut query = jobs::table.into_boxed();
        query = query.order(jobs::timestamp.desc());

        // Filtering

        // Sorting
        match sort.unwrap_or_else(|| "id.asc".to_string()).as_str() {
            "id.asc" => query = query.order(jobs::id.asc()),
            "id.desc" => query = query.order(jobs::id.desc()),
            "timestamp.asc" => query = query.order(jobs::timestamp.asc()),
            "timestamp.desc" => query = query.order(jobs::timestamp.desc()),
            "jid.asc" => query = query.order(jobs::jid.asc()),
            "jid.desc" => query = query.order(jobs::jid.desc()),
            "user.asc" => query = query.order(jobs::user.asc()),
            "user.desc" => query = query.order(jobs::user.desc()),
            _ => return Err(String::from("Invalid sort parameter")),
        }

        // Pagination
        query = query.limit(limit.unwrap_or(100));
        query = query.offset(offset.unwrap_or(0));

        // Query
        query
            .load::<SQLJob>(&mut connection)
            .map_err(|e| format!("{:?}", e))
            .map(|sql_jobs| sql_jobs.into_iter().map(|sql_job| sql_job.into()).collect())
    }

    fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String> {
        let mut connection = self.create_connection()?;
        jobs::table
            .filter(jobs::jid.eq(jid))
            .first::<SQLJob>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
            .map(|sql_job| sql_job.map(|sql_job| sql_job.into()))
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
        let mut connection = self.create_connection()?;
        let id = format!("jret_{}", uuid::Uuid::new_v4());
        let job_return: SQLJobReturn = JobReturn {
            id,
            timestamp,
            jid,
            job_id,
            event_id,
            minion_id,
        }
        .into();
        diesel::insert_into(job_returns::table)
            .values(&job_return)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<Event>, String> {
        let mut connection = self.create_connection()?;
        events::table
            .inner_join(job_returns::table.on(events::id.eq(job_returns::event_id)))
            .filter(job_returns::job_id.eq(&job.id))
            .load::<(SQLEvent, SQLJobReturn)>(&mut connection)
            .map(|v: Vec<(SQLEvent, SQLJobReturn)>| v.into_iter().map(|(e, _)| e.into()).collect())
            .map_err(|e| format!("{:?}", e))
    }

    /////////////////////////
    /// Permission Groups ///
    /////////////////////////

    fn create_permission_group(&self, name: &str) -> Result<String, String> {
        let mut connection = self.create_connection()?;
        let id = format!("pg_{}", uuid::Uuid::new_v4());
        let permission_group: SQLPermissionGroup = PermissionGroup {
            id: id.clone(),
            name: name.to_owned(),
            perms: "[]".to_string(),
            ldap_sync: None,
        }
        .into();
        diesel::insert_into(permission_groups::table)
            .values(&permission_group)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(id)
    }

    fn list_permission_groups(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        let mut query = permission_groups::table.into_boxed();
        query = query.order(permission_groups::name.asc());

        // Filtering

        // Pagination
        query = query.limit(limit.unwrap_or(100));
        query = query.offset(offset.unwrap_or(0));

        // Query
        query
            .load::<SQLPermissionGroup>(&mut connection)
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }

    fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        permission_groups::table
            .filter(permission_groups::id.eq(id))
            .first::<SQLPermissionGroup>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.map(|v| v.into()))
    }

    fn get_permission_group_by_name(&self, name: &str) -> Result<Option<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        permission_groups::table
            .filter(permission_groups::name.eq(name))
            .first::<SQLPermissionGroup>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.map(|v| v.into()))
    }

    fn get_permission_group_by_ldap_sync(
        &self,
        ldap_sync: &str,
    ) -> Result<Option<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        permission_groups::table
            .filter(permission_groups::ldap_sync.eq(ldap_sync))
            .first::<SQLPermissionGroup>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.map(|v| v.into()))
    }

    fn is_user_member_of_group(&self, user_id: &str, group_id: &str) -> Result<bool, String> {
        let mut connection = self.create_connection()?;
        permission_group_users::table
            .filter(permission_group_users::user_id.eq(user_id))
            .filter(permission_group_users::group_id.eq(group_id))
            .first::<SQLPermissionGroupUser>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.is_some())
    }

    fn update_permission_group(&self, permission_group: &PermissionGroup) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        diesel::update(permission_groups::table)
            .filter(permission_groups::id.eq(&permission_group.id))
            // set name, perms, ldap_sync
            .set((
                permission_groups::name.eq(&permission_group.name),
                permission_groups::perms.eq(&permission_group.perms),
                permission_groups::ldap_sync.eq(&permission_group.ldap_sync),
            ))
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn delete_permission_group(&self, id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        diesel::delete(permission_groups::table)
            .filter(permission_groups::id.eq(id))
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn insert_permission_group_user(&self, user_id: &str, group_id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let id = format!("pgu_{}", uuid::Uuid::new_v4());
        let permission_group_user: SQLPermissionGroupUser = PermissionGroupUser {
            id,
            user_id: user_id.to_string(),
            group_id: group_id.to_string(),
        }
        .into();
        diesel::insert_into(permission_group_users::table)
            .values(&permission_group_user)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn list_permission_groups_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        permission_groups::table
            .inner_join(permission_group_users::table)
            .filter(permission_group_users::user_id.eq(user_id))
            .select(permission_groups::all_columns)
            .load::<SQLPermissionGroup>(&mut connection)
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }

    fn list_users_by_permission_group_id(&self, group_id: &str) -> Result<Vec<User>, String> {
        let mut connection = self.create_connection()?;
        users::table
            .inner_join(permission_group_users::table)
            .filter(permission_group_users::group_id.eq(group_id))
            .select(users::all_columns)
            .load::<SQLUser>(&mut connection)
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }

    fn delete_permission_group_user(&self, user_id: &str, group_id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        diesel::delete(permission_group_users::table)
            .filter(permission_group_users::user_id.eq(user_id))
            .filter(permission_group_users::group_id.eq(group_id))
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    //////////////////////
    /// Minion Presets ///
    //////////////////////

    fn insert_minion_preset(&self, name: &str, filter: &str) -> Result<String, String> {
        let mut connection = self.create_connection()?;
        let id = format!("pre_{}", uuid::Uuid::new_v4());
        let minion_preset: SQLMinionPreset = MinionPreset {
            id: id.clone(),
            name: name.to_string(),
            filter: filter.to_string(),
        }
        .into();
        diesel::insert_into(minion_presets::table)
            .values(&minion_preset)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(id)
    }

    fn list_minion_presets(&self) -> Result<Vec<MinionPreset>, String> {
        let mut connection = self.create_connection()?;
        let query = minion_presets::table.into_boxed();
        query
            .order(minion_presets::name.asc())
            .load::<SQLMinionPreset>(&mut connection)
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }

    fn get_minion_preset_by_id(&self, id: &str) -> Result<Option<MinionPreset>, String> {
        let mut connection = self.create_connection()?;
        minion_presets::table
            .filter(minion_presets::id.eq(id))
            .first::<SQLMinionPreset>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.map(|v| v.into()))
    }

    fn update_minion_preset(&self, minion_preset: &MinionPreset) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let minion_preset: SQLMinionPreset = minion_preset.clone().into();
        diesel::update(minion_presets::table)
            .filter(minion_presets::id.eq(&minion_preset.id))
            .set(&minion_preset)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    fn delete_minion_preset(&self, id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        diesel::delete(minion_presets::table)
            .filter(minion_presets::id.eq(id))
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }
}
