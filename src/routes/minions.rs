use crate::{auth::*, components::*};
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::*;
use resalt_salt::SaltAPI;
use resalt_storage::StorageImpl;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MinionsListGetQuery {
    filter: Option<String>, // URL-encoded JSON
    sort: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_minions_get(
    data: web::Data<Box<dyn StorageImpl>>,
    query: web::Query<MinionsListGetQuery>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    // Validate permission
    if !has_permission(&data, &auth.user_id, P_MINION_LIST)? {
        return Err(api_error_forbidden());
    }

    let filter = query.filter.clone();
    let filter = match filter {
        Some(filter) => Some(match urlencoding::decode(filter.as_str()) {
            Ok(filter) => filter.to_string(),
            Err(e) => {
                error!("Failed to decode filter: {}", e);
                return Err(api_error_invalid_request());
            }
        }),
        None => None,
    };

    let sort = query.sort.clone();
    let limit = query.limit;
    let offset = query.offset;

    let filters: Vec<Filter> = match filter {
        Some(filter) => match serde_json::from_str(&filter) {
            Ok(filters) => filters,
            Err(e) => {
                error!("Failed to parse filter: {}", e);
                return Err(api_error_invalid_request());
            }
        },
        None => vec![],
    };

    let mut minions = match data.list_minions(filters, sort, limit, offset) {
        Ok(minions) => minions,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    // Validate extra permission
    if !has_permission(&data, &auth.user_id, P_MINION_CONFORMITY)? {
        for minion in minions.iter_mut() {
            minion.conformity = None;
        }
    }
    if !has_permission(&data, &auth.user_id, P_MINION_PILLARS)? {
        for minion in minions.iter_mut() {
            minion.pillars = None;
        }
    }
    if !has_permission(&data, &auth.user_id, P_MINION_PACKAGES)? {
        for minion in minions.iter_mut() {
            minion.pkgs = None;
        }
    }

    Ok(web::Json(minions))
}

#[derive(Deserialize)]
pub struct MinionGetInfo {
    id: String,
}

pub async fn route_minion_get(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<MinionGetInfo>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    // Validate permission
    if !has_permission(&data, &auth.user_id, P_MINION_LIST)? {
        return Err(api_error_forbidden());
    }

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
