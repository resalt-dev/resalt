use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}

pub fn api_error_unauthorized() -> actix_web::Error {
    actix_web::error::ErrorUnauthorized("Missing valid credentials".to_string())
}

pub fn api_error_database() -> actix_web::Error {
    actix_web::error::ErrorInternalServerError("Database error".to_string())
}

pub fn api_error_internal_error() -> actix_web::Error {
    actix_web::error::ErrorInternalServerError("Internal error".to_string())
}

pub fn api_error_ldap() -> actix_web::Error {
    actix_web::error::ErrorInternalServerError("Internal LDAP error".to_string())
}

pub fn api_error_invalid_request() -> actix_web::Error {
    actix_web::error::ErrorBadRequest("Invalid request".to_string())
}
