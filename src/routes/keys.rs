use crate::prelude::*;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use serde::Deserialize;
use serde_json::{json, Value};

pub async fn route_keys_get(salt: web::Data<SaltAPI>, req: HttpRequest) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(api_error_unauthorized());
        }
    };

    let keys = match salt.get_keys(salt_token).await {
        Ok(keys) => keys,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_internal_error());
        }
    };

    // map tuples to object
    let keys = keys
        .into_iter()
        .map(|(id, status, finger)| json!({ "id": id, "finger": finger, "status": status }))
        .collect::<Vec<Value>>();

    Ok(web::Json(keys))
}

#[derive(Deserialize)]
pub struct KeyInfo {
    id: String,
}

pub async fn route_key_accept_put(
    salt: web::Data<SaltAPI>,
    info: web::Path<KeyInfo>,
) -> Result<impl Responder> {
    match salt.accept_key(&info.id).await {
        Ok(()) => Ok(web::Json({})),
        Err(e) => {
            error!("{:?}", e);
            Err(api_error_internal_error())
        }
    }
}

pub async fn route_key_reject_put(
    salt: web::Data<SaltAPI>,
    info: web::Path<KeyInfo>,
) -> Result<impl Responder> {
    match salt.reject_key(&info.id).await {
        Ok(()) => Ok(web::Json({})),
        Err(e) => {
            error!("{:?}", e);
            Err(api_error_internal_error())
        }
    }
}

pub async fn route_key_delete_delete(
    salt: web::Data<SaltAPI>,
    info: web::Path<KeyInfo>,
) -> Result<impl Responder> {
    match salt.delete_key(&info.id).await {
        Ok(()) => Ok(web::Json({})),
        Err(e) => {
            error!("{:?}", e);
            Err(api_error_internal_error())
        }
    }
}
