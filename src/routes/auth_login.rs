use actix_web::{web, HttpRequest, Responder, Result};
use log::*;
use resalt_config::SConfig;
use resalt_ldap::LdapHandler;
use resalt_ldap::LdapUser;
use resalt_models::{ApiError, User};
use resalt_salt::SaltAPI;
use resalt_security::refresh_user_permissions;
use resalt_security::sync_ldap_groups;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

use crate::auth::auth_login_classic;
use crate::auth::auth_login_ldap;
use crate::auth::renew_token_salt_token;

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

pub async fn route_auth_login_post(
    data: web::Data<Box<dyn StorageImpl>>,
    salt: web::Data<SaltAPI>,
    input: web::Json<LoginRequest>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user: User = if SConfig::auth_forward_enabled() {
        // Use X-Forwarded-User header as username
        let mut username = match req.headers().get("X-Forwarded-User") {
            Some(forwarded_user) => forwarded_user.to_str().unwrap().to_string(),
            None => return Err(ApiError::Unauthorized),
        };

        // Fetch user to see if they exist
        match data.get_user_by_username(&username) {
            // User EXISTS
            Ok(Some(user)) => user,
            // User DOES NOT exist, but we are in AuthForward, so create user
            Ok(None) => {
                // Check if we are in LDAP or not
                let mut ldap_user: Option<LdapUser> = None;
                let mut email: Option<String> = None;
                let mut ldap_sync: Option<String> = None;
                if SConfig::auth_ldap_enabled() {
                    // Fetch user from LDAP
                    ldap_user = match LdapHandler::lookup_user_by_username(&username).await {
                        // User was found in LDAP, lets link
                        Ok(Some(ldap_user)) => {
                            username = ldap_user.username.clone();
                            email = Some(ldap_user.email.clone());
                            ldap_sync = Some(ldap_user.dn.clone());
                            Some(ldap_user)
                        }
                        // User was NOT found in LDAP,
                        Ok(None) => return Err(ApiError::Unauthorized),
                        Err(e) => {
                            error!("route_auth_login_post {:?}", e);
                            return Err(ApiError::LdapError);
                        }
                    };
                }

                // Create user
                let user = match data.create_user(username, None, email, ldap_sync) {
                    Ok(user) => user,
                    Err(e) => {
                        error!("Failed creating user {:?}", e);
                        return Err(ApiError::DatabaseError);
                    }
                };

                if let Some(ldap_user) = ldap_user {
                    sync_ldap_groups(&data, &user, Some(&ldap_user))?;
                }

                user
            }
            // ERROR from Database, which is critical, so return error
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
                debug!("User not found, testing LDAP");
                match auth_login_ldap(&data, &username, &password).await {
                    Ok(user) => match user {
                        Some(user) => user,
                        None => return Err(ApiError::Unauthorized),
                    },
                    Err(e) => return Err(e),
                }
            }
        }
    };

    debug!("User {} found, generating token", &user.username);

    // Refresh their user-cached permissions before doing anything else
    refresh_user_permissions(&data, &user)?;

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
    let session_lifespan = SConfig::auth_session_lifespan();
    let response = LoginResponse {
        user_id: user.id,
        token: authtoken.id,
        expiry: (authtoken.timestamp.timestamp() as u64) + session_lifespan,
    };
    Ok(web::Json(response))
}
