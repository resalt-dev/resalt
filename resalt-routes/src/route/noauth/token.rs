use axum::{extract::State, http::StatusCode, response::IntoResponse, Form, Json};
use log::*;
use resalt_config::ResaltConfig;
use resalt_models::StorageImpl;
use resalt_salt::RESALT_SALT_SYSTEM_SERVICE_USERNAME;
use resalt_storage::Storage;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::login::validate_auth_token;

#[derive(Deserialize, Debug)]
pub struct TokenValidateRequest {
    username: String,
    password: String,
}

pub async fn route_token_post(
    State(data): State<Storage>,
    Form(input): Form<TokenValidateRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = data;
    let username = input.username.to_lowercase();
    let token = input.password.clone();

    debug!("Token validation for {:?} with token {:?}", username, token);

    if username == RESALT_SALT_SYSTEM_SERVICE_USERNAME {
        if token == *ResaltConfig::SALT_API_SYSTEM_SERVICE_TOKEN {
            info!("System service token OK");
            return Ok(Json(json!([
                ".*".to_string(),
                "@runner".to_string(),
                "@wheel".to_string(),
            ])));
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    match validate_auth_token(&db, &token) {
        Ok(Some(auth_status)) => {
            info!("Token validated for {:?}", auth_status.user_id);

            let user = match db.get_user_by_id(&auth_status.user_id) {
                Ok(user) => match user {
                    Some(user) => user,
                    None => {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                },
                Err(err) => {
                    error!("Error getting user: {:?}", err);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };

            let perms: Result<Value, serde_json::Error> = serde_json::from_str(&user.perms);

            match perms {
                Ok(perms) => Ok(Json(perms)),
                Err(e) => {
                    error!("Error parsing permissions: {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => {
            info!("Invalid token from Salt validation");
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
