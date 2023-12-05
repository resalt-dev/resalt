use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use log::*;
use resalt_auth::auth_login_classic;
use resalt_auth::renew_token_salt_token;
use resalt_config::ResaltConfig;
use resalt_models::{ApiError, User};
use resalt_salt::SaltAPI;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

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
    State(data): State<Box<dyn StorageImpl>>,
    State(salt): State<SaltAPI>,
    Json(input): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user: User = if *ResaltConfig::AUTH_FORWARD_ENABLED {
        // Use X-Forwarded-User header as username
        let username = match headers.get("X-Forwarded-User") {
            Some(forwarded_user) => forwarded_user.to_str().unwrap().to_string(),
            None => return Err(ApiError::Unauthorized),
        };

        // Fetch user to see if they exist
        match data.get_user_by_username(&username) {
            // User EXISTS
            Ok(Some(user)) => user,
            // User DOES NOT exist, but we are in AuthForward, so create user
            Ok(None) => {
                let email: Option<String> = None;

                // Create user
                let user = match data.create_user(username, None, email) {
                    Ok(user) => user,
                    Err(e) => {
                        error!("Failed creating user {:?}", e);
                        return Err(ApiError::DatabaseError);
                    }
                };

                info!("Created user: {}", &user.username);

                user
            }
            // Error from Database, which is critical, so return error
            Err(e) => {
                error!("{:?}", e);
                return Err(ApiError::DatabaseError);
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
                return Err(ApiError::Unauthorized);
            }
        }
    };

    debug!("User {} found, generating token", &user.username);

    // Refresh their user-cached permissions before doing anything else
    match data.refresh_user_permissions(&user.id) {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    }

    // Create token
    let authtoken = match data.create_authtoken(user.id.clone()) {
        Ok(authtoken) => authtoken,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Create Salt session
    match renew_token_salt_token(&data, &salt, &user.id, &authtoken.id).await {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
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
