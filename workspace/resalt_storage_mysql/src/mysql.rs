extern crate diesel;

use self::diesel::prelude::*;
use crate::diesel_migrations::MigrationHarness;
use crate::schema::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::EmbeddedMigrations;
use log::*;
use resalt_storage::Storage;

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

                Ok(own)
            }
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    fn create_connection(&self) -> Result<DbPooledConnection, String> {
        return match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(format!("{:?}", e)),
        };
    }
}

impl Storage for StorageMySQL {
    fn create_user(
        &self,
        username: String,
        password: Option<String>,
        email: Option<String>,
        ldap_sync: Option<String>,
    ) {
        todo!()
    }

    fn list_users(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, String> {
        todo!()
    }

    fn get_user_by_id(&self, id: &str) -> Result<Option<User>, String> {
        todo!()
    }

    fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        todo!()
    }

    fn update_user(&self, user: &User) -> Result<(), String> {
        todo!()
    }

    fn create_authtoken(&self, user_id: String) -> Result<AuthToken, String> {
        todo!()
    }

    fn get_authtoken_by_id(&self, id: &str) -> Result<Option<AuthToken>, String> {
        todo!()
    }

    fn update_authtoken_salttoken(&self, auth_token: &str, salt_token: &Option<SaltToken>) {
        todo!()
    }

    fn list_minions(
        &self,
        filters: Vec<Filter>,
        sort: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) {
        todo!()
    }

    fn get_minion_by_id(&self, id: &str) -> Result<Option<Minion>, String> {
        todo!()
    }

    fn update_minion_last_seen(&self, minion_id: String, time: chrono::NaiveDateTime) {
        todo!()
    }

    fn update_minion_grains(&self, minion_id: String, time: chrono::NaiveDateTime, grains: String) {
        todo!()
    }

    fn update_minion_pillars(
        &self,
        minion_id: String,
        time: chrono::NaiveDateTime,
        pillars: String,
    ) {
        todo!()
    }

    fn update_minion_pkgs(&self, minion_id: String, time: chrono::NaiveDateTime, pkgs: String) {
        todo!()
    }

    fn update_minion_conformity(
        &self,
        minion_id: String,
        time: chrono::NaiveDateTime,
        conformity: String,
        success: i32,
        incorrect: i32,
        error: i32,
    ) {
        todo!()
    }

    fn prune_minions(&self, ids: Vec<String>) -> Result<(), String> {
        todo!()
    }

    fn insert_event(&self, tag: String, data: String, timestamp: chrono::NaiveDateTime) {
        todo!()
    }

    fn list_events(&self, limit: Option<i64>, offset: Option<i64>) {
        todo!()
    }

    fn insert_job(
        &self,
        jid: String,
        user: Option<String>,
        event_id: Option<String>,
        timestamp: chrono::NaiveDateTime,
    ) {
        todo!()
    }

    fn list_jobs(&self, sort: Option<String>, limit: Option<i64>, offset: Option<i64>) {
        todo!()
    }

    fn get_job_by_jid(&self, jid: &str) -> Result<Option<Job>, String> {
        todo!()
    }

    fn insert_job_return(
        &self,
        jid: String,
        job_id: String,
        event_id: String,
        minion_id: String,
        timestamp: chrono::NaiveDateTime,
    ) {
        todo!()
    }

    fn get_job_returns_by_job(&self, job: &Job) -> Result<Vec<Event>, String> {
        todo!()
    }

    fn get_metric_results(&self) -> Result<Vec<MetricResult>, String> {
        todo!()
    }

    fn create_permission_group(&self, name: &str) -> Result<String, String> {
        todo!()
    }

    fn list_permission_groups(&self, limit: Option<i64>, offset: Option<i64>) {
        todo!()
    }

    fn get_permission_group_by_id(&self, id: &str) -> Result<Option<PermissionGroup>, String> {
        todo!()
    }

    fn get_permission_group_by_name(&self, name: &str) {
        todo!()
    }

    fn is_user_member_of_group(&self, user_id: &str, group_id: &str) -> Result<bool, String> {
        todo!()
    }

    fn update_permission_group(&self, permission_group: &PermissionGroup) {
        todo!()
    }

    fn delete_permission_group(&self, id: &str) -> Result<(), String> {
        todo!()
    }

    fn insert_permission_group_user(&self, user_id: &str, group_id: &str) {
        todo!()
    }

    fn list_permission_groups_by_user_id(&self, user_id: &str) {
        todo!()
    }

    fn list_users_by_permission_group_id(&self, group_id: &str) -> Result<Vec<User>, String> {
        todo!()
    }

    fn delete_permission_group_user(&self, user_id: &str, group_id: &str) {
        todo!()
    }
}
