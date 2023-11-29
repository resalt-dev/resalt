use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_models::{ApiError, AuthStatus};
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct UsersListGetQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_users_get(
    query: Query<UsersListGetQuery>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_USER_LIST)? {
        return Err(ApiError::Forbidden);
    }

    // Pagination
    let limit = query.limit;
    let offset = query.offset;

    let users = match data.list_users(limit, offset) {
        Ok(users) => users,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Map to - among other things - remove password
    let mut results: Vec<Value> = Vec::new();
    for user in users {
        let permission_groups = match data.list_permission_groups_by_user_id(&user.id) {
            Ok(permission_groups) => permission_groups,
            Err(e) => {
                error!("{:?}", e);
                return Err(ApiError::DatabaseError);
            }
        };
        results.push(user.public(permission_groups));
    }

    Ok(Json(results))
}

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub username: String,
    pub email: Option<String>,
}

pub async fn route_users_post(
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<UserCreateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_USER_ADMIN)? {
        return Err(ApiError::Forbidden);
    }

    // Check if username is taken
    let user = match data.get_user_by_username(&input.username) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    if user.is_some() {
        return Err(ApiError::InvalidRequest);
    }

    // Create user
    let user = match data.create_user(input.username.clone(), None, input.email.clone()) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Map to - among other things - remove password
    let permission_group = match data.list_permission_groups_by_user_id(&user.id) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    let user = user.public(permission_group);

    Ok(Json(user))
}

pub async fn route_user_get(
    Path(user_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if auth.user_id == user_id {
        // Always allow fetching self
    } else {
        #[allow(clippy::collapsible_else_if)]
        if !has_resalt_permission(&auth.perms, P_USER_LIST)? {
            return Err(ApiError::Forbidden);
        }
    }

    let user = match data.get_user_by_id(&user_id) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let user = match user {
        Some(user) => user,
        None => return Err(ApiError::NotFound),
    };

    // Map to - among other things - remove password
    let permission_group = match data.list_permission_groups_by_user_id(&user.id) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    let user = user.public(permission_group);

    Ok(Json(user))
}

pub async fn route_user_delete(
    Path(user_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_USER_ADMIN)? {
        return Err(ApiError::Forbidden);
    }

    // Don't allow deleting self
    if auth.user_id == user_id {
        return Err(ApiError::Forbidden);
    }

    // Don't allow deleting user with name "admin"
    let user = match data.get_user_by_id(&user_id) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    if let Some(user) = user {
        if user.username == "admin" {
            error!("Tried to delete user with name \"admin\"");
            return Err(ApiError::Forbidden);
        }
    } else {
        return Err(ApiError::NotFound);
    }

    // Delete user
    match data.delete_user(&user_id) {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(Json(()))
}

#[derive(Deserialize)]
pub struct UserPostPasswordData {
    password: String,
}

pub async fn route_user_password_post(
    Path(user_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<UserPostPasswordData>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if auth.user_id == user_id {
        if !has_resalt_permission(&auth.perms, P_USER_PASSWORD)? {
            return Err(ApiError::Forbidden);
        }
    } else {
        #[allow(clippy::collapsible_else_if)]
        if !has_resalt_permission(&auth.perms, P_USER_LIST)? {
            return Err(ApiError::Forbidden);
        }
    }

    // Minimum password check
    if input.password.len() < 8 {
        return Err(ApiError::InvalidRequest);
    }

    // Check if user exists
    let user = match data.get_user_by_id(&user_id) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let mut user = match user {
        Some(user) => user,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    // Update password
    user.password = Some(hash_password(&input.password));

    match data.update_user(&user) {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    }

    Ok(Json(()))
}

pub async fn route_user_permissions_post(
    Path((user_id, group_id)): Path<(String, String)>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    // Check if user exists
    let user = match data.get_user_by_id(&user_id) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let user = match user {
        Some(user) => user,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    // Check if group exists
    let permission_group = match data.get_permission_group_by_id(&group_id) {
        Ok(permission_group) => permission_group,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let permission_group = match permission_group {
        Some(permission_group) => permission_group,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    // Check if user is already member of that group
    let user_is_already_member = match data.is_user_member_of_group(&user_id, &group_id) {
        Ok(user_permission) => user_permission,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    if !user_is_already_member {
        match data.insert_permission_group_user(&user.id, &permission_group.id) {
            Ok(_) => (),
            Err(e) => {
                error!("{:?}", e);
                return Err(ApiError::DatabaseError);
            }
        }

        // Update user-cached permissions
        match data.refresh_user_permissions(&user) {
            Ok(_) => (),
            Err(e) => {
                error!("{:?}", e);
                return Err(ApiError::DatabaseError);
            }
        }
    }

    Ok(Json(()))
}

pub async fn route_user_permissions_delete(
    Path((user_id, group_id)): Path<(String, String)>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    // Check if user exists
    let user = match data.get_user_by_id(&user_id) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let user = match user {
        Some(user) => user,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    let permission_group = match data.get_permission_group_by_id(&group_id) {
        Ok(permission_group) => permission_group,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let permission_group = match permission_group {
        Some(permission_group) => permission_group,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    match data.delete_permission_group_user(&user.id, &permission_group.id) {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    }

    // Update user-cached permissions
    match data.refresh_user_permissions(&user) {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    }

    Ok(Json(()))
}
