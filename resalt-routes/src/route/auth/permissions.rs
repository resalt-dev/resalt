use crate::permission::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_api::permission::{
    create_permission_group, delete_permission_group, get_permission_group_by_id,
    get_permission_group_users, get_permission_groups, update_permission_group,
};
use resalt_models::*;
use resalt_storage::Storage;
use serde::Deserialize;
use serde_json::Value;

async fn get_group(data: &Storage, group_id: &str) -> Result<impl IntoResponse, StatusCode> {
    let permission_group = match get_permission_group_by_id(data, group_id) {
        Ok(Some(permission_group)) => permission_group,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("get_group.group {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let users = match get_permission_group_users(data, group_id) {
        Ok(users) => users,
        Err(e) => {
            error!("get_group.users {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(permission_group.public(users)))
}

pub async fn route_permissions_get(
    query: Query<PaginateQuery>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_GROUP)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // Pagination
    let paginate: Paginate = query.parse_query();

    // API
    let permission_groups = match get_permission_groups(&data, paginate) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("route_permissions_get.groups {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut results: Vec<Value> = Vec::new();
    for group in permission_groups {
        let users = match get_permission_group_users(&data, &group.id) {
            Ok(users) => users,
            Err(e) => {
                error!("route_permissions_get.users {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
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
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<PermissionGroupCreateRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_GROUP)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // API
    let permission_group_id = match create_permission_group(&data, None, &input.name, None) {
        Ok(id) => id,
        Err(e) => {
            error!("route_permissions_post.create {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let permission_group = match get_permission_group_by_id(&data, &permission_group_id) {
        Ok(Some(permission_group)) => permission_group,
        Ok(None) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        Err(e) => {
            error!("route_permissions_post.group {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let permission_group_users = match get_permission_group_users(&data, &permission_group_id) {
        Ok(permission_group_users) => permission_group_users,
        Err(e) => {
            error!("route_permissions_post.users {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(permission_group.public(permission_group_users)))
}

pub async fn route_permission_get(
    Path(id): Path<String>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_GROUP)? {
        return Err(StatusCode::FORBIDDEN);
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
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<PermissionGroupUpdateRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_GROUP)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // Get permission group
    let mut permission_group = match get_permission_group_by_id(&data, &id) {
        Ok(Some(permission_group)) => permission_group,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Update permission group
    permission_group.name = input.name.clone();
    permission_group.perms = input.perms.clone(); // TODO: Validate JSON

    // API
    if let Err(e) = update_permission_group(&data, &permission_group) {
        error!("{:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    get_group(&data, &id).await
}

pub async fn route_permission_delete(
    Path(id): Path<String>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_GROUP)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // Get the group so we can return it as result
    let group = get_group(&data, &id).await?;

    // API
    if let Err(e) = delete_permission_group(&data, &id) {
        error!("{:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    Ok(group)
}
