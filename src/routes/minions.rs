use crate::{auth::*, components::*};
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::*;
use resalt_salt::{SaltAPI, SaltError};
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
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_MINION_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let filter = query.filter.clone();
    let filter = match filter {
        Some(filter) => Some(match urlencoding::decode(filter.as_str()) {
            Ok(filter) => filter.to_string(),
            Err(e) => {
                error!("Failed to decode filter: {}", e);
                return Err(ApiError::InvalidRequest);
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
                return Err(ApiError::InvalidRequest);
            }
        },
        None => vec![],
    };

    let mut minions = match data.list_minions(filters, sort, limit, offset) {
        Ok(minions) => minions,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // Validate extra permission
    if !has_resalt_permission(&data, &auth.user_id, P_MINION_CONFORMITY)? {
        for minion in minions.iter_mut() {
            minion.conformity = None;
        }
    }
    if !has_resalt_permission(&data, &auth.user_id, P_MINION_PILLARS)? {
        for minion in minions.iter_mut() {
            minion.pillars = None;
        }
    }
    if !has_resalt_permission(&data, &auth.user_id, P_MINION_PACKAGES)? {
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
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_MINION_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let minion = match data.get_minion_by_id(&info.id) {
        Ok(minion) => minion,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let minion = match minion {
        Some(minion) => minion,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    Ok(web::Json(minion))
}

pub async fn route_minion_refresh_post(
    salt: web::Data<SaltAPI>,
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<MinionGetInfo>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let mut auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };
    match salt.refresh_minion(salt_token, &info.id).await {
        Ok(_) => (),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(ApiError::InternalError);
            }
            error!("Salt token expired, renewing and retrying");
            auth = renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token).await?;
            match salt
                .refresh_minion(&auth.salt_token.unwrap(), &info.id)
                .await
            {
                Ok(_) => (),
                Err(e) => {
                    error!("refresh_minion {:?}", e);
                    return Err(ApiError::InternalError);
                }
            }
        }
        Err(e) => {
            error!("refresh_minion {:?}", e);
            return Err(ApiError::InternalError);
        }
    };

    Ok(web::Json(()))
}
