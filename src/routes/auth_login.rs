use crate::prelude::*;
use actix_web::{web, HttpRequest, Responder, Result};
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
    req: HttpRequest,
) -> Result<impl Responder> {
    let user: User = if SConfig::auth_forward_enabled() {
        // Use X-Forwarded-User header as username
        let username = match req.headers().get("X-Forwarded-User") {
            Some(forwarded_user) => forwarded_user.to_str().unwrap().to_string(),
            None => return Err(api_error_unauthorized()),
        };

        // Fetch user to see if they exist
        let user = match data.get_user_by_username(&username) {
            Ok(user) => user,
            Err(e) => {
                error!("{:?}", e);
                return Err(api_error_database());
            }
        };
        match user {
            Some(user) => user,
            None => {
                // Create User
                let (ldap_sync, username, email) = match SConfig::auth_ldap_enabled() {
                    true => {
                        // Create user
                        match LdapHandler::find_dn(&username).await {
                            // User was found in LDAP, lets link
                            Ok(Some((ldap_sync, username, email))) => {
                                (Some(ldap_sync), username, Some(email))
                            }
                            // User was not found in LDAP,
                            Ok(None) => return Err(api_error_unauthorized()),
                            Err(e) => {
                                error!("route_auth_login_post {:?}", e);
                                return Err(api_error_ldap());
                            }
                        }
                    }
                    false => (None, username, None),
                };

                match data.create_user(username, None, email, ldap_sync) {
                    Ok(user) => user,
                    Err(e) => {
                        error!("Failed creating user {:?}", e);
                        return Err(api_error_database());
                    }
                }
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
                        None => return Err(api_error_unauthorized()),
                    },
                    Err(e) => return Err(e),
                }
            }
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
