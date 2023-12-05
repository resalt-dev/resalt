use axum::{extract::State, response::IntoResponse, Extension, Json};
use log::*;
use resalt_api::user::get_user_by_id;
use resalt_models::{ApiError, AuthStatus};
use resalt_storage::StorageImpl;

pub async fn route_myself_get(
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // API
    let user = match get_user_by_id(&data, &auth.user_id) {
        Ok(Some(user)) => user,
        Ok(None) => return Err(ApiError::Unauthorized),
        Err(e) => {
            error!("route_myself_get.user {:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let permission_groups = match data.list_permission_groups_by_user_id(&user.id) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("route_myself_get.groups {:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    Ok(Json(user.public(permission_groups)))
}
