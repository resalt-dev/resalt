extern crate diesel;

use self::diesel::prelude::*;
use crate::{diesel_migrations::MigrationHarness, schema::*, *};
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::EmbeddedMigrations;
use log::*;
use rand::Rng;
use resalt_config::SConfig;
use resalt_models::*;
use resalt_storage::{StorageImpl, StorageStatus};
use serde_json::{json, Value};

type DbPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run_pending_migrations`. This allows the code
// to be run and tested without any outside setup of the database.
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Clone)]
pub struct StorageMySQL {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl StorageMySQL {
    pub async fn connect(database_url: &str) -> Result<Self, String> {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder().build(manager);

        match pool {
            Ok(pool) => {
                let own = Self { pool };
                let mut connection = own.create_connection()?;

                match connection.run_pending_migrations(MIGRATIONS) {
                    Ok(_) => {
                        info!("Successfully ran all pending migration tasks.");
                    }
                    Err(e) => {
                        error!("Failed to run database migrations: {:?}", e);
                        return Err(format!("{:?}", e));
                    }
                };

                own.init();

                Ok(own)
            }
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    fn init(&self) {
        // Create default user
        if self.get_user_by_username("admin").unwrap().is_none() {
            // Generate random password instead of using default
            let random_password = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(15)
                .map(|c| c.to_string())
                .collect::<String>();

            // Create initial admin user
            let mut user = self
                .create_user(
                    "admin".to_string(),
                    Some(random_password.to_string()),
                    None,
                    None,
                )
                .unwrap();

            // Give permissions to admmin
            let mut perms: Value = json!([
                ".*".to_string(),
                "@runner".to_string(),
                "@wheel".to_string(),
            ]);
            // Add object permission. The array is of both strings and objects...
            perms.as_array_mut().unwrap().push(json!({
                "@resalt": [
                    "admin.superadmin".to_string(),
                ],
            }));
            user.perms = serde_json::to_string(&perms).unwrap();
            self.update_user(&user).unwrap();

            // Announce randomly generated password
            warn!("============================================================");
            warn!(
                "==  CREATED DEFAULT USER: admin WITH PASSWORD: {}  ==",
                random_password
            );
            warn!("============================================================");
        }
        // Create default permission group
        if self
            .get_permission_group_by_name("$superadmins")
            .unwrap()
            .is_none()
        {
            self.create_permission_group("$superadmins").unwrap();
            let mut group = self
                .get_permission_group_by_name("$superadmins")
                .unwrap()
                .unwrap();
            group.perms = json!([
                ".*".to_string(),
                "@runner".to_string(),
                "@wheel".to_string(),
                {
                    "@resalt": [
                        "admin.superadmin".to_string(),
                    ]
                }
            ])
            .to_string();
            self.update_permission_group(&group).unwrap();
        }
        // Add admin to $superadmins if not member
        let superadmins_group_id = self
            .get_permission_group_by_name("$superadmins")
            .unwrap()
            .unwrap()
            .id;
        let admin_user_id = self.get_user_by_username("admin").unwrap().unwrap().id;
        if !self
            .is_user_member_of_group(&admin_user_id, &superadmins_group_id)
            .unwrap()
        {
            self.insert_permission_group_user(&admin_user_id, &superadmins_group_id)
                .unwrap();
        }
    }

    fn create_connection(&self) -> Result<DbPooledConnection, String> {
        match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

impl StorageImpl for StorageMySQL {
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

        let auth_tokens_total = authtokens::table
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        let auth_tokens_active = authtokens::table
            .filter(authtokens::timestamp.ge(auth_expiry))
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        // let events_total = events::table
        //     .count()
        //     .get_result::<i64>(&mut connection)
        //     .map_err(|e| format!("{:?}", e))?;
        let events_total = -1;
        let job_returns_total = job_returns::table
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        let jobs_total = jobs::table
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;

        // last_updated_conformity != null, and conformity_error = 0 and conformity_incorrect = 0
        let minions_success = minions::table
            .filter(minions::last_updated_conformity.is_not_null())
            .filter(minions::conformity_error.eq(0))
            .filter(minions::conformity_incorrect.eq(0))
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        let minions_incorrect = minions::table
            .filter(minions::last_updated_conformity.is_not_null())
            .filter(minions::conformity_error.eq(0))
            .filter(minions::conformity_incorrect.ne(0))
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        let minions_error = minions::table
            .filter(minions::last_updated_conformity.is_not_null())
            .filter(minions::conformity_error.ne(0))
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        let minions_unknown = minions::table
            .filter(minions::last_updated_conformity.is_null())
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        let minions_total = minions_success + minions_incorrect + minions_error + minions_unknown;

        let permission_group_users_total = permission_group_users::table
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        let permission_groups_total = permission_groups::table
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        let users_total = users::table
            .count()
            .get_result::<i64>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(StorageStatus {
            auth_tokens_total,
            auth_tokens_active,
            events_total,
            job_returns_total,
            jobs_total,
            minions_total,
            minions_success,
            minions_incorrect,
            minions_error,
            minions_unknown,
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
        let user = SQLUser {
            id,
            username,
            password: password.map(|v| resalt_security::hash_password(&v)),
            perms: "[]".to_string(),
            last_login: None,
            email,
            ldap_sync,
        };

        diesel::insert_into(users::table)
            .values(&user)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;

        Ok(user.into())
    }

    fn list_users(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, String> {
        let mut connection = self.create_connection()?;
        let mut query = users::table.into_boxed();
        query = query.order(users::id.asc());

        // Filtering

        // Pagination
        query = query.limit(limit.unwrap_or(100));
        query = query.offset(offset.unwrap_or(0));

        // Query
        query
            .load::<SQLUser>(&mut connection)
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }

    fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let mut connection = self.create_connection()?;
        users::table
            .find(id)
            .first::<SQLUser>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.map(|v| v.into()))
    }

    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let mut connection = self.create_connection()?;
        users::table
            .filter(users::username.eq(username))
            .first::<SQLUser>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.map(|v| v.into()))
    }

    fn update_user(&self, user: &User) -> Result<(), String> {
        let user: SQLUser = user.clone().into();
        let mut connection = self.create_connection()?;
        diesel::update(users::table.find(&user.id))
            .set(&user)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))
            .map(|_| ())
    }

    fn delete_user(&self, id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        diesel::delete(users::table.find(id))
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))
            .map(|_| ())
    }

    ///////////////////
    /// Auth tokens ///
    ///////////////////

    fn create_authtoken(&self, user_id: String) -> Result<AuthToken, String> {
        let mut connection = self.create_connection()?;
        let id = format!("auth_{}", uuid::Uuid::new_v4());
        let authtoken = SQLAuthToken {
            id,
            user_id: user_id.clone(),
            timestamp: chrono::Utc::now().naive_utc(),
            salt_token: None,
        };

        // Insert auth token
        diesel::insert_into(authtokens::table)
            .values(&authtoken)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;

        // Update user's last_login
        diesel::update(users::table.filter(users::id.eq(&user_id)))
            .set(users::last_login.eq(&authtoken.timestamp))
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;

        Ok(authtoken.into())
    }

    fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        let mut connection = self.create_connection()?;
        authtokens::table
            .find(id)
            .first::<SQLAuthToken>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
            .map(|v| v.map(|v| v.into()))
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
        diesel::update(authtokens::table)
            .filter(authtokens::id.eq(auth_token))
            .set(authtokens::salt_token.eq(salt_token_str))
            .execute(&mut connection)
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
        let mut query = minions::table.into_boxed();
        query = query.order(minions::id.asc());
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        // Filtering
        let mut has_grain_filters = false;
        let mut has_package_filters = false;
        for filter in &filters {
            match filter.field_type {
                FilterFieldType::None => {}
                FilterFieldType::Object => match filter.field.as_str() {
                    "id" => match filter.operand {
                        FilterOperand::Contains => {
                            query = query.filter(minions::id.like(format!("%{}%", filter.value)))
                        }
                        FilterOperand::NotContains => {
                            query =
                                query.filter(minions::id.not_like(format!("%{}%", filter.value)))
                        }
                        FilterOperand::Equals => {
                            query = query.filter(minions::id.eq(filter.value.clone()))
                        }
                        FilterOperand::NotEquals => {
                            query = query.filter(minions::id.ne(filter.value.clone()))
                        }
                        FilterOperand::StartsWith => {
                            query = query.filter(minions::id.like(format!("{}%", filter.value)))
                        }
                        FilterOperand::EndsWith => {
                            query = query.filter(minions::id.like(format!("%{}", filter.value)))
                        }
                        FilterOperand::GreaterThanOrEqual => {
                            query = query.filter(minions::id.ge(filter.value.clone()))
                        }
                        FilterOperand::LessThanOrEqual => {
                            query = query.filter(minions::id.le(filter.value.clone()))
                        }
                    },
                    "os_type" => match filter.operand {
                        FilterOperand::Contains => {
                            query =
                                query.filter(minions::os_type.like(format!("%{}%", filter.value)))
                        }
                        FilterOperand::NotContains => {
                            query = query
                                .filter(minions::os_type.not_like(format!("%{}%", filter.value)))
                        }
                        FilterOperand::Equals => {
                            query = query.filter(minions::os_type.eq(filter.value.clone()))
                        }
                        FilterOperand::NotEquals => {
                            query = query.filter(minions::os_type.ne(filter.value.clone()))
                        }
                        FilterOperand::StartsWith => {
                            query =
                                query.filter(minions::os_type.like(format!("{}%", filter.value)))
                        }
                        FilterOperand::EndsWith => {
                            query =
                                query.filter(minions::os_type.like(format!("%{}", filter.value)))
                        }
                        FilterOperand::GreaterThanOrEqual => {
                            query = query.filter(minions::os_type.ge(filter.value.clone()))
                        }
                        FilterOperand::LessThanOrEqual => {
                            query = query.filter(minions::os_type.le(filter.value.clone()))
                        }
                    },
                    "last_seen" => {
                        let timestamp: chrono::NaiveDateTime =
                            chrono::NaiveDateTime::parse_from_str(
                                &filter.value,
                                "%Y-%m-%d %H:%M:%S",
                            )
                            .map_err(|e| format!("{:?}", e))?;

                        // Contains, not contains, starts with, and end withds does not exist for Timestamp.
                        match filter.operand {
                            FilterOperand::Equals => {
                                query = query.filter(minions::last_seen.eq(timestamp))
                            }
                            FilterOperand::NotEquals => {
                                query = query.filter(minions::last_seen.ne(timestamp))
                            }
                            FilterOperand::GreaterThanOrEqual => {
                                query = query.filter(minions::last_seen.ge(timestamp))
                            }
                            FilterOperand::LessThanOrEqual => {
                                query = query.filter(minions::last_seen.le(timestamp))
                            }
                            FilterOperand::Contains
                            | FilterOperand::NotContains
                            | FilterOperand::StartsWith
                            | FilterOperand::EndsWith => {
                                return Err("Invalid OBJECT last_seen filter operand".to_string())
                            }
                        }
                    }
                    "conformity_success" => {
                        let number: &str = if filter.value.is_empty() {
                            "0"
                        } else {
                            &filter.value
                        };
                        let number = number.parse::<i32>().map_err(|e| {
                            format!("Invalid OBJECT conformity_success filter value: {:?}", e)
                        })?;

                        // Contains, not contains, starts with, and end withds does not exist for i32.
                        match filter.operand {
                            FilterOperand::Equals => {
                                query = query.filter(minions::conformity_success.eq(number))
                            }
                            FilterOperand::NotEquals => {
                                query = query.filter(minions::conformity_success.ne(number))
                            }
                            FilterOperand::GreaterThanOrEqual => {
                                query = query.filter(minions::conformity_success.ge(number))
                            }
                            FilterOperand::LessThanOrEqual => {
                                query = query.filter(minions::conformity_success.le(number))
                            }
                            FilterOperand::Contains
                            | FilterOperand::NotContains
                            | FilterOperand::StartsWith
                            | FilterOperand::EndsWith => {
                                return Err(
                                    "Invalid OBJECT conformity_success filter operand".to_string()
                                )
                            }
                        }
                    }
                    "conformity_incorrect" => {
                        let number: &str = if filter.value.is_empty() {
                            "0"
                        } else {
                            &filter.value
                        };
                        let number = number.parse::<i32>().map_err(|e| {
                            format!("Invalid OBJECT conformity_incorrect filter value: {:?}", e)
                        })?;

                        // Contains, not contains, starts with, and end withds does not exist for i32.
                        match filter.operand {
                            FilterOperand::Equals => {
                                query = query.filter(minions::conformity_incorrect.eq(number))
                            }
                            FilterOperand::NotEquals => {
                                query = query.filter(minions::conformity_incorrect.ne(number))
                            }
                            FilterOperand::GreaterThanOrEqual => {
                                query = query.filter(minions::conformity_incorrect.ge(number))
                            }
                            FilterOperand::LessThanOrEqual => {
                                query = query.filter(minions::conformity_incorrect.le(number))
                            }
                            FilterOperand::Contains
                            | FilterOperand::NotContains
                            | FilterOperand::StartsWith
                            | FilterOperand::EndsWith => {
                                return Err("Invalid OBJECT conformity_incorrect filter operand"
                                    .to_string())
                            }
                        }
                    }
                    "conformity_error" => {
                        let number: &str = if filter.value.is_empty() {
                            "0"
                        } else {
                            &filter.value
                        };
                        let number = number.parse::<i32>().map_err(|e| {
                            format!("Invalid OBJECT conformity_error filter value: {:?}", e)
                        })?;

                        // Contains, not contains, starts with, and end withds does not exist for i32.
                        match filter.operand {
                            FilterOperand::Equals => {
                                query = query.filter(minions::conformity_error.eq(number))
                            }
                            FilterOperand::NotEquals => {
                                query = query.filter(minions::conformity_error.ne(number))
                            }
                            FilterOperand::GreaterThanOrEqual => {
                                query = query.filter(minions::conformity_error.ge(number))
                            }
                            FilterOperand::LessThanOrEqual => {
                                query = query.filter(minions::conformity_error.le(number))
                            }
                            FilterOperand::Contains
                            | FilterOperand::NotContains
                            | FilterOperand::StartsWith
                            | FilterOperand::EndsWith => {
                                return Err(
                                    "Invalid OBJECT conformity_error filter operand".to_string()
                                )
                            }
                        }
                    }
                    _ => {
                        log::warn!("Unknown OBJECT filter field: {}", filter.field);
                    }
                },
                // Grains are JSON-encoded, so we need to use the JSON operators. The "filter.field" is a valid "JSONPath".
                // Use the JSONPath to get the value out of the minion.grains JSON field, and filter the value on filter.value.
                FilterFieldType::Grain => has_grain_filters = true,
                FilterFieldType::Package => has_package_filters = true,
            }
        }

        // Sorting
        match sort.unwrap_or_else(|| "id.asc".to_string()).as_str() {
            "id.asc" => query = query.order(minions::id.asc()),
            "id.desc" => query = query.order(minions::id.desc()),
            "lastSeen.asc" => query = query.order(minions::last_seen.asc()),
            "lastSeen.desc" => query = query.order(minions::last_seen.desc()),
            "conformitySuccess.asc" => query = query.order(minions::conformity_success.asc()),
            "conformitySuccess.desc" => query = query.order(minions::conformity_success.desc()),
            "conformityIncorrect.asc" => query = query.order(minions::conformity_incorrect.asc()),
            "conformityIncorrect.desc" => query = query.order(minions::conformity_incorrect.desc()),
            "conformityError.asc" => query = query.order(minions::conformity_error.asc()),
            "conformityError.desc" => query = query.order(minions::conformity_error.desc()),
            "osType.asc" => query = query.order(minions::os_type.asc()),
            "osType.desc" => query = query.order(minions::os_type.desc()),
            _ => return Err(String::from("Invalid sort parameter")),
        }

        // Pagination
        if !has_grain_filters && !has_package_filters {
            if limit > 0 {
                query = query.limit(limit);
            }
            if offset > 0 {
                query = query.offset(offset);
            }
        }

        let mut minions: Vec<Minion> = query
            .load::<SQLMinion>(&mut connection)
            .map_err(|e| format!("{:?}", e))
            .map(|sql_minions| {
                sql_minions
                    .into_iter()
                    .map(|sql_minion| sql_minion.into())
                    .collect()
            })?;

        // 2nd stage JSON Filtering
        // Filter on grains
        if has_grain_filters {
            resalt_storage::filter_minions_on_grains(&mut minions, &filters);
        }
        if has_package_filters {
            resalt_storage::filter_minions_on_packages(&mut minions, &filters);
        }

        // 2nd Limit if the first limit didn't kick in
        if has_grain_filters || has_package_filters {
            let offset = offset as usize;
            let limit = limit as usize;
            minions = minions.into_iter().skip(offset).take(limit).collect();
        }

        Ok(minions)
    }

    fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String> {
        let mut connection = self.create_connection()?;
        minions::table
            .filter(minions::id.eq(id))
            .first::<SQLMinion>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
            .map(|sql_minion| sql_minion.map(|sql_minion| sql_minion.into()))
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

        let changeset: SQLMinion = Minion {
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

        // Update if it exists, insert if it doesn't

        let result = diesel::update(minions::table)
            .filter(minions::id.eq(&minion_id))
            .set(&changeset)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;

        if result == 0 {
            match diesel::insert_into(minions::table)
                .values(&changeset)
                .execute(&mut connection)
            {
                Ok(_) => {}
                Err(e) => {
                    match e {
                        diesel::result::Error::DatabaseError(
                            diesel::result::DatabaseErrorKind::UniqueViolation,
                            _,
                        ) => {
                            // Maybe it was inserted between we checked, so lets try update with our info again
                            diesel::update(minions::table)
                                .filter(minions::id.eq(minion_id))
                                .set(&changeset)
                                .execute(&mut connection)
                                .map_err(|e| format!("{:?}", e))?;
                        }
                        e => return Err(format!("{:?}", e)),
                    }
                }
            }
        }

        Ok(())
    }

    // Delete minions not in the list of ID's
    fn prune_minions(&self, ids: Vec<String>) -> Result<(), String> {
        let mut connection = self.create_connection()?;

        diesel::delete(minions::table.filter(minions::id.ne_all(ids)))
            .execute(&mut connection)
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
