use log::error;
use resalt_models::{ApiError, Paginate, PermissionGroup, User};
use resalt_storage::StorageImpl;

pub fn get_permission_groups(
    data: &Box<dyn StorageImpl>,
    paginate: Paginate,
) -> Result<Vec<PermissionGroup>, ApiError> {
    data.list_permission_groups(paginate).map_err(|e| {
        error!("api.get_permission_groups {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn get_permission_groups_by_user_id(
    data: &Box<dyn StorageImpl>,
    user_id: &str,
) -> Result<Vec<PermissionGroup>, ApiError> {
    data.list_permission_groups_by_user_id(user_id)
        .map_err(|e| {
            error!("api.get_permission_groups_by_user_id {:?}", e);
            ApiError::DatabaseError
        })
}

pub fn get_permission_group_by_id(
    data: &Box<dyn StorageImpl>,
    group_id: &str,
) -> Result<Option<PermissionGroup>, ApiError> {
    data.get_permission_group_by_id(group_id).map_err(|e| {
        error!("api.get_permission_group_by_id {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn get_permission_group_users(
    data: &Box<dyn StorageImpl>,
    group_id: &str,
) -> Result<Vec<User>, ApiError> {
    data.list_users_by_permission_group_id(group_id)
        .map_err(|e| {
            error!("api.get_permission_group_users {:?}", e);
            ApiError::DatabaseError
        })
}

pub fn create_permission_group(
    data: &Box<dyn StorageImpl>,
    id: Option<String>,
    name: &str,
    perms: Option<String>,
) -> Result<String, ApiError> {
    data.create_permission_group(id, name, perms).map_err(|e| {
        error!("api.create_group {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn update_permission_group(
    data: &Box<dyn StorageImpl>,
    group: &PermissionGroup,
) -> Result<(), ApiError> {
    data.update_permission_group(group).map_err(|e| {
        error!("api.update_group {:?}", e);
        ApiError::DatabaseError
    })?;

    // Update members
    match get_permission_group_users(data, &group.id) {
        Ok(users) => {
            for user in users {
                if let Err(e) = data.refresh_user_permissions(&user) {
                    error!("{:?}", e);
                    return Err(ApiError::DatabaseError);
                }
            }
        }
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(())
}
