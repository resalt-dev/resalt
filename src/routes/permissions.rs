use crate::prelude::*;
use actix_web::{web, Responder, Result};
use log::*;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct PermissionGroupsListGetQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}
/// # Route: /permissions (GET)
pub async fn route_permissions_get(
    data: web::Data<Storage>,
    query: web::Query<PermissionGroupsListGetQuery>,
) -> Result<impl Responder> {
    // Pagination
    let limit = query.limit;
    let offset = query.offset;

    let permission_groups = match data.list_permission_groups(limit, offset) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let mut results: Vec<Value> = Vec::new();
    for group in permission_groups {
        let users = match data.list_users_by_permission_group_id(&group.id) {
            Ok(users) => users,
            Err(e) => {
                error!("{:?}", e);
                return Err(api_error_database());
            }
        };
        results.push(group.public(users));
    }
    Ok(web::Json(results))
}

#[derive(Deserialize)]
pub struct PermissionGroupCreateRequest {
    pub name: String,
}

/// # Route: /permissions (POST)
pub async fn route_permissions_post(
    data: web::Data<Storage>,
    input: web::Json<PermissionGroupCreateRequest>,
) -> Result<impl Responder> {
    let permission_group = match data.insert_permission_group(&input.name) {
        Ok(permission_group) => permission_group,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };
    Ok(web::Json(permission_group))
}

#[derive(Deserialize)]
pub struct PermissionInfo {
    id: String,
}
/// # Route: /permissions/{id} (GET)
pub async fn route_permission_get(
    data: web::Data<Storage>,
    info: web::Path<PermissionInfo>,
) -> Result<impl Responder> {
    let permission_group = match data.get_permission_group_by_id(&info.id) {
        Ok(permission_group) => permission_group,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let permission_group = match permission_group {
        Some(permission_group) => permission_group,
        None => return Err(api_error_not_found()),
    };

    let users = match data.list_users_by_permission_group_id(&info.id) {
        Ok(users) => users,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };
    Ok(web::Json(permission_group.public(users)))
}

#[derive(Deserialize)]
pub struct PermissionGroupUpdateRequest {
    pub name: String,
    pub perms: String, // JSON encoded array
    #[serde(rename = "ldapSync")]
    pub ldap_sync: Option<String>,
}
/// # Route: /permissions/{id} (PUT)
pub async fn route_permission_update(
    data: web::Data<Storage>,
    info: web::Path<PermissionInfo>,
    input: web::Json<PermissionGroupUpdateRequest>,
) -> Result<impl Responder> {
    let permission_group = match data.get_permission_group_by_id(&info.id) {
        Ok(permission_group) => permission_group,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let mut permission_group = match permission_group {
        Some(permission_group) => permission_group,
        None => {
            return Err(api_error_not_found());
        }
    };

    permission_group.name = input.name.clone();
    permission_group.perms = input.perms.clone(); // TODO: Validate JSON
    permission_group.ldap_sync = input.ldap_sync.clone(); // TODO: Validate LDAP

    match data.update_permission_group(&permission_group) {
        Ok(()) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    Ok(web::Json(()))
}

/// # Route: /permissions/{id} (DELETE)
pub async fn route_permission_delete(
    data: web::Data<Storage>,
    info: web::Path<PermissionInfo>,
) -> Result<impl Responder> {
    let permission_group = match data.delete_permission_group(&info.id) {
        Ok(permission_group) => permission_group,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };
    Ok(web::Json(permission_group))
}
