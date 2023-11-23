use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_models::*;
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::Deserialize;
use serde_json::Value;

#[allow(clippy::borrowed_box)]
async fn get_group(
    data: &Box<dyn StorageImpl>,
    group_id: &str,
) -> Result<impl IntoResponse, ApiError> {
    let permission_group = match data.get_permission_group_by_id(group_id) {
        Ok(permission_group) => permission_group,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let permission_group = match permission_group {
        Some(permission_group) => permission_group,
        None => return Err(ApiError::NotFound),
    };

    let users = match data.list_users_by_permission_group_id(group_id) {
        Ok(users) => users,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    Ok(Json(permission_group.public(users)))
}

#[derive(Deserialize)]
pub struct PermissionGroupsListGetQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_permissions_get(
    query: Query<PermissionGroupsListGetQuery>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    // Pagination
    let limit = query.limit;
    let offset = query.offset;

    let permission_groups = match data.list_permission_groups(limit, offset) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let mut results: Vec<Value> = Vec::new();
    for group in permission_groups {
        let users = match data.list_users_by_permission_group_id(&group.id) {
            Ok(users) => users,
            Err(e) => {
                error!("{:?}", e);
                return Err(ApiError::DatabaseError);
            }
        };
        results.push(group.public(users));
    }
    Ok(Json(results))
}

#[derive(Deserialize)]
pub struct PermissionGroupCreateRequest {
    pub name: String,
}

pub async fn route_permissions_post(
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<PermissionGroupCreateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    let permission_group_id = match data.create_permission_group(None, &input.name, None) {
        Ok(id) => id,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    let permission_group = match data.get_permission_group_by_id(&permission_group_id) {
        Ok(permission_group) => match permission_group {
            Some(permission_group) => permission_group,
            None => return Err(ApiError::DatabaseError),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    let permission_group_users = match data.list_users_by_permission_group_id(&permission_group_id)
    {
        Ok(permission_group_users) => permission_group_users,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    Ok(Json(permission_group.public(permission_group_users)))
}

pub async fn route_permission_get(
    Path(id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    get_group(&data, &id).await
}

#[derive(Deserialize)]
pub struct PermissionGroupUpdateRequest {
    pub name: String,
    pub perms: String, // JSON encoded array
}

pub async fn route_permission_put(
    Path(id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<PermissionGroupUpdateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    // Get permission group
    let mut permission_group = match data.get_permission_group_by_id(&id) {
        Ok(Some(permission_group)) => permission_group,
        Ok(None) => return Err(ApiError::NotFound),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Update permission group
    permission_group.name = input.name.clone();
    permission_group.perms = input.perms.clone(); // TODO: Validate JSON

    match data.update_permission_group(&permission_group) {
        Ok(()) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Update members
    match data.list_users_by_permission_group_id(&id) {
        Ok(users) => {
            for user in users {
                match data.refresh_user_permissions(&user) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("{:?}", e);
                        return Err(ApiError::DatabaseError);
                    }
                }
            }
        }
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    get_group(&data, &id).await
}

pub async fn route_permission_delete(
    Path(id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    // Get the group so we can return it as result
    let group = get_group(&data, &id).await?;

    // Get list of all users, so we can update them after deleting the group
    let users = match data.list_users_by_permission_group_id(&id) {
        Ok(users) => users,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Delete group
    match &data.delete_permission_group(&id) {
        Ok(()) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Update ex-members
    for user in users {
        match data.refresh_user_permissions(&user) {
            Ok(_) => (),
            Err(e) => {
                error!("{:?}", e);
                return Err(ApiError::DatabaseError);
            }
        }
    }

    Ok(group)
}
