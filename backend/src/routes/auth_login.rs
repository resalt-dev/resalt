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

    let user = match data.get_user_by_username(&username).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(api_error_unauthorized()),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };
    let recent_authtokens = match data.list_authtokens_by_user_id(&user.id).await {
        Ok(authtokens) => authtokens,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let max_tries = SConfig::user_login_max_tries();
    if recent_authtokens.len() >= max_tries as usize {
        warn!("Too many login attempts for user {:?}", user.id);
        return Err(api_error_user_ratelimited());
    }

    let user_pass = match user.password {
        Some(user_pass) => user_pass,
        None => {
            warn!("Attempted login with user {:?} has no password", user.id);
            return Err(api_error_unauthorized());
        }
    };

    if !verify_password(&password, &user_pass) {
        match data.create_authtoken(&user.id, false).await {
            Ok(_) => {}
            Err(e) => {
                error!("{:?}", e);
                return Err(api_error_database());
            }
        }
        return Err(api_error_unauthorized());
    }

    let authtoken = match data.create_authtoken(&user.id, true).await {
        Ok(authtoken) => authtoken,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_unauthorized());
        }
    };

    let salt_token = match salt.login(&username, &authtoken.id).await {
        Ok(salt_token) => salt_token,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_unauthorized());
        }
    };

    match data
        .update_authtoken_salttoken(&authtoken.id, &Some(salt_token))
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    }

    let response = LoginResponse {
        token: authtoken.id,
    };
    Ok(web::Json(response))
}
