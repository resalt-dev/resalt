use crate::permission::*;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use resalt_api::status::get_status;
use resalt_models::*;
use resalt_salt::SaltEventListenerStatus;
use resalt_storage::Storage;

pub async fn route_status_get(
    State(listener_status): State<SaltEventListenerStatus>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_LIST)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // API
    Ok(Json(get_status(&data, &listener_status)))
}
