use crate::{login::renew_token_salt_token, permission::*};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_api::minion::{get_minion, get_minions, refresh_minion};
use resalt_models::*;
use resalt_salt::{SaltAPI, SaltError};
use resalt_storage::Storage;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MinionsListGetQuery {
    filter: Option<String>, // URL-encoded JSON
    sort: Option<MinionSort>,
    // Include fields from PaginateQuery
    #[serde(flatten)]
    paginate_query: PaginateQuery,
}

pub async fn route_minions_get(
    query: Query<MinionsListGetQuery>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_LIST)? {
        return Err(StatusCode::FORBIDDEN);
    }

    let filter = match &query.filter {
        Some(filter) => Some(match urlencoding::decode(filter.as_str()) {
            Ok(filter) => filter.to_string(),
            Err(e) => {
                error!("Failed to decode filter: {}", e);
                return Err(StatusCode::BAD_REQUEST);
            }
        }),
        None => None,
    };

    let sort = query.sort.clone();
    // Pagination
    let paginate: Paginate = query.paginate_query.parse_query();

    let filters: Vec<Filter> = match filter {
        Some(filter) => match serde_json::from_str(&filter) {
            Ok(filters) => filters,
            Err(e) => {
                error!("Failed to parse filter: {}", e);
                return Err(StatusCode::BAD_REQUEST);
            }
        },
        None => vec![],
    };

    let mut minions = match get_minions(&data, filters, sort, paginate) {
        Ok(minions) => minions,
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Validate extra permission
    if !has_resalt_permission(&auth, P_MINION_CONFORMITY)? {
        for minion in minions.iter_mut() {
            minion.conformity = None;
        }
    }
    if !has_resalt_permission(&auth, P_MINION_PILLARS)? {
        for minion in minions.iter_mut() {
            minion.pillars = None;
        }
    }
    if !has_resalt_permission(&auth, P_MINION_PACKAGES)? {
        for minion in minions.iter_mut() {
            minion.pkgs = None;
        }
    }

    Ok(Json(minions))
}

pub async fn route_minion_get(
    Path(minion_id): Path<String>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_LIST)? {
        return Err(StatusCode::FORBIDDEN);
    }

    let minion = match get_minion(&data, &minion_id) {
        Ok(Some(minion)) => minion,
        Ok(None) => {
            return Err(StatusCode::NOT_FOUND);
        }
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(minion))
}

pub async fn route_minion_refresh_post(
    Path(minion_id): Path<String>,
    State(salt): State<SaltAPI>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_REFRESH)? {
        return Err(StatusCode::FORBIDDEN);
    }

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // API
    match refresh_minion(&salt, salt_token, &minion_id).await {
        Ok(()) => Ok(Json(())),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            error!("Salt token expired, renewing and retrying");
            let auth =
                renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token).await?;
            match refresh_minion(&salt, &auth.salt_token.unwrap(), &minion_id).await {
                Ok(()) => Ok(Json(())),
                Err(e) => {
                    error!("route_minion_refresh_post {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            error!("route_minion_refresh_post {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
