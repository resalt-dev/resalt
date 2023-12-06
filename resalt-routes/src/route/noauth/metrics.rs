use axum::{extract::State, response::IntoResponse};
use log::*;
use resalt_config::ResaltConfig;
use resalt_models::{ApiError, StorageStatus};
use resalt_salt::SaltEventListenerStatus;
use resalt_storage::StorageImpl;

pub async fn route_metrics_get(
    State(listener_status): State<SaltEventListenerStatus>,
    State(data): State<Box<dyn StorageImpl>>,
) -> Result<impl IntoResponse, ApiError> {
    if !*ResaltConfig::METRICS_ENABLED {
        return Err(ApiError::NotFound);
    }

    let db_status: Option<StorageStatus> = match data.get_status() {
        Ok(s) => Some(s),
        Err(e) => {
            error!("Error getting database status: {}", e);
            None
        }
    };

    let salt: bool;
    {
        salt = *listener_status.connected.lock().unwrap();
    }

    // Print Prometheus metrics
    let mut result = String::new();

    result.push_str("# HELP resalt-salt_api_up Salt API is up\n");
    result.push_str("# TYPE resalt-salt_api_up gauge\n");
    result.push_str(&format!("resalt-salt_api_up {}\n", i32::from(salt)));

    result.push_str("# HELP resalt-db_up Database is up\n");
    result.push_str("# TYPE resalt-db_up gauge\n");
    result.push_str(&format!(
        "resalt-db_up {}\n",
        i32::from(db_status.is_some())
    ));

    result.push_str("# HELP resalt-db_auth_tokens_total Total number of auth tokens\n");
    result.push_str("# TYPE resalt-db_auth_tokens_total gauge\n");
    result.push_str(&format!(
        "resalt-db_auth_tokens_total {}\n",
        db_status.clone().map(|s| s.auth_tokens_total).unwrap_or(0)
    ));

    result.push_str("# HELP resalt-db_auth_tokens_active Total number of active auth tokens\n");
    result.push_str("# TYPE resalt-db_auth_tokens_active gauge\n");
    result.push_str(&format!(
        "resalt-db_auth_tokens_active {}\n",
        db_status.clone().map(|s| s.auth_tokens_active).unwrap_or(0)
    ));

    result.push_str("# HELP resalt-db_events_total Total number of events\n");
    result.push_str("# TYPE resalt-db_events_total gauge\n");
    result.push_str(&format!(
        "resalt-db_events_total {}\n",
        db_status.clone().map(|s| s.events_total).unwrap_or(0)
    ));

    result.push_str("# HELP resalt-db_job_returns_total Total number of job returns\n");
    result.push_str("# TYPE resalt-db_job_returns_total gauge\n");
    result.push_str(&format!(
        "resalt-db_job_returns_total {}\n",
        db_status.clone().map(|s| s.job_returns_total).unwrap_or(0)
    ));
    result.push_str("# HELP resalt-db_jobs_total Total number of jobs\n");
    result.push_str("# TYPE resalt-db_jobs_total gauge\n");
    result.push_str(&format!(
        "resalt-db_jobs_total {}\n",
        db_status.clone().map(|s| s.jobs_total).unwrap_or(0)
    ));

    result.push_str("# HELP resalt-db_minions_total Total number of minions\n");
    result.push_str("# TYPE resalt-db_minions_total gauge\n");
    result.push_str(&format!(
        "resalt-db_minions_total {}\n",
        db_status.clone().map(|s| s.minions_total).unwrap_or(0)
    ));
    result.push_str("# HELP resalt-db_minions_success Total number of successful minions\n");
    result.push_str("# TYPE resalt-db_minions_success gauge\n");
    result.push_str(&format!(
        "resalt-db_minions_success {}\n",
        db_status.clone().map(|s| s.minions_success).unwrap_or(0)
    ));
    result.push_str("# HELP resalt-db_minions_incorrect Total number of incorrect minions\n");
    result.push_str("# TYPE resalt-db_minions_incorrect gauge\n");
    result.push_str(&format!(
        "resalt-db_minions_incorrect {}\n",
        db_status.clone().map(|s| s.minions_incorrect).unwrap_or(0)
    ));
    result.push_str("# HELP resalt-db_minions_error Total number of errored minions\n");
    result.push_str("# TYPE resalt-db_minions_error gauge\n");
    result.push_str(&format!(
        "resalt-db_minions_error {}\n",
        db_status.clone().map(|s| s.minions_error).unwrap_or(0)
    ));
    result.push_str("# HELP resalt-db_minions_unknown Total number of unknown state minions\n");
    result.push_str("# TYPE resalt-db_minions_unknown gauge\n");
    result.push_str(&format!(
        "resalt-db_minions_unknown {}\n",
        db_status.clone().map(|s| s.minions_unknown).unwrap_or(0)
    ));

    result.push_str(
        "# HELP resalt-db_permission_group_users_total Total number of permission group users\n",
    );
    result.push_str("# TYPE resalt-db_permission_group_users_total gauge\n");
    result.push_str(&format!(
        "resalt-db_permission_group_users_total {}\n",
        db_status
            .clone()
            .map(|s| s.permission_group_users_total)
            .unwrap_or(0)
    ));

    result.push_str("# HELP resalt-db_permission_groups_total Total number of permission groups\n");
    result.push_str("# TYPE resalt-db_permission_groups_total gauge\n");
    result.push_str(&format!(
        "resalt-db_permission_groups_total {}\n",
        db_status
            .clone()
            .map(|s| s.permission_groups_total)
            .unwrap_or(0)
    ));

    result.push_str("# HELP resalt-db_users_total Total number of users\n");
    result.push_str("# TYPE resalt-db_users_total gauge\n");
    result.push_str(&format!(
        "resalt-db_users_total {}\n",
        db_status.map(|s| s.users_total).unwrap_or(0)
    ));

    Ok(format!("{}\n", result))
}
