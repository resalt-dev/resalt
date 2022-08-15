extern crate diesel;

use self::diesel::prelude::*;
use crate::{prelude::*, schema::*};
use chrono::NaiveDateTime;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::embed_migrations;
use log::{error, warn};

type DbPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

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
                let connection = match own.create_connection() {
                    Ok(conn) => conn,
                    Err(e) => return Err(e),
                };

                match embedded_migrations::run(&*connection) {
                    Ok(()) => {
                        warn!("Ran database migration!");
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
        if self.list_users().unwrap_or_default().is_empty() {
            self.create_user("admin".to_string(), Some("admin".to_string()))
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

    pub fn create_user(&self, username: String, password: Option<String>) -> Result<User, String> {
        let connection = self.create_connection()?;
        let id = format!("usr_{}", uuid::Uuid::new_v4());
        let user = User {
            id,
            username,
            password: password.map(|v| hash_password(&v)),
        };

        diesel::insert_into(users::table)
            .values(&user)
            .execute(&connection)
            .map_err(|e| format!("{:?}", e))?;

        Ok(user)
    }

    pub fn list_users(&self) -> Result<Vec<User>, String> {
        let connection = self.create_connection()?;
        users::table
            .load::<User>(&connection)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let connection = self.create_connection()?;
        users::table
            .filter(users::id.eq(id))
            .first(&connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let connection = self.create_connection()?;
        users::table
            .filter(users::username.eq(username))
            .first(&connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    ///////////////////
    /// Auth tokens ///
    ///////////////////

    pub fn create_authtoken(&self, user_id: String) -> Result<AuthToken, String> {
        let connection = self.create_connection()?;
        let id = format!("auth_{}", uuid::Uuid::new_v4());
        let authtoken = AuthToken {
            id,
            user_id,
            timestamp: chrono::Utc::now().naive_utc(),
            salt_token: None,
        };

        diesel::insert_into(authtokens::table)
            .values(&authtoken)
            .execute(&connection)
            .map_err(|e| format!("{:?}", e))?;

        Ok(authtoken)
    }

    pub fn update_authtoken_salttoken(
        &self,
        auth_token: &str,
        salt_token: &Option<SaltToken>,
    ) -> Result<(), String> {
        let connection = self.create_connection()?;
        let salt_token = salt_token
            .as_ref()
            .map(|salt_token| serde_json::to_string(salt_token).unwrap());

        diesel::update(authtokens::table)
            .filter(authtokens::id.eq(auth_token))
            .set(authtokens::salt_token.eq(salt_token))
            .execute(&connection)
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    pub fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        let connection = self.create_connection()?;
        authtokens::table
            .filter(authtokens::id.eq(id))
            .first(&connection)
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
        let connection = self.create_connection()?;

        let last_updated_grains = grains.as_ref().map(|_| time);
        let last_updated_pillars = pillars.as_ref().map(|_| time);
        let last_updated_pkgs = pkgs.as_ref().map(|_| time);
        let last_updated_conformity = conformity.as_ref().map(|_| time);
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
        };

        // Update if it exists, insert if it doesn't

        let result = diesel::update(minions::table)
            .filter(minions::id.eq(&minion_id))
            .set(&changeset)
            .execute(&connection)
            .map_err(|e| format!("{:?}", e))?;

        if result == 0 {
            match diesel::insert_into(minions::table)
                .values(&changeset)
                .execute(&connection)
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
                                .execute(&connection)
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
        let connection = self.create_connection()?;
        minions::table
            .filter(minions::id.eq(id))
            .first(&connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub fn list_minions(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Minion>, String> {
        let connection = self.create_connection()?;
        let mut query = minions::table.into_boxed();
        query = query.order(minions::id.asc());

        // Filtering

        // Pagination
        query = query.limit(limit.unwrap_or(100));
        query = query.offset(offset.unwrap_or(0));

        query
            .load::<Minion>(&connection)
            .map_err(|e| format!("{:?}", e))
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
        let connection = self.create_connection()?;
        let id = format!("evnt_{}", uuid::Uuid::new_v4());
        let event = Event {
            id: id.clone(),
            timestamp,
            tag,
            data,
        };
        diesel::insert_into(events::table)
            .values(&event)
            .execute(&connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(id)
    }

    pub fn list_events(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Event>, String> {
        let connection = self.create_connection()?;
        let mut query = events::table.into_boxed();
        query = query.order(events::timestamp.desc());

        // Filtering

        // Pagination
        query = query.limit(limit.unwrap_or(100));
        query = query.offset(offset.unwrap_or(0));

        // Query
        query
            .load::<Event>(&connection)
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
        let connection = self.create_connection()?;
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
            .execute(&connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    pub fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String> {
        let connection = self.create_connection()?;
        jobs::table
            .filter(jobs::jid.eq(jid))
            .first(&connection)
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
        let connection = self.create_connection()?;
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
            .load::<Job>(&connection)
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
        let connection = self.create_connection()?;
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
            .execute(&connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    pub fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<Event>, String> {
        let connection = self.create_connection()?;
        events::table
            .inner_join(job_returns::table.on(events::id.eq(job_returns::event_id)))
            .filter(job_returns::job_id.eq(&job.id))
            .load::<(Event, JobReturn)>(&connection)
            .map(|v: Vec<(Event, JobReturn)>| v.into_iter().map(|(e, _)| e).collect())
            .map_err(|e| format!("{:?}", e))
    }
}
