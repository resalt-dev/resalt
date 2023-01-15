use crate::auth::*;
use actix_web::{web, HttpResponse, Responder, Result};
use log::*;
use resalt_config::SConfig;
use resalt_models::ApiError;
use resalt_salt::RESALT_SALT_SYSTEM_SERVICE_USERNAME;
use resalt_storage::StorageImpl;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct TokenValidateRequest {
    username: String,
    password: String,
}

pub async fn route_auth_token_post(
    data: web::Data<Box<dyn StorageImpl>>,
    input: web::Form<TokenValidateRequest>,
) -> Result<impl Responder, ApiError> {
    let db = data;
    let username = input.username.to_lowercase();
    let token = input.password.clone();

    debug!("Token validation for {:?} with token {:?}", username, token);

    if username == RESALT_SALT_SYSTEM_SERVICE_USERNAME {
        if token == SConfig::salt_api_system_service_token() {
            info!("System service token OK");
            return Ok(HttpResponse::Ok().json([
                ".*".to_string(),
                "@runner".to_string(),
                "@wheel".to_string(),
            ]));
        } else {
            return Err(ApiError::Unauthorized);
        }
    }

    match validate_auth_token(&db, &token) {
        Ok(Some(auth_status)) => {
            info!("Token validated for {:?}", auth_status.user_id);

            let user = match db.get_user_by_id(&auth_status.user_id) {
                Ok(user) => match user {
                    Some(user) => user,
                    None => {
                        return Err(ApiError::Unauthorized);
                    }
                },
                Err(err) => {
                    error!("Error getting user: {:?}", err);
                    return Err(ApiError::InternalError);
                }
            };

            let perms: Result<Value, serde_json::Error> = serde_json::from_str(&user.perms);

            match perms {
                Ok(perms) => Ok(HttpResponse::Ok().json(perms)),
                Err(e) => {
                    error!("Error parsing permissions: {:?}", e);
                    Err(ApiError::InternalError)
                }
            }
        }
        Ok(None) => {
            info!("Invalid token from Salt validation");
            Err(ApiError::Unauthorized)
        }
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::DatabaseError)
        }
    }
}
