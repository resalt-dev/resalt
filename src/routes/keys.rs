use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::*;
use resalt_salt::{SaltAPI, SaltKeyState};
use resalt_storage::StorageImpl;
use serde::Deserialize;

use crate::components::*;

pub async fn route_keys_get(
    salt: web::Data<SaltAPI>,
    data: web::Data<Box<dyn StorageImpl>>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };

    let keys = match salt.get_keys(salt_token).await {
        Ok(keys) => keys,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::InternalError);
        }
    };

    // Clean out non-existing minions
    let ids = keys
        .clone()
        .into_iter()
        .map(|key| key.id)
        .collect::<Vec<String>>();
    match data.prune_minions(ids) {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::InternalError);
        }
    };

    Ok(web::Json(keys))
}

#[derive(Deserialize)]
pub struct KeyInfo {
    state: SaltKeyState,
    id: String,
}

pub async fn route_key_accept_put(
    salt: web::Data<SaltAPI>,
    info: web::Path<KeyInfo>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };

    match salt.accept_key(salt_token, &info.state, &info.id).await {
        Ok(()) => Ok(web::Json(())),
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::InternalError)
        }
    }
}

pub async fn route_key_reject_put(
    salt: web::Data<SaltAPI>,
    info: web::Path<KeyInfo>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };

    match salt.reject_key(salt_token, &info.state, &info.id).await {
        Ok(()) => Ok(web::Json(())),
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::InternalError)
        }
    }
}

pub async fn route_key_delete_delete(
    salt: web::Data<SaltAPI>,
    info: web::Path<KeyInfo>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };

    match salt.delete_key(salt_token, &info.state, &info.id).await {
        Ok(()) => Ok(web::Json(())),
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::InternalError)
        }
    }
}
