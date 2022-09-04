use crate::prelude::*;
use actix_web::{web, Responder, Result};
use log::*;
use serde::Deserialize;

/// # Route: /permissions (GET)
pub async fn route_permissions_get(data: web::Data<Storage>) -> Result<impl Responder> {
    let permission_groups = match data.list_permission_groups() {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };
    Ok(web::Json(permission_groups))
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
#[derive(Deserialize)]
pub struct PermissionGroupUpdateRequest {
    pub name: String,
    pub perms: String, // JSON encoded array
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
