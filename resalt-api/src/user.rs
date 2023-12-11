use axum::http::StatusCode;
use log::error;
use resalt_models::{Paginate, StorageImpl, User};
use resalt_security::hash_password;
use resalt_storage::Storage;

pub fn create_user(
    data: &Storage,
    username: String,
    password: Option<String>,
    email: Option<String>,
) -> Result<User, StatusCode> {
    data.create_user_hashed(
        None,
        username,
        password.map(|v| hash_password(&v)),
        "[]".to_string(),
        None,
        email,
    )
    .map_err(|e| {
        error!("api.create_user {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn get_users(data: &Storage, paginate: Paginate) -> Result<Vec<User>, StatusCode> {
    data.list_users(paginate).map_err(|e| {
        error!("api.get_users {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn get_user_by_id(data: &Storage, user_id: &str) -> Result<Option<User>, StatusCode> {
    data.get_user_by_id(user_id).map_err(|e| {
        error!("api.get_user_by_id {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn get_user_by_username(data: &Storage, username: &str) -> Result<Option<User>, StatusCode> {
    data.get_user_by_username(username).map_err(|e| {
        error!("api.get_user_by_username {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn update_user(data: &Storage, user: &User) -> Result<(), StatusCode> {
    data.update_user(user).map_err(|e| {
        error!("api.update_user {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn delete_user(data: &Storage, user_id: &str) -> Result<(), StatusCode> {
    data.delete_user(user_id).map_err(|e| {
        error!("api.delete_user {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
