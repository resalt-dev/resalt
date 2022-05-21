use crate::prelude::*;
use actix_web::{web, Result};
use log::*;

pub async fn validate_auth_token(
    data: &Storage,
    token: &str,
) -> Result<Option<AuthStatus>, actix_web::Error> {
    if token.len() < 20 {
        return Ok(None);
    }

    let authtoken = match data.get_authtoken_by_id(&token).await {
        Ok(authtoken) => match authtoken {
            Some(authtoken) => authtoken,
            None => return Ok(None),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let session_lifespan = SConfig::auth_user_session_lifespan();

    if (authtoken.timestamp.timestamp() as u64) + session_lifespan
        < chrono::Utc::now().timestamp() as u64
    {
        return Ok(None);
    }

    return Ok(Some(AuthStatus {
        user_id: authtoken.user_id,
        salt_token: match authtoken.salt_token {
            Some(v) => match serde_json::from_str::<SaltToken>(&v) {
                Ok(v) => Some(v),
                Err(e) => {
                    error!("Failed parsing authtoken.salt_token {:?}", e);
                    return Err(api_error_internal_error());
                }
            },
            None => None,
        },
    }));
}

pub async fn auth_login_classic(
    data: &web::Data<Storage>,
    username: &String,
    password: &String,
) -> Result<Option<User>, actix_web::Error> {
    // Fetch user
    let user = match data.get_user_by_username(username).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return Ok(None),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    // Fetch password
    let user_pass = match &user.password {
        Some(user_pass) => user_pass,
        None => return Ok(None),
    };

    // Check password
    if !verify_password(&password, user_pass) {
        return Ok(None);
    }

    return Ok(Some(user));
}

pub async fn auth_login_ldap(
    ldap: &web::Data<LdapHandler>,
    data: &web::Data<Storage>,
    username: &String,
    password: &String,
) -> Result<Option<User>, actix_web::Error> {
    let uid = match ldap.authenticate(username, password).await {
        Ok(Some(uid)) => uid,
        Ok(None) => return Ok(None),
        Err(e) => {
            error!("auth_login_ldap {:?}", e);
            return Err(api_error_ldap());
        }
    };

    let mut user = match data.get_user_by_username(&uid).await {
        Ok(user) => user,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    // Create user if doesn't exist, as LDAP passed.
    if user.is_none() {
        user = match data.create_user(&uid, None).await {
            Ok(user) => Some(user),
            Err(e) => {
                error!("{:?}", e);
                return Err(api_error_database());
            }
        };
    }

    return Ok(user);
}
