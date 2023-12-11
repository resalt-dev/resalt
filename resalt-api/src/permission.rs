use axum::http::StatusCode;
use log::error;
use resalt_models::{Paginate, PermissionGroup, StorageImpl, User};
use resalt_storage::Storage;

pub fn get_permission_groups(
    data: &Storage,
    paginate: Paginate,
) -> Result<Vec<PermissionGroup>, StatusCode> {
    data.list_permission_groups(paginate).map_err(|e| {
        error!("api.get_permission_groups {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn get_permission_groups_by_user_id(
    data: &Storage,
    user_id: &str,
) -> Result<Vec<PermissionGroup>, StatusCode> {
    data.list_permission_groups_by_user_id(user_id)
        .map_err(|e| {
            error!("api.get_permission_groups_by_user_id {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub fn get_permission_group_by_id(
    data: &Storage,
    group_id: &str,
) -> Result<Option<PermissionGroup>, StatusCode> {
    data.get_permission_group_by_id(group_id).map_err(|e| {
        error!("api.get_permission_group_by_id {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn get_permission_group_users(data: &Storage, group_id: &str) -> Result<Vec<User>, StatusCode> {
    data.list_users_by_permission_group_id(group_id)
        .map_err(|e| {
            error!("api.get_permission_group_users {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub fn create_permission_group(
    data: &Storage,
    id: Option<String>,
    name: &str,
    perms: Option<String>,
) -> Result<String, StatusCode> {
    data.create_permission_group(id, name, perms).map_err(|e| {
        error!("api.create_group {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn update_permission_group(data: &Storage, group: &PermissionGroup) -> Result<(), StatusCode> {
    data.update_permission_group(group).map_err(|e| {
        error!("api.update_group {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Update members
    match get_permission_group_users(data, &group.id) {
        Ok(users) => {
            for user in users {
                if let Err(e) = data.refresh_user_permissions(&user.id) {
                    error!("{:?}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
            Ok(())
        }
        Err(e) => {
            error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn delete_permission_group(data: &Storage, group_id: &str) -> Result<(), StatusCode> {
    let users = get_permission_group_users(data, group_id)?;

    for user in &users {
        data.delete_permission_group_user(&user.id, group_id)
            .map_err(|e| {
                error!("api.delete_group_user {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    data.delete_permission_group(group_id).map_err(|e| {
        error!("api.delete_group {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Update ex-members
    for user in users {
        if let Err(e) = data.refresh_user_permissions(&user.id) {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(())
}

pub fn is_user_member_of_group(
    data: &Storage,
    user_id: &str,
    group_id: &str,
) -> Result<bool, StatusCode> {
    data.is_user_member_of_group(user_id, group_id)
        .map_err(|e| {
            error!("api.is_user_member_of_group {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub fn add_user_to_group(data: &Storage, user_id: &str, group_id: &str) -> Result<(), StatusCode> {
    data.insert_permission_group_user(user_id, group_id)
        .map_err(|e| {
            error!("api.add_user_to_group {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Update user-cached permissions
    match data.refresh_user_permissions(user_id) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn remove_user_from_group(
    data: &Storage,
    user_id: &str,
    group_id: &str,
) -> Result<(), StatusCode> {
    data.delete_permission_group_user(user_id, group_id)
        .map_err(|e| {
            error!("api.remove_user_from_group {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Update user-cached permissions
    match data.refresh_user_permissions(user_id) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
