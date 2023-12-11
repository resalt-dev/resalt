use crate::{login::renew_token_salt_token, permission::*};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_api::key::{accept_key, delete_key, get_keys, reject_key};
use resalt_models::*;
use resalt_salt::SaltAPI;
use resalt_storage::Storage;

pub async fn route_keys_get(
    State(data): State<Storage>,
    State(salt): State<SaltAPI>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_SALTKEY_LIST)? {
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
    let keys = match get_keys(&salt, salt_token).await {
        Ok(keys) => keys,
        Err(e) => {
            error!("get_keys {:?}", e);
            // Try refresh salt token, and try again
            let salt_token = renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token)
                .await?
                .salt_token;
            match get_keys(&salt, &salt_token.unwrap()).await {
                Ok(keys) => keys,
                Err(e) => {
                    error!("get_keys {:?}", e);
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
    };

    // Clean out non-existing minions
    let ids = keys
        .clone()
        .into_iter()
        .map(|key| key.id)
        .collect::<Vec<String>>();
    for id in ids {
        match data.delete_minion(id) {
            Ok(_) => (),
            Err(e) => {
                error!("{:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    }

    Ok(Json(keys))
}

pub async fn route_key_accept_put(
    Path((state, id)): Path<(SaltKeyState, String)>,
    State(data): State<Storage>,
    State(salt): State<SaltAPI>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_SALTKEY_ACCEPT)? {
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
    match accept_key(&salt, salt_token, &state, &id).await {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("accept_key {:?}", e);
            // Try refresh salt token, and try again
            let salt_token = renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token)
                .await?
                .salt_token;
            match accept_key(&salt, &salt_token.unwrap(), &state, &id).await {
                Ok(()) => Ok(()),
                Err(e) => {
                    error!("accept_key {:?}", e);
                    Err(StatusCode::UNAUTHORIZED)
                }
            }
        }
    }
}

pub async fn route_key_reject_put(
    Path((state, id)): Path<(SaltKeyState, String)>,
    State(data): State<Storage>,
    State(salt): State<SaltAPI>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_SALTKEY_REJECT)? {
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
    match reject_key(&salt, salt_token, &state, &id).await {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("reject_key {:?}", e);
            // Try refresh salt token, and try again
            let salt_token = renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token)
                .await?
                .salt_token;
            match reject_key(&salt, &salt_token.unwrap(), &state, &id).await {
                Ok(()) => Ok(()),
                Err(e) => {
                    error!("reject_key {:?}", e);
                    Err(StatusCode::UNAUTHORIZED)
                }
            }
        }
    }
}

pub async fn route_key_delete_delete(
    Path((state, id)): Path<(SaltKeyState, String)>,
    State(data): State<Storage>,
    State(salt): State<SaltAPI>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_SALTKEY_DELETE)? {
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
    match delete_key(&salt, salt_token, &state, &id).await {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("delete_key {:?}", e);
            // Try refresh salt token, and try again
            let salt_token = renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token)
                .await?
                .salt_token;
            match delete_key(&salt, &salt_token.unwrap(), &state, &id).await {
                Ok(()) => Ok(()),
                Err(e) => {
                    error!("delete_key {:?}", e);
                    Err(StatusCode::UNAUTHORIZED)
                }
            }
        }
    }
}
