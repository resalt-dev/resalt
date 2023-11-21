use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_auth::renew_token_salt_token;
use resalt_models::*;
use resalt_salt::{SaltAPI, SaltError};
use resalt_security::*;
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
    query: Query<MinionsListGetQuery>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_LIST)? {
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
    if !has_resalt_permission(&auth.perms, P_MINION_CONFORMITY)? {
        for minion in minions.iter_mut() {
            minion.conformity = None;
        }
    }
    if !has_resalt_permission(&auth.perms, P_MINION_PILLARS)? {
        for minion in minions.iter_mut() {
            minion.pillars = None;
        }
    }
    if !has_resalt_permission(&auth.perms, P_MINION_PACKAGES)? {
        for minion in minions.iter_mut() {
            minion.pkgs = None;
        }
    }

    Ok(Json(minions))
}

pub async fn route_minion_get(
    Path(minion_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let minion = match data.get_minion_by_id(&minion_id) {
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

    Ok(Json(minion))
}

pub async fn route_minion_refresh_post(
    Path(minion_id): Path<String>,
    State(salt): State<SaltAPI>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(mut auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_REFRESH)? {
        return Err(ApiError::Forbidden);
    }

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };
    match salt.refresh_minion(salt_token, &minion_id).await {
        Ok(_) => (),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(ApiError::InternalError);
            }
            error!("Salt token expired, renewing and retrying");
            auth = renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token).await?;
            match salt
                .refresh_minion(&auth.salt_token.unwrap(), &minion_id)
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

    Ok(Json(()))
}
