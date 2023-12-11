use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::State, http::StatusCode};
use log::*;
use resalt_api::user::create_user;
use resalt_config::ResaltConfig;
use resalt_models::{StorageImpl, User};
use resalt_salt::SaltAPI;
use resalt_storage::Storage;
use serde::{Deserialize, Serialize};

use crate::login::{auth_login_classic, renew_token_salt_token};

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct LoginResponse {
    #[serde(rename = "userId")]
    user_id: String,
    token: String,
    expiry: u64,
}

pub async fn route_login_post(
    headers: HeaderMap,
    State(data): State<Storage>,
    State(salt): State<SaltAPI>,
    Json(input): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let user: User = if *ResaltConfig::AUTH_FORWARD_ENABLED {
        // Use X-Forwarded-User header as username
        let username = match headers.get("X-Forwarded-User") {
            Some(forwarded_user) => forwarded_user.to_str().unwrap().to_string(),
            None => return Err(StatusCode::UNAUTHORIZED),
        };

        // Fetch user to see if they exist
        match data.get_user_by_username(&username) {
            // User EXISTS
            Ok(Some(user)) => user,
            // User DOES NOT exist, but we are in AuthForward, so create user
            Ok(None) => {
                let email: Option<String> = None;

                // Create user
                let user = match create_user(&data, username, None, email) {
                    Ok(user) => user,
                    Err(e) => {
                        error!("Failed creating user {:?}", e);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                };

                info!("Created user: {}", &user.username);

                user
            }
            // Error from Database, which is critical, so return error
            Err(e) => {
                error!("{:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    } else {
        let username = input.username.to_lowercase();
        let password = input.password.clone();

        info!("Attempting login for {:?}", username);
        let user = match auth_login_classic(&data, &username, &password) {
            Ok(user) => user,
            Err(e) => return Err(e),
        };
        match user {
            Some(user) => user,
            None => {
                info!("User login failed for: {}", &username);
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    };

    debug!("User {} found, generating token", &user.username);

    // Refresh their user-cached permissions before doing anything else
    match data.refresh_user_permissions(&user.id) {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Create token
    let authtoken = match data.create_authtoken(user.id.clone()) {
        Ok(authtoken) => authtoken,
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Create Salt session
    match renew_token_salt_token(&data, &salt, &user.id, &authtoken.id).await {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Return
    let session_lifespan = *ResaltConfig::AUTH_SESSION_LIFESPAN;
    let response = LoginResponse {
        user_id: user.id,
        token: authtoken.id,
        expiry: (authtoken.timestamp.timestamp() as u64) + session_lifespan,
    };
    Ok(Json(response))
}
