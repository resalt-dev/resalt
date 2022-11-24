extern crate diesel;

use std::collections::HashMap;

use self::diesel::prelude::*;
use crate::diesel_migrations::MigrationHarness;
use crate::{prelude::*, schema::*};
use chrono::NaiveDateTime;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::EmbeddedMigrations;
use log::{error, info, warn};
use rand::Rng;
use serde_json::{json, Value};
use version_compare::Cmp;

type DbPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run_pending_migrations`. This allows the code
// to be run and tested without any outside setup of the database.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Clone)]
pub struct Storage {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Storage {
    pub async fn connect(database_url: &str) -> Result<Self, String> {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder().build(manager);

        match pool {
            Ok(pool) => {
                let own = Self { pool };
                let mut connection = own.create_connection()?;

                match connection.run_pending_migrations(MIGRATIONS) {
                    Ok(_) => {
                        info!("Data migration successfully completed and verified.");
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
        return match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(format!("{:?}", e)),
        };
    }

    /////////////
    /// Users ///
    /////////////

    pub fn create_user(
        &self,
        username: String,
        password: Option<String>,
        email: Option<String>,
        ldap_sync: Option<String>,
    ) -> Result<User, String> {
        let mut connection = self.create_connection()?;
        let id = format!("usr_{}", uuid::Uuid::new_v4());
        let user = User {
            id,
            username,
            password: password.map(|v| hash_password(&v)),
            perms: "[]".to_string(),
            last_login: None,
            email,
            ldap_sync,
        };

        diesel::insert_into(users::table)
            .values(&user)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;

        Ok(user)
    }

    pub fn list_users(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, String> {
        let mut connection = self.create_connection()?;
        let mut query = users::table.into_boxed();
        query = query.order(users::id.asc());

        // Filtering

        // Pagination
        query = query.limit(limit.unwrap_or(100));
        query = query.offset(offset.unwrap_or(0));

        // Query
        query
            .load::<User>(&mut connection)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let mut connection = self.create_connection()?;
        users::table
            .filter(users::id.eq(id))
            .first(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let mut connection = self.create_connection()?;
        users::table
            .filter(users::username.eq(username))
            .first(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub fn update_user(&self, user: &User) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        diesel::update(users::table.filter(users::id.eq(&user.id)))
            .set(user)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    ///////////////////
    /// Auth tokens ///
    ///////////////////

    pub fn create_authtoken(&self, user_id: String) -> Result<AuthToken, String> {
        let mut connection = self.create_connection()?;
        let id = format!("auth_{}", uuid::Uuid::new_v4());
        let authtoken = AuthToken {
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

        Ok(authtoken)
    }

    pub fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        let mut connection = self.create_connection()?;
        authtokens::table
            .filter(authtokens::id.eq(id))
            .first(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub fn update_authtoken_salttoken(
        &self,
        auth_token: &str,
        salt_token: &Option<SaltToken>,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;

        let salt_token_str = salt_token
            .as_ref()
            .map(|st| serde_json::to_string(st).unwrap());

        // Update authtoken with salttoken
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

    pub fn list_minions(
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
                        let number = filter.value.parse::<i32>().map_err(|e| {
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
                        let number = filter.value.parse::<i32>().map_err(|e| {
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
                        let number = filter.value.parse::<i32>().map_err(|e| {
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
        match sort.unwrap_or(String::from("id.asc")).as_str() {
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
            query = query.limit(limit);
            query = query.offset(offset);
        }

        let mut minions: Vec<Minion> = query
            .load::<Minion>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;

        // 2nd stage JSON Filtering
        // Util
        let value_to_simple_str = |value: &Value| -> String {
            match value {
                Value::String(s) => strip_quotes!(s.to_string()),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Array(a) => a
                    .iter()
                    .map(|v| strip_quotes!(v.to_string()))
                    .collect::<Vec<String>>()
                    .join(", "),
                Value::Object(_) => String::from("<OBJECT>"),
                Value::Null => String::from("null"),
            }
        };
        // Filter on grains
        if has_grain_filters {
            // Map grain values to json paths
            // If filter.field does not start with "$.", prepend it.
            let json_paths: Vec<String> = filters
                .iter()
                .map(|f| {
                    f.field
                        .starts_with("$.")
                        .then(|| f.field.clone())
                        .unwrap_or(format!("$.{}", f.field))
                })
                .collect();

            // Filter
            minions.retain(|minion| {
                // Parse Grains from JSON
                let grains = minion.grains.clone().unwrap_or_default();
                let grains: Value = serde_json::from_str(&grains).unwrap_or_default();
                // Iterate filters with index
                for (i, filter) in filters.iter().enumerate() {
                    // Skip filters that are not of type Grain, when filtering grains
                    if filter.field_type != FilterFieldType::Grain {
                        continue;
                    }

                    let json_path = &json_paths[i];
                    let selected = match jsonpath_lib::select(&grains, json_path) {
                        Ok(selected) => selected,
                        Err(_) => {
                            log::warn!("Filtering on grain with invalid JSONPath: {}", json_path);
                            return false;
                        }
                    };

                    log::info!("Selected: {:?}", selected);

                    // Convert the selected JSON value to a string. "selected" is always a JSON array.
                    // If it is empty, return an empty string.
                    // If it contains just one object, return that, without quotes.
                    // If it contains multiple objects, join them with ", " and without each string having quotes.
                    let selected_str = match selected.len() {
                        0 => {
                            if filter.operand == FilterOperand::NotContains
                                && filter.value.is_empty()
                            {
                                return false;
                            }
                            String::new()
                        }
                        1 => value_to_simple_str(selected[0]),
                        _ => selected
                            .iter()
                            .map(|s| value_to_simple_str(s.clone()))
                            .collect::<Vec<String>>()
                            .join(", "),
                    };

                    log::debug!("Selected stringified: {}", selected_str);

                    match filter.operand {
                        FilterOperand::Contains => {
                            if !selected_str.contains(&filter.value) {
                                return false;
                            }
                        }
                        FilterOperand::NotContains => {
                            if selected_str.contains(&filter.value) {
                                return false;
                            }
                        }
                        FilterOperand::Equals => {
                            if selected_str != filter.value {
                                return false;
                            }
                        }
                        FilterOperand::NotEquals => {
                            if selected_str == filter.value {
                                return false;
                            }
                        }
                        FilterOperand::StartsWith => {
                            if !selected_str.starts_with(&filter.value) {
                                return false;
                            }
                        }
                        FilterOperand::EndsWith => {
                            if !selected_str.ends_with(&filter.value) {
                                return false;
                            }
                        }
                        FilterOperand::GreaterThanOrEqual => {
                            let selected_float = match selected_str.parse::<f64>() {
                                Ok(selected_float) => selected_float,
                                Err(_) => {
                                    return false;
                                }
                            };
                            let filter_float = match filter.value.parse::<f64>() {
                                Ok(filter_float) => filter_float,
                                Err(_) => {
                                    return false;
                                }
                            };
                            if selected_float < filter_float {
                                return false;
                            }
                        }
                        FilterOperand::LessThanOrEqual => {
                            let selected_float = match selected_str.parse::<f64>() {
                                Ok(selected_float) => selected_float,
                                Err(_) => {
                                    return false;
                                }
                            };
                            let filter_float = match filter.value.parse::<f64>() {
                                Ok(filter_float) => filter_float,
                                Err(_) => {
                                    return false;
                                }
                            };
                            if selected_float > filter_float {
                                return false;
                            }
                        }
                    };
                }

                return true;
            });
        }
        if has_package_filters {
            // Filtering on packages is much easier, as we don't use JSONPath's here. The JSON object is a simple "Map<String,String> | null".
            minions.retain(|minion| {
                // Parse Grains from JSON
                let packages = minion.pkgs.clone().unwrap_or_default();
                let packages: Value = serde_json::from_str(&packages).unwrap_or_default();
                // Iterate filters with index
                for filter in &filters {
                    // Skip filters that are not of type Grain, when filtering grains
                    if filter.field_type != FilterFieldType::Package {
                        continue;
                    }

                    let filter_value = filter.value.trim().to_owned();
                    let version = match &packages[&filter.field] {
                        Value::String(s) => Some(s),
                        _ => None,
                    };

                    match filter.operand {
                        FilterOperand::Contains => {
                            if filter_value.len() == 0 {
                                if version.is_none() {
                                    return false;
                                }
                            } else if version.is_none() || !version.unwrap().contains(&filter_value)
                            {
                                return false;
                            }
                        }
                        FilterOperand::NotContains => {
                            if filter_value.len() == 0 {
                                if version.is_some() {
                                    return false;
                                }
                            } else if version.is_some() && version.unwrap().contains(&filter_value)
                            {
                                return false;
                            }
                        }
                        FilterOperand::Equals => {
                            if version.is_none() || version.unwrap() != &filter_value {
                                return false;
                            }
                        }
                        FilterOperand::NotEquals => {
                            if version.is_none() || version.unwrap() == &filter_value {
                                return false;
                            }
                        }
                        FilterOperand::StartsWith => {
                            if version.is_none() || !version.unwrap().starts_with(&filter_value) {
                                return false;
                            }
                        }
                        FilterOperand::EndsWith => {
                            if version.is_none() || !version.unwrap().ends_with(&filter_value) {
                                return false;
                            }
                        }
                        FilterOperand::GreaterThanOrEqual => {
                            if version.is_none() {
                                return false;
                            }
                            match version_compare::compare_to(
                                version.unwrap(),
                                &filter_value,
                                Cmp::Ge,
                            ) {
                                Ok(result) => {
                                    if !result {
                                        return false;
                                    }
                                }
                                Err(_) => return false,
                            }
                        }
                        FilterOperand::LessThanOrEqual => {
                            if version.is_none() {
                                return false;
                            }
                            match version_compare::compare_to(
                                version.unwrap(),
                                &filter_value,
                                Cmp::Le,
                            ) {
                                Ok(result) => {
                                    if !result {
                                        return false;
                                    }
                                }
                                Err(_) => return false,
                            }
                        }
                    };
                }
                return true;
            });
        }

        // 2nd Limit if the first limit didn't kick in
        if has_grain_filters || has_package_filters {
            let offset = offset as usize;
            let limit = limit as usize;
            minions = minions.into_iter().skip(offset).take(limit).collect();
        }

        return Ok(minions);
    }

    pub fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String> {
        let mut connection = self.create_connection()?;
        minions::table
            .filter(minions::id.eq(id))
            .first(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
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

        let changeset = Minion {
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

    pub fn update_minion_last_seen(
        &self,
        minion_id: String,
        time: chrono::NaiveDateTime,
    ) -> Result<(), String> {
        self.update_minion(minion_id, time, None, None, None, None, None, None, None)
    }

    pub fn update_minion_grains(
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

    pub fn update_minion_pillars(
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

    pub fn update_minion_pkgs(
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

    pub fn update_minion_conformity(
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

    // Delete minions not in the list of ID's
    pub fn prune_minions(&self, ids: Vec<String>) -> Result<(), String> {
        let mut connection = self.create_connection()?;

        diesel::delete(minions::table.filter(minions::id.ne_all(ids)))
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    //////////////
    /// Events ///
    //////////////

    pub fn insert_event(
        &self,
        tag: String,
        data: String,
        timestamp: NaiveDateTime,
    ) -> Result<String, String> {
        let mut connection = self.create_connection()?;
        let id = format!("evnt_{}", uuid::Uuid::new_v4());
        let event = Event {
            id: id.clone(),
            timestamp,
            tag,
            data,
        };
        diesel::insert_into(events::table)
            .values(&event)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(id)
    }

    pub fn list_events(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Event>, String> {
        let mut connection = self.create_connection()?;
        let mut query = events::table.into_boxed();
        query = query.order(events::timestamp.desc());

        // Filtering

        // Pagination
        query = query.limit(limit.unwrap_or(100));
        query = query.offset(offset.unwrap_or(0));

        // Query
        query
            .load::<Event>(&mut connection)
            .map_err(|e| format!("{:?}", e))
    }

    ////////////
    /// Jobs ///
    ////////////

    pub fn insert_job(
        &self,
        jid: String,
        user: Option<String>,
        event_id: Option<String>,
        timestamp: NaiveDateTime,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let id = format!("job_{}", uuid::Uuid::new_v4());
        let job = Job {
            id,
            timestamp,
            jid,
            user,
            event_id,
        };
        diesel::insert_into(jobs::table)
            .values(&job)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    pub fn list_jobs(
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
        match sort.unwrap_or(String::from("id.asc")).as_str() {
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
            .load::<Job>(&mut connection)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String> {
        let mut connection = self.create_connection()?;
        jobs::table
            .filter(jobs::jid.eq(jid))
            .first(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    ///////////////////
    /// Job Returns ///
    ///////////////////

    pub fn insert_job_return(
        &self,
        jid: String,
        job_id: String,
        event_id: String,
        minion_id: String,
        timestamp: NaiveDateTime,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let id = format!("jret_{}", uuid::Uuid::new_v4());
        let job_return = JobReturn {
            id,
            timestamp,
            jid,
            job_id,
            event_id,
            minion_id,
        };
        diesel::insert_into(job_returns::table)
            .values(&job_return)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    pub fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<Event>, String> {
        let mut connection = self.create_connection()?;
        events::table
            .inner_join(job_returns::table.on(events::id.eq(job_returns::event_id)))
            .filter(job_returns::job_id.eq(&job.id))
            .load::<(Event, JobReturn)>(&mut connection)
            .map(|v: Vec<(Event, JobReturn)>| v.into_iter().map(|(e, _)| e).collect())
            .map_err(|e| format!("{:?}", e))
    }

    ///////////////
    /// Metrics ///
    ///////////////

    pub fn get_metric_results(&self) -> Result<Vec<MetricResult>, String> {
        let mut connection = self.create_connection()?;
        let mut results: Vec<MetricResult> = Vec::new();

        //
        // Gather data
        //
        let mut custom_grains_metrics: Vec<(&str, &str)> = vec![
            ("osfinger", "Operating System"),
            ("efi-secure-boot", "EFI Secure Boot"),
        ];
        let minions = minions::table
            .load::<Minion>(&mut connection)
            .map_err(|e| format!("{:?}", e))?;

        let mut minions_success = 0;
        let mut minions_incorrect = 0;
        let mut minions_error = 0;
        let mut minions_unknown = 0;
        let mut grains: Vec<Option<Value>> = Vec::new();
        for minion in minions {
            // Minion compliance
            if minion.conformity_success.is_none() {
                minions_unknown += 1;
            } else {
                let conf_incorrect = minion.conformity_incorrect.unwrap_or(0);
                let conf_error = minion.conformity_error.unwrap_or(0);

                if conf_error > 0 {
                    minions_error += 1;
                } else if conf_incorrect > 0 {
                    minions_incorrect += 1;
                } else {
                    minions_success += 1;
                }
            }

            // Grains
            grains.push(match minion.grains {
                Some(ref grains) => match serde_json::from_str(grains) {
                    Ok(grains) => Some(grains),
                    Err(e) => {
                        error!(
                            "Failed to deserialize grains for minion {}: {}",
                            minion.id, e
                        );
                        None
                    }
                },
                None => None,
            });
        }
        results.push(MetricResult {
            title: "Conformity".to_string(),
            chart: "pie".to_string(),
            labels: vec![
                "Correct".to_string(),
                "Incorrect".to_string(),
                "Error".to_string(),
                "Unknown".to_string(),
            ],
            data: vec![MetricResultData {
                label: String::new(), // this label is unused on pie charts
                data: vec![
                    minions_success,
                    minions_incorrect,
                    minions_error,
                    minions_unknown,
                ],
            }],
        });

        //
        // Custom grain metrics
        //
        for (mid, mname) in &mut custom_grains_metrics {
            let mut founds: HashMap<String, i32> = HashMap::new();
            for grain in grains.iter() {
                let value = match grain {
                    Some(ggg) => ggg
                        .get(mid.clone())
                        .and_then(|v| {
                            if v.is_string() {
                                Some(v.as_str().unwrap().to_string())
                            } else {
                                Some(v.to_string())
                            }
                        })
                        .unwrap_or("Missing".to_string()),
                    None => "Unknown".to_string(),
                };
                let counter = founds.get(&value).unwrap_or(&0);
                founds.insert(value, counter + 1);
            }

            // Insert final metric
            let mut founds: Vec<(String, i32)> = founds.into_iter().collect();
            founds.sort_by(|a, b| b.1.cmp(&a.1));
            let mut labels: Vec<String> = Vec::new();
            let mut data: Vec<i32> = Vec::new();
            for (label, value) in founds {
                labels.push(label);
                data.push(value);
            }
            results.push(MetricResult {
                title: mname.to_string(),
                chart: "pie".to_string(),
                labels,
                data: vec![MetricResultData {
                    label: String::new(), // this label is unused on pie charts
                    data,
                }],
            });
        }

        Ok(results)
    }

    /////////////////////////
    /// Permission Groups ///
    /////////////////////////

    pub fn create_permission_group(&self, name: &str) -> Result<String, String> {
        let mut connection = self.create_connection()?;
        let id = format!("pg_{}", uuid::Uuid::new_v4());
        let permission_group = PermissionGroup {
            id: id.clone(),
            name: name.to_owned(),
            perms: "[]".to_string(),
            ldap_sync: None,
        };
        diesel::insert_into(permission_groups::table)
            .values(&permission_group)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(id)
    }

    pub fn list_permission_groups(
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
            .load::<PermissionGroup>(&mut connection)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        permission_groups::table
            .filter(permission_groups::id.eq(id))
            .first::<PermissionGroup>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub fn get_permission_group_by_name(
        &self,
        name: &str,
    ) -> Result<Option<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        permission_groups::table
            .filter(permission_groups::name.eq(name))
            .first::<PermissionGroup>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub fn is_user_member_of_group(&self, user_id: &str, group_id: &str) -> Result<bool, String> {
        let mut connection = self.create_connection()?;
        match permission_group_users::table
            .filter(permission_group_users::user_id.eq(user_id))
            .filter(permission_group_users::group_id.eq(group_id))
            .first::<PermissionGroupUser>(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))?
        {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub fn update_permission_group(
        &self,
        permission_group: &PermissionGroup,
    ) -> Result<(), String> {
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

    pub fn delete_permission_group(&self, id: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        diesel::delete(permission_groups::table.filter(permission_groups::id.eq(id)))
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    /////////////////////////////////
    /// Permission Groups (Users) ///
    /////////////////////////////////

    pub fn insert_permission_group_user(
        &self,
        user_id: &str,
        group_id: &str,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        let id = format!("pgu_{}", uuid::Uuid::new_v4());
        let permission_group_user = PermissionGroupUser {
            id,
            user_id: user_id.to_string(),
            group_id: group_id.to_string(),
        };
        diesel::insert_into(permission_group_users::table)
            .values(&permission_group_user)
            .execute(&mut connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    pub fn list_permission_groups_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<PermissionGroup>, String> {
        let mut connection = self.create_connection()?;
        permission_groups::table
            .inner_join(permission_group_users::table)
            .filter(permission_group_users::user_id.eq(user_id))
            .select(permission_groups::all_columns)
            .load(&mut connection)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn list_users_by_permission_group_id(&self, group_id: &str) -> Result<Vec<User>, String> {
        let mut connection = self.create_connection()?;
        users::table
            .inner_join(permission_group_users::table)
            .filter(permission_group_users::group_id.eq(group_id))
            .select(users::all_columns)
            .load(&mut connection)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn delete_permission_group_user(
        &self,
        user_id: &str,
        group_id: &str,
    ) -> Result<(), String> {
        let mut connection = self.create_connection()?;
        diesel::delete(
            permission_group_users::table
                .filter(permission_group_users::user_id.eq(user_id))
                .filter(permission_group_users::group_id.eq(group_id)),
        )
        .execute(&mut connection)
        .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }
}
