use std::fmt::Display;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde_json::json;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ApiError {
    Unauthorized,                 // Missing credentials
    Forbidden,                    // Lackign permissions
    NotFound,                     // Resource not found
    NotFoundMessage(String),      // Resource not found with custom message
    InvalidRequest,               // Invalid request
    InternalError,                // Internal error
    InternalErrorMessage(String), // Internal error with custom message
    LdapError,                    // LDAP error
    DatabaseError,                // Database error
}

impl ApiError {
    fn code(&self) -> StatusCode {
        match self {
            // Login and perms
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            // Request-related
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::NotFoundMessage(_) => StatusCode::NOT_FOUND,
            ApiError::InvalidRequest => StatusCode::BAD_REQUEST,
            // Internal server errors
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InternalErrorMessage(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::LdapError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match self {
            ApiError::Unauthorized => String::from("Missing credentials"),
            ApiError::Forbidden => String::from("Insufficient permissions"),
            ApiError::NotFound => String::from("Resource not found"),
            ApiError::NotFoundMessage(str) => str.clone(),
            ApiError::InvalidRequest => String::from("Invalid request"),
            ApiError::InternalError => String::from("Internal error"),
            ApiError::InternalErrorMessage(str) => str.clone(),
            ApiError::LdapError => String::from("LDAP error"),
            ApiError::DatabaseError => String::from("Database error"),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = json! {
            {
                "error": {
                    "code": self.code().as_u16(),
                    "message": self.message(),
                },
            }
        };
        write!(f, "{}", body)
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.code()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}
