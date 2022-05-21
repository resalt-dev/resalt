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
    ldap: web::Data<LdapHandler>,
    data: web::Data<Storage>,
    salt: web::Data<SaltAPI>,
    input: web::Json<LoginRequest>,
) -> Result<impl Responder> {
    let username = input.username.to_lowercase();
    let password = input.password.clone();

    info!("Attempting login for {:?}", username);

    let mut user = match auth_login_classic(&data, &username, &password).await {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    if user.is_none() {
        debug!("User not found, testing LDAP");
        user = match auth_login_ldap(&ldap, &data, &username, &password).await {
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

    // Create token
    let authtoken = match data.create_authtoken(&user.id).await {
        Ok(authtoken) => authtoken,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    // Create Salt session
    let salt_token = match salt.login(&user.username, &authtoken.id).await {
        Ok(salt_token) => salt_token,
        Err(e) => {
            error!("route_auth_login_post salt login {:?}", e);
            return Err(api_error_internal_error());
        }
    };

    // Update token with salt session
    match data
        .update_authtoken_salttoken(&authtoken.id, &Some(salt_token))
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("route_auth_login_post update_salttoken {:?}", e);
            return Err(api_error_database());
        }
    }

    // Return
    let response = LoginResponse {
        token: authtoken.id,
    };
    Ok(web::Json(response))
}
