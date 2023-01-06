use crate::auth::{update_token_salt_token, validate_auth_token};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Query,
    Error, HttpMessage,
};
use futures::future::{ok, Future, Ready};
use log::*;
use resalt_models::AuthStatus;
use resalt_salt::SaltAPI;
use resalt_storage::StorageImpl;
use std::rc::Rc;
use std::{collections::HashMap, pin::Pin};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct ValidateAuth {
    db: Box<dyn StorageImpl>,
    salt: SaltAPI,
}

impl ValidateAuth {
    pub fn new(db: Box<dyn StorageImpl>, salt: SaltAPI) -> Self {
        Self { db, salt }
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for ValidateAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ValidateAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ValidateAuthMiddleware {
            service: Rc::new(service),
            db: self.db.clone(),
            salt: self.salt.clone(),
        })
    }
}

pub struct ValidateAuthMiddleware<S: 'static> {
    service: Rc<S>,
    db: Box<dyn StorageImpl>,
    salt: SaltAPI,
}

impl<S, B> Service<ServiceRequest> for ValidateAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let data = self.db.clone();
        let salt = self.salt.clone();

        // Extract token from header or query params
        let token = match req.headers().get("Authorization") {
            Some(header) => header.to_str().unwrap().replace("Bearer ", ""),
            None => {
                // Try fetch value "token" from query params
                let token = match Query::<HashMap<String, String>>::from_query(req.query_string()) {
                    Ok(params) => match params.get("token") {
                        Some(token) => token.to_string(),
                        None => "".to_string(),
                    },
                    Err(_) => "".to_string(),
                };
                token
            }
        };

        Box::pin(async move {
            {
                let auth_status = resolve_auth_status(data, salt, token).await?;
                if let Some(auth_status) = auth_status {
                    req.extensions_mut().insert(auth_status);
                }
            }

            let fut = srv.call(req);
            let res = fut.await?;
            Ok(res)
        })
    }
}

async fn resolve_auth_status(
    db: Box<dyn StorageImpl>,
    salt: SaltAPI,
    token: String,
) -> Result<Option<AuthStatus>, Error> {
    let data = db.clone();

    let auth_status = match validate_auth_token(&data, &token) {
        Ok(auth_status) => auth_status,
        Err(e) => {
            error!("{:?}", e);
            return Err(e.into());
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

            match update_token_salt_token(&data, &salt, &auth_status.user_id, &token).await {
                Ok(_) => {}
                Err(e) => {
                    error!("{:?}", e);
                    return Err(e.into());
                }
            }

            // Re-fetch their auth_status with the new salt_token saved in DB
            let auth_status = match validate_auth_token(&data, &token) {
                Ok(auth_status) => auth_status,
                Err(e) => {
                    error!("{:?}", e);
                    return Err(e.into());
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
