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
            self.create_user("admin", Some("admin")).unwrap();
        }
    }

    fn create_connection(&self) -> Result<DbPooledConnection, String> {
        return match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(format!("{:?}", e)),
        };
    }

    pub fn create_user(&self, username: &str, password: Option<&str>) -> Result<User, String> {
        let connection = self.create_connection()?;
        let uuid = format!("usr_{}", uuid::Uuid::new_v4());
        let user = User {
            id: uuid,
            username: username.to_string(),
            password: password.map(|v| hash_password(v)),
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

    pub fn create_authtoken(&self, user_id: &str) -> Result<AuthToken, String> {
        let connection = self.create_connection()?;
        let uuid = format!("auth_{}", uuid::Uuid::new_v4());
        let authtoken = AuthToken {
            id: uuid,
            user_id: user_id.to_string(),
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
        let salt_token = match salt_token {
            Some(salt_token) => Some(serde_json::to_string(salt_token).unwrap()),
            None => None,
        };

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

    fn update_minion(
        &self,
        minion_id: &str,
        time: chrono::NaiveDateTime,
        grains: Option<&str>,
        pillars: Option<&str>,
        pkgs: Option<&str>,
        conformity: Option<&str>,
        conformity_success: Option<i32>,
        conformity_incorrect: Option<i32>,
        conformity_error: Option<i32>,
    ) -> Result<(), String> {
        let connection = self.create_connection()?;

        let last_updated_grains = match grains {
            Some(_) => Some(time.clone()),
            None => None,
        };
        let last_updated_pillars = match pillars {
            Some(_) => Some(time.clone()),
            None => None,
        };
        let last_updated_pkgs = match pkgs {
            Some(_) => Some(time.clone()),
            None => None,
        };
        let last_updated_conformity = match conformity {
            Some(_) => Some(time.clone()),
            None => None,
        };
        let changeset = Minion {
            id: minion_id.to_string(),
            last_seen: time,
            grains: grains.map(|s| s.to_string()),
            pillars: pillars.map(|s| s.to_string()),
            pkgs: pkgs.map(|s| s.to_string()),
            last_updated_grains,
            last_updated_pillars,
            last_updated_pkgs,
            conformity: conformity.map(|s| s.to_string()),
            conformity_success,
            conformity_incorrect,
            conformity_error,
            last_updated_conformity,
        };

        // Update if it exists, insert if it doesn't

        let result = diesel::update(minions::table)
            .filter(minions::id.eq(minion_id))
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
        minion_id: &str,
        time: chrono::NaiveDateTime,
    ) -> Result<(), String> {
        self.update_minion(minion_id, time, None, None, None, None, None, None, None)
    }

    pub fn update_minion_grains(
        &self,
        minion_id: &str,
        time: chrono::NaiveDateTime,
        grains: &str,
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
        minion_id: &str,
        time: chrono::NaiveDateTime,
        pillars: &str,
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
        minion_id: &str,
        time: chrono::NaiveDateTime,
        pkgs: &str,
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
        minion_id: &str,
        time: chrono::NaiveDateTime,
        conformity: &str,
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

    pub fn list_minions(&self) -> Result<Vec<Minion>, String> {
        let connection = self.create_connection()?;
        minions::table
            .load::<Minion>(&connection)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn insert_event(
        &self,
        tag: &str,
        data: &str,
        time: &NaiveDateTime,
    ) -> Result<String, String> {
        let connection = self.create_connection()?;
        let uuid = format!("evnt_{}", uuid::Uuid::new_v4());
        let event = Event {
            id: uuid.clone(),
            timestamp: *time,
            tag: tag.to_string(),
            data: data.to_string(),
        };
        diesel::insert_into(events::table)
            .values(&event)
            .execute(&connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(uuid)
    }

    pub fn list_events(&self) -> Result<Vec<Event>, String> {
        let connection = self.create_connection()?;
        // filter by latest timestamp first, limit to 100 for now.
        events::table
            .order(events::timestamp.desc())
            .limit(2000)
            .load::<Event>(&connection)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn insert_job(
        &self,
        jid: &str,
        user: &str,
        minions: &str,
        event_id: &str,
        time: &NaiveDateTime,
    ) -> Result<(), String> {
        let connection = self.create_connection()?;
        let uuid = format!("job_{}", uuid::Uuid::new_v4());
        let job = Job {
            id: uuid.clone(),
            timestamp: *time,
            jid: jid.to_string(),
            user: user.to_string(),
            minions: minions.to_string(),
            event_id: event_id.to_string(),
        };
        diesel::insert_into(jobs::table)
            .values(&job)
            .execute(&connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
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

        query
            .order(jobs::timestamp.desc())
            .limit(100)
            .load::<Job>(&connection)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String> {
        let connection = self.create_connection()?;
        jobs::table
            .filter(jobs::jid.eq(jid))
            .first(&connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub fn insert_job_return(
        &self,
        jid: &str,
        job_id: &str,
        event_id: &str,
        time: &NaiveDateTime,
    ) -> Result<(), String> {
        let connection = self.create_connection()?;
        let uuid = format!("jret_{}", uuid::Uuid::new_v4());
        let job_return = JobReturn {
            id: uuid.clone(),
            timestamp: *time,
            jid: jid.to_string(),
            job_id: job_id.to_string(),
            event_id: event_id.to_string(),
        };
        diesel::insert_into(job_returns::table)
            .values(&job_return)
            .execute(&connection)
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    pub fn get_job_returns_by_job(&self, job: Job) -> Result<Vec<Event>, String> {
        let connection = self.create_connection()?;
        events::table
            .inner_join(job_returns::table)
            .filter(job_returns::job_id.eq(job.id))
            .load::<(Event, JobReturn)>(&connection)
            .map(|v: Vec<(Event, JobReturn)>| v.into_iter().map(|(e, _)| e).collect())
            .map_err(|e| format!("{:?}", e))
    }
}
