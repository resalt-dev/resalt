use log::error;
use resalt_models::{ApiError, Paginate, User};
use resalt_storage::StorageImpl;

pub fn create_user(
    data: &Box<dyn StorageImpl>,
    username: String,
    password: Option<String>,
    email: Option<String>,
) -> Result<User, ApiError> {
    data.create_user(username, password, email).map_err(|e| {
        error!("api.create_user {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn get_users(data: &Box<dyn StorageImpl>, paginate: Paginate) -> Result<Vec<User>, ApiError> {
    data.list_users(paginate).map_err(|e| {
        error!("api.get_users {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn get_user_by_id(
    data: &Box<dyn StorageImpl>,
    user_id: &str,
) -> Result<Option<User>, ApiError> {
    data.get_user_by_id(user_id).map_err(|e| {
        error!("api.get_user_by_id {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn get_user_by_username(
    data: &Box<dyn StorageImpl>,
    username: &str,
) -> Result<Option<User>, ApiError> {
    data.get_user_by_username(username).map_err(|e| {
        error!("api.get_user_by_username {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn update_user(data: &Box<dyn StorageImpl>, user: &User) -> Result<(), ApiError> {
    data.update_user(user).map_err(|e| {
        error!("api.update_user {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn delete_user(data: &Box<dyn StorageImpl>, user_id: &str) -> Result<(), ApiError> {
    data.delete_user(user_id).map_err(|e| {
        error!("api.delete_user {:?}", e);
        ApiError::DatabaseError
    })
}
