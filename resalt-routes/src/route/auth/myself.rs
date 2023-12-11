use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use log::*;
use resalt_api::{permission::get_permission_groups_by_user_id, user::get_user_by_id};
use resalt_models::AuthStatus;
use resalt_storage::Storage;

pub async fn route_myself_get(
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // API
    let user = match get_user_by_id(&data, &auth.user_id) {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(e) => {
            error!("route_myself_get.user {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let permission_groups = match get_permission_groups_by_user_id(&data, &user.id) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("route_myself_get.groups {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(user.public(permission_groups)))
}
