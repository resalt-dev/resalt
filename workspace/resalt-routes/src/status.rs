use std::sync::{Arc, Mutex};

use actix_web::{get, web, Responder, Result};
use resalt_models::*;
use resalt_salt::SaltEventListenerStatus;
use resalt_storage::{StorageImpl, StorageStatus};

#[get("/status")]
pub async fn route_status_get(
    listener_status: web::Data<Arc<Mutex<SaltEventListenerStatus>>>,
    data: web::Data<Box<dyn StorageImpl>>,
) -> Result<impl Responder, ApiError> {
    let db_status: Option<StorageStatus> = match data.get_status() {
        Ok(s) => Some(s),
        Err(e) => {
            log::error!("Error getting database status: {}", e);
            None
        }
    };

    let salt: bool;
    {
        salt = listener_status.lock().unwrap().connected;
    }

    #[allow(clippy::redundant_clone)]
    let status = SystemStatus {
        // Salt
        salt,
        // DB
        db: db_status.is_some(),
        db_auth_tokens_total: db_status.clone().map(|s| s.auth_tokens_total),
        db_auth_tokens_active: db_status.clone().map(|s| s.auth_tokens_active),
        db_events_total: db_status.clone().map(|s| s.events_total),
        db_job_returns_total: db_status.clone().map(|s| s.job_returns_total),
        db_jobs_total: db_status.clone().map(|s| s.jobs_total),
        db_minions_total: db_status.clone().map(|s| s.minions_total),
        db_permission_group_users_total: db_status.clone().map(|s| s.permission_group_users_total),
        db_permission_groups_total: db_status.clone().map(|s| s.permission_groups_total),
        db_users_total: db_status.clone().map(|s| s.users_total),
    };

    Ok(web::Json(status))
}
