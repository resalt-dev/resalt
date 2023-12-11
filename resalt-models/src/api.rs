use std::fmt::Display;

use axum::http::StatusCode;
use serde_json::json;

#[derive(Clone, Debug)]
#[deprecated]
pub enum ApiError {
    Unauthorized,   // Missing credentials
    Forbidden,      // Lackign permissions
    NotFound,       // Resource not found
    InvalidRequest, // Invalid request
    InternalError,  // Internal error
    DatabaseError,  // Database error
}

impl ApiError {
    fn code(&self) -> StatusCode {
        match self {
            // Login and perms
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            // Request-related
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::InvalidRequest => StatusCode::BAD_REQUEST,
            // Internal server errors
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match self {
            ApiError::Unauthorized => String::from("Unauthorized - Invalid credentials"),
            ApiError::Forbidden => String::from("Forbidden - Insufficient permissions"),
            ApiError::NotFound => String::from("Not found"),
            ApiError::InvalidRequest => String::from("Invalid request"),
            ApiError::InternalError => {
                String::from("Internal error, please contact the system administrator")
            }
            ApiError::DatabaseError => {
                String::from("Database error, please contact the system administrator")
            }
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

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.code(), self.message()).into_response()
    }
}
