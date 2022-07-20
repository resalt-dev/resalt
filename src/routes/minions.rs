use crate::prelude::*;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MinionsListGetQuery {
    id: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_minions_get(
    data: web::Data<Storage>,
    query: web::Query<MinionsListGetQuery>,
) -> Result<impl Responder> {
    let id = query.id.clone();
    let limit = query.limit;
    let offset = query.offset;

    let minions = match data.list_minions(id, limit, offset) {
        Ok(minions) => minions,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    Ok(web::Json(minions))
}

#[derive(Deserialize)]
pub struct MinionGetInfo {
    id: String,
}

pub async fn route_minion_get(
    data: web::Data<Storage>,
    info: web::Path<MinionGetInfo>,
) -> Result<impl Responder> {
    let minion = match data.get_minion_by_id(&info.id) {
        Ok(minion) => minion,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let minion = match minion {
        Some(minion) => minion,
        None => {
            return Err(api_error_not_found());
        }
    };

    Ok(web::Json(minion))
}

pub async fn route_minions_refresh_post(
    salt: web::Data<SaltAPI>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(api_error_unauthorized());
        }
    };
    match salt.refresh_minions(salt_token).await {
        Ok(_) => (),
        Err(e) => {
            error!("refresh_minions {:?}", e);
            return Err(api_error_internal_error());
        }
    };

    Ok(web::Json(()))
}
