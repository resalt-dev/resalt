use axum::{extract::State, response::IntoResponse, Extension, Json};
use log::*;
use resalt_models::{ApiError, AuthStatus};
use resalt_storage::StorageImpl;

pub async fn route_myself_get(
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    let db = data;

    let user = match db.get_user_by_id(&auth.user_id) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(ApiError::Unauthorized),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let permission_groups = match db.list_permission_groups_by_user_id(&user.id) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    Ok(Json(user.public(permission_groups)))
}
