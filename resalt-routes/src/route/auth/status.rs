use crate::permission::*;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use resalt_models::*;
use resalt_salt::SaltEventListenerStatus;
use resalt_storage::Storage;

pub async fn route_status_get(
    State(_listener_status): State<SaltEventListenerStatus>,
    State(_data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_LIST)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // API
    Ok(Json(StorageStatus {
        auth_tokens_total: -1,
        auth_tokens_active: -1,
        events_total: -1,
        job_returns_total: -1,
        jobs_total: -1,
        minions_total: -1,
        minions_success: -1,
        minions_incorrect: -1,
        minions_error: -1,
        minions_unknown: -1,
        permission_group_users_total: -1,
        permission_groups_total: -1,
        users_total: -1,
    }))
}
