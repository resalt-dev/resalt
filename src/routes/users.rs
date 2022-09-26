use crate::prelude::*;
use actix_web::{web, Responder, Result};
use log::*;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct UsersListGetQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_users_get(
    data: web::Data<Storage>,
    query: web::Query<UsersListGetQuery>,
) -> Result<impl Responder> {
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
pub struct UserGetInfo {
    user_id: String,
}

pub async fn route_user_get(
    data: web::Data<Storage>,
    info: web::Path<UserGetInfo>,
) -> Result<impl Responder> {
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

#[derive(Deserialize)]
pub struct UserPermissionInfo {
    user_id: String,
    group_id: String,
}

/// # Route: /users/{user_id}/permission/{group_id} (PUT)
pub async fn route_user_permission_post(
    data: web::Data<Storage>,
    info: web::Path<UserPermissionInfo>,
) -> Result<impl Responder> {
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

/// # Route: /users/{user_id}/permission/{group_id} (DELETE)
pub async fn route_user_permission_delete(
    data: web::Data<Storage>,
    info: web::Path<UserPermissionInfo>,
) -> Result<impl Responder> {
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
