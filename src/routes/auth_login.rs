use crate::prelude::*;
use actix_web::{web, Responder, Result};
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct LoginResponse {
    token: String,
}

pub async fn route_auth_login_post(
    data: web::Data<Storage>,
    salt: web::Data<SaltAPI>,
    input: web::Json<LoginRequest>,
) -> Result<impl Responder> {
    let username = input.username.to_lowercase();
    let password = input.password.clone();

    info!("Attempting login for {:?}", username);

    let mut user = match auth_login_classic(&data, &username, &password) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    if user.is_none() {
        debug!("User not found, testing LDAP");
        user = match auth_login_ldap(&data, &username, &password).await {
            Ok(user) => user,
            Err(e) => return Err(e),
        };
    }

    let user = match user {
        Some(user) => user,
        None => {
            return Err(api_error_unauthorized());
        }
    };

    debug!("User {} found, generating token", &user.username);

    // Refresh their user-cached permissions before doing anything else
    update_user_permissions_from_groups(&data, &user)?;

    // Create token
    let authtoken = match data.create_authtoken(user.id.clone()) {
        Ok(authtoken) => authtoken,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    // Create Salt session
    match update_token_salt_token(&data, &salt, &user.id, &authtoken.id).await {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    // Return
    let response = LoginResponse {
        token: authtoken.id,
    };
    Ok(web::Json(response))
}
