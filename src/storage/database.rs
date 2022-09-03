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

type DbPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
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

                Ok(own)
            }
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub async fn init(&self) {
        // Create default user
        if self.get_user_by_username("admin").unwrap().is_none() {
            // Generate random password instead of using default
            let random_password = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(15)
                .map(|c| c.to_string())
                .collect::<String>();

            // Create initial admin user
            let user = self
                .create_user("admin".to_string(), Some(random_password.to_string()))
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
            let perms = serde_json::to_string(&perms).unwrap();
            self.update_user_permissions(&user.id, &perms).unwrap();

            // Announce randomly generated password
            warn!("============================================================");
            warn!(
                "==  CREATED DEFAULT USER: admin WITH PASSWORD: {}  ==",
                random_password
            );
            warn!("============================================================");
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

    pub fn create_user(&self, username: String, password: Option<String>) -> Result<User, String> {
        let mut connection = self.create_connection()?;
        let id = format!("usr_{}", uuid::Uuid::new_v4());
        let user = User {
            id,
            username,
            password: password.map(|v| hash_password(&v)),
            perms: "[]".to_string(),
            last_login: None,
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

    pub fn update_user_permissions(&self, id: &str, perms_str: &str) -> Result<(), String> {
        let mut connection = self.create_connection()?;

        diesel::update(users::table)
            .filter(users::id.eq(&id))
            .set(users::perms.eq(perms_str))
            .execute(&mut connection)
            .map(|_| ())
            .map_err(|e| format!("{:?}", e))
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

    pub fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        let mut connection = self.create_connection()?;
        authtokens::table
            .filter(authtokens::id.eq(id))
            .first(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    ///////////////
    /// Minions ///
    ///////////////

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

    pub fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String> {
        let mut connection = self.create_connection()?;
        minions::table
            .filter(minions::id.eq(id))
            .first(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub fn list_minions(
        &self,
        sort: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Minion>, String> {
        let mut connection = self.create_connection()?;
        let mut query = minions::table.into_boxed();
        query = query.order(minions::id.asc());

        // Filtering

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
            _ => {}
        }

        // Pagination
        query = query.limit(limit.unwrap_or(100));
        query = query.offset(offset.unwrap_or(0));

        query
            .load::<Minion>(&mut connection)
            .map_err(|e| format!("{:?}", e))
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

    pub fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String> {
        let mut connection = self.create_connection()?;
        jobs::table
            .filter(jobs::jid.eq(jid))
            .first(&mut connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub fn list_jobs(
        &self,
        user: Option<String>,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Job>, String> {
        let mut connection = self.create_connection()?;
        let mut query = jobs::table.into_boxed();
        query = query.order(jobs::timestamp.desc());

        // Filtering
        if let Some(user) = user {
            query = query.filter(jobs::user.eq(user));
        }
        if let Some(start_date) = start_date {
            query = query.filter(jobs::timestamp.ge(start_date));
        }
        if let Some(end_date) = end_date {
            query = query.filter(jobs::timestamp.le(end_date));
        }

        // Pagination
        query = query.limit(limit.unwrap_or(100));
        query = query.offset(offset.unwrap_or(0));

        // Query
        query
            .load::<Job>(&mut connection)
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
        let mut custom_grains_metrics: Vec<(&str, &str)> =
            vec![("osfinger", "Operating System"), ("ipv4", "IPv4")];
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
}
