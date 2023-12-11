use axum::{
    extract::{Query, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use log::*;
use resalt_models::AuthStatus;
use resalt_salt::SaltAPI;
use resalt_storage::Storage;
use std::collections::HashMap;

use crate::login::{renew_token_salt_token, validate_auth_token};

#[allow(clippy::let_and_return)]
pub async fn middleware_auth<B>(
    State(data): State<Storage>,
    State(salt): State<SaltAPI>,
    Query(params): Query<HashMap<String, String>>,
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    //
    // PRE-REQUEST PROCESSING
    //

    // Extract token from header or query params
    let token = match req.headers().get("Authorization") {
        Some(header) => header.to_str().unwrap().replace("Bearer ", ""),
        None => {
            // Try fetch value "token" from query params
            let token = match params.get("token") {
                Some(token) => token.to_string(),
                None => "".to_string(),
            };
            token
        }
    };

    // Resolve auth status
    let auth_status = match resolve_auth_status(data, salt, token).await {
        Ok(auth_status) => match auth_status {
            Some(auth_status) => auth_status,
            None => {
                return StatusCode::UNAUTHORIZED.into_response();
            }
        },
        Err(e) => {
            error!("{:?}", e);
            return e.into_response();
        }
    };

    // Add AuthStatus to request extensions
    req.extensions_mut().insert(auth_status);

    //
    // MAIN
    //
    let response = next.run(req).await;

    //
    // POST-REQUEST PROCESSING
    //

    response
}

async fn resolve_auth_status(
    db: Storage,
    salt: SaltAPI,
    token: String,
) -> Result<Option<AuthStatus>, StatusCode> {
    let data = db.clone();

    let auth_status = match validate_auth_token(&data, &token) {
        Ok(auth_status) => auth_status,
        Err(e) => {
            error!("{:?}", e);
            return Err(e);
        }
    };

    let auth_status: AuthStatus = match auth_status {
        Some(auth_status) => auth_status,
        None => {
            return Ok(None);
        }
    };

    // Check if salt_token has expired
    let salt_token = match &auth_status.salt_token {
        Some(salt_token) => salt_token,
        None => {
            return Ok(Some(auth_status));
        }
    };

    match salt_token.expired() {
        true => {
            warn!(
                "Salt token expired for {}! Attempting to renew...",
                auth_status.user_id
            );

            match renew_token_salt_token(&data, &salt, &auth_status.user_id, &token).await {
                Ok(_) => {}
                Err(e) => {
                    error!("{:?}", e);
                    return Err(e);
                }
            }

            // Re-fetch their auth_status with the new salt_token saved in DB
            let auth_status = match validate_auth_token(&data, &token) {
                Ok(auth_status) => auth_status,
                Err(e) => {
                    error!("{:?}", e);
                    return Err(e);
                }
            };

            match auth_status {
                Some(auth_status) => Ok(Some(auth_status)),
                None => Ok(None),
            }
        }
        false => Ok(Some(auth_status)),
    }
}
