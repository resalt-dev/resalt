use axum::http::StatusCode;
use log::*;
use resalt_config::ResaltConfig;
use resalt_models::*;
use resalt_salt::SaltAPI;
use resalt_security::verify_password;
use resalt_storage::Storage;
use serde_json::from_str;

pub async fn renew_token_salt_token(
    data: &Storage,
    salt: &SaltAPI,
    user_id: &str,
    auth_token: &str,
) -> Result<AuthStatus, StatusCode> {
    // Fetch username of user
    let user = match data.get_user_by_id(user_id) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Create Salt session
    let salt_token = match salt.login(&user.username, auth_token).await {
        Ok(salt_token) => salt_token,
        Err(e) => {
            error!("update_token_salt_token salt login {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Update token with salt session
    match data.update_authtoken_salttoken(auth_token, Some(&salt_token)) {
        Ok(_) => {}
        Err(e) => {
            error!("update_token_salt_token update_salttoken {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(AuthStatus {
        user_id: user_id.to_owned(),
        perms: user.perms,
        auth_token: auth_token.to_owned(),
        salt_token: Some(salt_token),
    })
}

pub fn validate_auth_token(data: &Storage, token: &str) -> Result<Option<AuthStatus>, StatusCode> {
    if token.len() < 20 {
        return Ok(None);
    }

    let authtoken = match data.get_authtoken_by_id(token) {
        Ok(authtoken) => match authtoken {
            Some(authtoken) => authtoken,
            None => return Ok(None),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let session_lifespan = *ResaltConfig::AUTH_SESSION_LIFESPAN;

    if (authtoken.timestamp.timestamp() as u64) + session_lifespan
        < ResaltTime::now().timestamp() as u64
    {
        return Ok(None);
    }

    let user = match data.get_user_by_id(&authtoken.user_id) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Some(AuthStatus {
        user_id: authtoken.user_id,
        perms: user.perms,
        auth_token: authtoken.id,
        salt_token: match authtoken.salt_token {
            Some(v) => match from_str::<SaltToken>(&v) {
                Ok(v) => Some(v),
                Err(e) => {
                    error!("Failed parsing authtoken.salt_token {:?}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            },
            None => None,
        },
    }))
}

pub fn auth_login_classic(
    data: &Storage,
    username: &str,
    password: &str,
) -> Result<Option<User>, StatusCode> {
    // Fetch user
    let user = match data.get_user_by_username(username) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Ok(None),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Check password
    let user_pass = match &user.password {
        Some(user_pass) => user_pass,
        None => return Ok(None),
    };
    if !verify_password(password, user_pass) {
        return Ok(None);
    }

    Ok(Some(user))
}
