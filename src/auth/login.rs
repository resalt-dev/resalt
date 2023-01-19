use actix_web::{web, Result};
use log::*;
use resalt_config::SConfig;
use resalt_ldap::LdapHandler;
use resalt_models::*;
use resalt_salt::SaltAPI;
use resalt_security::verify_password;
use resalt_storage::StorageImpl;

#[allow(clippy::borrowed_box)]
pub async fn renew_token_salt_token(
    data: &Box<dyn StorageImpl>,
    salt: &SaltAPI,
    user_id: &str,
    auth_token: &str,
) -> Result<AuthStatus, ApiError> {
    // Fetch username of user
    let user = match data.get_user_by_id(user_id) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(ApiError::InternalError),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Create Salt session
    let salt_token = match salt.login(&user.username, auth_token).await {
        Ok(salt_token) => salt_token,
        Err(e) => {
            error!("update_token_salt_token salt login {:?}", e);
            return Err(ApiError::InternalError);
        }
    };

    // Update token with salt session
    match data.update_authtoken_salttoken(auth_token, Some(&salt_token)) {
        Ok(_) => {}
        Err(e) => {
            error!("update_token_salt_token update_salttoken {:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(AuthStatus {
        user_id: user_id.to_owned(),
        auth_token: auth_token.to_owned(),
        salt_token: Some(salt_token),
    })
}

#[allow(clippy::borrowed_box)]
pub fn validate_auth_token(
    data: &Box<dyn StorageImpl>,
    token: &str,
) -> Result<Option<AuthStatus>, ApiError> {
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
            return Err(ApiError::DatabaseError);
        }
    };

    let session_lifespan = SConfig::auth_session_lifespan();

    if (authtoken.timestamp.timestamp() as u64) + session_lifespan
        < chrono::Utc::now().timestamp() as u64
    {
        return Ok(None);
    }

    Ok(Some(AuthStatus {
        user_id: authtoken.user_id,
        auth_token: authtoken.id,
        salt_token: match authtoken.salt_token {
            Some(v) => match serde_json::from_str::<SaltToken>(&v) {
                Ok(v) => Some(v),
                Err(e) => {
                    error!("Failed parsing authtoken.salt_token {:?}", e);
                    return Err(ApiError::InternalError);
                }
            },
            None => None,
        },
    }))
}

pub fn auth_login_classic(
    data: &web::Data<Box<dyn StorageImpl>>,
    username: &str,
    password: &str,
) -> Result<Option<User>, ApiError> {
    // Fetch user
    let user = match data.get_user_by_username(username) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Ok(None),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Check if they are local user
    if user.ldap_sync.is_some() {
        return Ok(None);
    }

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

pub async fn auth_login_ldap(
    data: &web::Data<Box<dyn StorageImpl>>,
    username: &str,
    password: &str,
) -> Result<Option<User>, ApiError> {
    let ldap_user = match LdapHandler::authenticate(username, password).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(None),
        Err(e) => {
            error!("auth_login_ldap {:?}", e);
            return Err(ApiError::LdapError);
        }
    };

    // Fetch user in Database
    let user = match data.get_user_by_username(&ldap_user.username) {
        Ok(Some(user)) => Some(user),
        Ok(None) => {
            // Create user if doesn't exist in DB, as LDAP auth was successful.
            match data.create_user(
                ldap_user.username,
                None,
                Some(ldap_user.email),
                Some(ldap_user.dn),
            ) {
                Ok(user) => Some(user),
                Err(e) => {
                    error!("{:?}", e);
                    return Err(ApiError::DatabaseError);
                }
            }
        }
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(user)
}
