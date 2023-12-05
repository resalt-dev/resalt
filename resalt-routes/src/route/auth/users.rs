use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_api::{
    permission::{
        add_user_to_group, get_permission_group_by_id, get_permission_groups_by_user_id,
        is_user_member_of_group, remove_user_from_group,
    },
    user::{
        create_user, delete_user, get_user_by_id, get_user_by_username, get_users, update_user,
    },
};
use resalt_models::{ApiError, AuthStatus, Paginate, PaginateQuery};
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::Deserialize;
use serde_json::Value;

pub async fn route_users_get(
    query: Query<PaginateQuery>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_USER_LIST)? {
        return Err(ApiError::Forbidden);
    }

    // Pagination
    let paginate: Paginate = query.parse_query();

    // API
    let users = get_users(&data, paginate)?;

    // Map to "public" - for among other things - remove password
    let mut results: Vec<Value> = Vec::new();
    for user in users {
        results.push(user.public(get_permission_groups_by_user_id(&data, &user.id)?));
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
    if !has_resalt_permission(&auth, P_USER_ADMIN)? {
        return Err(ApiError::Forbidden);
    }

    // Check if username is taken
    if (get_user_by_username(&data, &input.username)?).is_none() {
        return Err(ApiError::InvalidRequest);
    };

    // Create user
    let user = create_user(&data, input.username.clone(), None, input.email.clone())?;

    // Map to "public" - for among other things - remove password
    let user = user.public(get_permission_groups_by_user_id(&data, &user.id)?);

    Ok(Json(user))
}

pub async fn route_user_get(
    Path(user_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission (always allow fetching self)
    if auth.user_id != user_id && !has_resalt_permission(&auth, P_USER_LIST)? {
        return Err(ApiError::Forbidden);
    }

    // API
    let user = match get_user_by_id(&data, &user_id)? {
        Some(user) => user,
        None => return Err(ApiError::NotFound),
    };

    // Map to "public" - for among other things - remove password
    let user = user.public(get_permission_groups_by_user_id(&data, &user.id)?);

    Ok(Json(user))
}

pub async fn route_user_delete(
    Path(user_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_USER_ADMIN)? {
        return Err(ApiError::Forbidden);
    }

    // Don't allow deleting self
    if auth.user_id == user_id {
        warn!("Tried to delete self: {}", user_id);
        return Err(ApiError::InvalidRequest);
    }

    // Delete user
    delete_user(&data, &user_id)?;

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
        if !has_resalt_permission(&auth, P_USER_PASSWORD)? {
            return Err(ApiError::Forbidden);
        }
    } else {
        #[allow(clippy::collapsible_else_if)]
        if !has_resalt_permission(&auth, P_USER_ADMIN)? {
            return Err(ApiError::Forbidden);
        }
    }

    // Minimum password check
    if input.password.len() < 8 {
        return Err(ApiError::InvalidRequest);
    }

    // Check if user exists
    let mut user = match get_user_by_id(&data, &user_id)? {
        Some(user) => user,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    // Update password
    user.password = Some(hash_password(&input.password));
    update_user(&data, &user)?;

    Ok(Json(()))
}

pub async fn route_user_permissions_post(
    Path((user_id, group_id)): Path<(String, String)>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    // Check if user exists
    let user = match get_user_by_id(&data, &user_id)? {
        Some(user) => user,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    // Check if group exists
    let permission_group = match get_permission_group_by_id(&data, &group_id)? {
        Some(permission_group) => permission_group,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    // Add user to group
    if !is_user_member_of_group(&data, &user_id, &group_id)? {
        add_user_to_group(&data, &user.id, &permission_group.id)?;
    }

    Ok(Json(()))
}

pub async fn route_user_permissions_delete(
    Path((user_id, group_id)): Path<(String, String)>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    // Check if user exists
    let user = match get_user_by_id(&data, &user_id)? {
        Some(user) => user,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    // Check if group exists
    let permission_group = match get_permission_group_by_id(&data, &group_id)? {
        Some(permission_group) => permission_group,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    // Remove user from group
    if is_user_member_of_group(&data, &user_id, &group_id)? {
        remove_user_from_group(&data, &user.id, &permission_group.id)?;
    }

    Ok(Json(()))
}
