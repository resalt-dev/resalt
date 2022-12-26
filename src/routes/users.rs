use crate::{auth::*, components::*};
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::AuthStatus;
use resalt_security::hash_password;
use resalt_storage::StorageImpl;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct UsersListGetQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_users_get(
    data: web::Data<Box<dyn StorageImpl>>,
    query: web::Query<UsersListGetQuery>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    // Validate permission
    if !has_permission(&data, &auth.user_id, P_ADMIN_USER)? {
        return Err(api_error_forbidden());
    }

    // Pagination
    let limit = query.limit;
    let offset = query.offset;

    let users = match data.list_users(limit, offset) {
        Ok(users) => users,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    // Map to - among other things - remove password
    let mut results: Vec<Value> = Vec::new();
    for user in users {
        let permission_groups = match data.list_permission_groups_by_user_id(&user.id) {
            Ok(permission_groups) => permission_groups,
            Err(e) => {
                error!("{:?}", e);
                return Err(api_error_database());
            }
        };
        results.push(user.public(permission_groups));
    }

    Ok(web::Json(results))
}

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub username: String,
    pub email: Option<String>,
    #[serde(rename = "ldapSync")]
    pub ldap_sync: Option<String>,
}

pub async fn route_users_post(
    data: web::Data<Box<dyn StorageImpl>>,
    body: web::Json<UserCreateRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    // Validate permission
    if !has_permission(&data, &auth.user_id, P_ADMIN_USER)? {
        return Err(api_error_forbidden());
    }

    // Check if username is taken
    let user = match data.get_user_by_username(&body.username) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    if user.is_some() {
        return Err(api_error_invalid_request());
    }

    // Create user
    let user = match data.create_user(
        body.username.clone(),
        None,
        body.email.clone(),
        body.ldap_sync.clone(),
    ) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    // Map to - among other things - remove password
    let permission_group = match data.list_permission_groups_by_user_id(&user.id) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };
    let user = user.public(permission_group);

    Ok(web::Json(user))
}

#[derive(Deserialize)]
pub struct UserGetInfo {
    user_id: String,
}

pub async fn route_user_get(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<UserGetInfo>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    // Validate permission
    if !has_permission(&data, &auth.user_id, P_ADMIN_USER)? {
        return Err(api_error_forbidden());
    }

    // Don't allow deleting self
    if auth.user_id == info.user_id {
        return Err(api_error_forbidden());
    }

    let user = match data.get_user_by_id(&info.user_id) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let user = match user {
        Some(user) => user,
        None => return Err(api_error_not_found()),
    };

    // Map to - among other things - remove password
    let permission_group = match data.list_permission_groups_by_user_id(&user.id) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };
    let user = user.public(permission_group);

    Ok(web::Json(user))
}

pub async fn route_user_delete(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<UserGetInfo>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    // Validate permission
    if !has_permission(&data, &auth.user_id, P_ADMIN_USER)? {
        return Err(api_error_forbidden());
    }

    // Delete user
    match data.delete_user(&info.user_id) {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    return Ok(web::Json(()));
}

#[derive(Deserialize)]
pub struct UserPostPasswordData {
    password: String,
}

pub async fn route_user_password_post(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<UserGetInfo>,
    req: HttpRequest,
    body: web::Json<UserPostPasswordData>,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    // Validate permission
    let permission_ok = has_permission(&data, &auth.user_id, P_ADMIN_USER)?
        || (auth.user_id.eq(&info.user_id)
            && has_permission(&data, &auth.user_id, P_USER_PASSWORD)?);
    if !permission_ok {
        return Err(api_error_forbidden());
    }

    // Minimum password check
    if &body.password.len() < &8 {
        return Err(api_error_invalid_request());
    }

    // Check if user exists
    let user = match data.get_user_by_id(&info.user_id) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let mut user = match user {
        Some(user) => user,
        None => {
            return Err(api_error_not_found());
        }
    };

    // Update password
    user.password = Some(hash_password(&body.password));

    match data.update_user(&user) {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    }

    return Ok(web::Json(()));
}

#[derive(Deserialize)]
pub struct UserPermissionInfo {
    user_id: String,
    group_id: String,
}

/// # Route: /users/{user_id}/permissions/{group_id} (PUT)
pub async fn route_user_permissions_post(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<UserPermissionInfo>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    // Validate permission
    if !has_permission(&data, &auth.user_id, P_ADMIN_GROUP)? {
        return Err(api_error_forbidden());
    }

    // Check if user exists
    let user = match data.get_user_by_id(&info.user_id) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let user = match user {
        Some(user) => user,
        None => {
            return Err(api_error_not_found());
        }
    };

    // Check if group exists
    let permission_group = match data.get_permission_group_by_id(&info.group_id) {
        Ok(permission_group) => permission_group,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let permission_group = match permission_group {
        Some(permission_group) => permission_group,
        None => {
            return Err(api_error_not_found());
        }
    };

    // Check if user is already member of that group
    let user_is_already_member = match data.is_user_member_of_group(&info.user_id, &info.group_id) {
        Ok(user_permission) => user_permission,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    if !user_is_already_member {
        match data.insert_permission_group_user(&user.id, &permission_group.id) {
            Ok(_) => (),
            Err(e) => {
                error!("{:?}", e);
                return Err(api_error_database());
            }
        }

        // Update user-cached permissions
        update_user_permissions_from_groups(&data, &user)?;
    }

    return Ok(web::Json(()));
}

/// # Route: /users/{user_id}/permissions/{group_id} (DELETE)
pub async fn route_user_permissions_delete(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<UserPermissionInfo>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    // Validate permission
    if !has_permission(&data, &auth.user_id, P_ADMIN_GROUP)? {
        return Err(api_error_forbidden());
    }

    // Check if user exists
    let user = match data.get_user_by_id(&info.user_id) {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let user = match user {
        Some(user) => user,
        None => {
            return Err(api_error_not_found());
        }
    };

    let permission_group = match data.get_permission_group_by_id(&info.group_id) {
        Ok(permission_group) => permission_group,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let permission_group = match permission_group {
        Some(permission_group) => permission_group,
        None => {
            return Err(api_error_not_found());
        }
    };

    match data.delete_permission_group_user(&user.id, &permission_group.id) {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    }

    // Update user-cached permissions
    update_user_permissions_from_groups(&data, &user)?;

    Ok(web::Json(()))
}
