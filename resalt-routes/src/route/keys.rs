use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_auth::renew_token_salt_token;
use resalt_models::*;
use resalt_salt::{SaltAPI, SaltError, SaltKeyState};
use resalt_security::*;
use resalt_storage::StorageImpl;

pub async fn route_keys_get(
    State(data): State<Box<dyn StorageImpl>>,
    State(salt): State<SaltAPI>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_SALTKEY_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };

    let keys = match salt.get_keys(salt_token).await {
        Ok(keys) => keys,
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(ApiError::InternalError);
            }
            error!("Salt token expired, renewing and retrying");
            let auth =
                renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token).await?;
            match salt.get_keys(&auth.salt_token.unwrap()).await {
                Ok(keys) => keys,
                Err(e) => {
                    error!("salt.get_keys {:?}", e);
                    return Err(ApiError::InternalError);
                }
            }
        }
        Err(e) => {
            error!("salt.get_keys {:?}", e);
            return Err(ApiError::InternalError);
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
                return Err(ApiError::InternalError);
            }
        };
    }

    Ok(Json(keys))
}

pub async fn route_key_accept_put(
    Path(state): Path<SaltKeyState>,
    Path(id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    State(salt): State<SaltAPI>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_SALTKEY_ACCEPT)? {
        return Err(ApiError::Forbidden);
    }

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };

    match salt.accept_key(salt_token, &state, &id).await {
        Ok(()) => Ok(Json(())),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(ApiError::InternalError);
            }
            error!("Salt token expired, renewing and retrying");
            let auth =
                renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token).await?;
            match salt
                .accept_key(&auth.salt_token.unwrap(), &state, &id)
                .await
            {
                Ok(()) => Ok(Json(())),
                Err(e) => {
                    error!("{:?}", e);
                    Err(ApiError::InternalError)
                }
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::InternalError)
        }
    }
}

pub async fn route_key_reject_put(
    Path(state): Path<SaltKeyState>,
    Path(id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    State(salt): State<SaltAPI>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_SALTKEY_REJECT)? {
        return Err(ApiError::Forbidden);
    }

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };

    match salt.reject_key(salt_token, &state, &id).await {
        Ok(()) => Ok(Json(())),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(ApiError::InternalError);
            }
            error!("Salt token expired, renewing and retrying");
            let auth =
                renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token).await?;
            match salt
                .reject_key(&auth.salt_token.unwrap(), &state, &id)
                .await
            {
                Ok(()) => Ok(Json(())),
                Err(e) => {
                    error!("{:?}", e);
                    Err(ApiError::InternalError)
                }
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::InternalError)
        }
    }
}

pub async fn route_key_delete_delete(
    Path(state): Path<SaltKeyState>,
    Path(id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    State(salt): State<SaltAPI>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_SALTKEY_DELETE)? {
        return Err(ApiError::Forbidden);
    }

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };

    match salt.delete_key(salt_token, &state, &id).await {
        Ok(()) => Ok(Json(())),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(ApiError::InternalError);
            }
            error!("Salt token expired, renewing and retrying");
            let auth =
                renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token).await?;
            match salt
                .delete_key(&auth.salt_token.unwrap(), &state, &id)
                .await
            {
                Ok(()) => Ok(Json(())),
                Err(e) => {
                    error!("{:?}", e);
                    Err(ApiError::InternalError)
                }
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::InternalError)
        }
    }
}
