extern crate diesel;

use self::diesel::prelude::*;
use crate::{prelude::*, schema::*};
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

type DbPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct Storage {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Storage {
    pub async fn connect(database_url: &str) -> Result<Self, PoolError> {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder().build(manager);

        match pool {
            Ok(pool) => Ok(Self { pool }),
            Err(e) => Err(e),
        }
    }

    pub async fn init(&self) {
        // Create default user
        if self.list_users().await.unwrap_or_default().is_empty() {
            self.create_user("admin", Some("admin")).await.unwrap();
        }
    }

    async fn create_connection(&self) -> Result<DbPooledConnection, String> {
        return match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(format!("{:?}", e)),
        };
    }

    pub async fn create_user(
        &self,
        username: &str,
        password: Option<&str>,
    ) -> Result<User, String> {
        let connection = self.create_connection().await?;
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

    pub async fn list_users(&self) -> Result<Vec<User>, String> {
        let connection = self.create_connection().await?;
        users::table
            .load::<User>(&connection)
            .map_err(|e| format!("{:?}", e))
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let connection = self.create_connection().await?;
        users::table
            .filter(users::id.eq(id))
            .first(&connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let connection = self.create_connection().await?;
        users::table
            .filter(users::username.eq(username))
            .first(&connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub async fn create_authtoken(&self, user_id: &str) -> Result<AuthToken, String> {
        let connection = self.create_connection().await?;
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

    pub async fn update_authtoken_salttoken(
        &self,
        auth_token: &str,
        salt_token: &Option<SaltToken>,
    ) -> Result<(), String> {
        let connection = self.create_connection().await?;
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

    pub async fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        let connection = self.create_connection().await?;
        authtokens::table
            .filter(authtokens::id.eq(id))
            .first(&connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    async fn update_minion(
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
        let connection = self.create_connection().await?;

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

    pub async fn update_minion_last_seen(
        &self,
        minion_id: &str,
        time: chrono::NaiveDateTime,
    ) -> Result<(), String> {
        self.update_minion(minion_id, time, None, None, None, None, None, None, None)
            .await
    }

    pub async fn update_minion_grains(
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
        .await
    }

    pub async fn update_minion_pillars(
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
        .await
    }

    pub async fn update_minion_pkgs(
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
        .await
    }

    pub async fn update_minion_conformity(
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
        .await
    }

    pub async fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String> {
        let connection = self.create_connection().await?;
        minions::table
            .filter(minions::id.eq(id))
            .first(&connection)
            .optional()
            .map_err(|e| format!("{:?}", e))
    }

    pub async fn list_minions(&self) -> Result<Vec<Minion>, String> {
        let connection = self.create_connection().await?;
        minions::table
            .load::<Minion>(&connection)
            .map_err(|e| format!("{:?}", e))
    }
}
