use crate::auth::*;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::*;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

async fn get_group(
    data: &web::Data<Box<dyn StorageImpl>>,
    group_id: &str,
) -> Result<impl Responder, ApiError> {
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
    Ok(web::Json(permission_group.public(users)))
}

#[derive(Deserialize)]
pub struct PermissionGroupsListGetQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}
/// # Route: /permissions (GET)
pub async fn route_permissions_get(
    data: web::Data<Box<dyn StorageImpl>>,
    query: web::Query<PermissionGroupsListGetQuery>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_ADMIN_GROUP)? {
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
    Ok(web::Json(results))
}

#[derive(Deserialize)]
pub struct PermissionGroupCreateRequest {
    pub name: String,
}

/// # Route: /permissions (POST)
pub async fn route_permissions_post(
    data: web::Data<Box<dyn StorageImpl>>,
    input: web::Json<PermissionGroupCreateRequest>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    let permission_group_id = match data.create_permission_group(&input.name) {
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
    Ok(web::Json(permission_group.public(permission_group_users)))
}

#[derive(Deserialize)]
pub struct PermissionInfo {
    id: String,
}
/// # Route: /permissions/{id} (GET)
pub async fn route_permission_get(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<PermissionInfo>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    get_group(&data, &info.id).await
}

#[derive(Deserialize)]
pub struct PermissionGroupUpdateRequest {
    pub name: String,
    pub perms: String, // JSON encoded array
    // allow ldapSync(string) to be null
    #[serde(rename = "ldapSync", deserialize_with = "deserialize_null")]
    pub ldap_sync: Option<String>,
}
/// # Route: /permissions/{id} (PUT)
pub async fn route_permission_update(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<PermissionInfo>,
    input: web::Json<PermissionGroupUpdateRequest>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    // Get permission group
    let permission_group = match data.get_permission_group_by_id(&info.id) {
        Ok(permission_group) => permission_group,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    let mut permission_group = match permission_group {
        Some(permission_group) => permission_group,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    // Update permission group
    permission_group.name = input.name.clone();
    permission_group.perms = input.perms.clone(); // TODO: Validate JSON
    permission_group.ldap_sync = input.ldap_sync.clone(); // TODO: Validate LDAP

    match data.update_permission_group(&permission_group) {
        Ok(()) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Update members
    match data.list_users_by_permission_group_id(&info.id) {
        Ok(users) => {
            for user in users {
                update_user_permissions_from_groups(&data, &user)?;
            }
        }
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    get_group(&data, &info.id).await
}

/// # Route: /permissions/{id} (DELETE)
pub async fn route_permission_delete(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<PermissionInfo>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_ADMIN_GROUP)? {
        return Err(ApiError::Forbidden);
    }

    // Get the group so we can return it as result
    let group = get_group(&data, &info.id).await?;

    // Get list of all users, so we can update them after deleting the group
    let users = match data.list_users_by_permission_group_id(&info.id) {
        Ok(users) => users,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Delete group
    match &data.delete_permission_group(&info.id) {
        Ok(()) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Update ex-members
    for user in users {
        update_user_permissions_from_groups(&data, &user)?;
    }

    Ok(group)
}

fn deserialize_null<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(None))
}
