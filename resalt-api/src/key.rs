use axum::http::StatusCode;
use log::error;
use resalt_models::{SaltKeyState, SaltMinionKey, SaltToken};
use resalt_salt::{SaltAPI, SaltError};

pub async fn get_keys(
    salt: &SaltAPI,
    salt_token: &SaltToken,
) -> Result<Vec<SaltMinionKey>, StatusCode> {
    Ok(match salt.get_keys(salt_token).await {
        Ok(keys) => keys,
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            error!("Salt token expired");
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(e) => {
            error!("salt.get_keys {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    })
}

pub async fn accept_key(
    salt: &SaltAPI,
    salt_token: &SaltToken,
    state: &SaltKeyState,
    id: &str,
) -> Result<(), StatusCode> {
    match salt.accept_key(salt_token, state, id).await {
        Ok(()) => Ok(()),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            error!("Salt token expired");
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("salt.accept_key {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn reject_key(
    salt: &SaltAPI,
    salt_token: &SaltToken,
    state: &SaltKeyState,
    id: &str,
) -> Result<(), StatusCode> {
    match salt.reject_key(salt_token, state, id).await {
        Ok(()) => Ok(()),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            error!("Salt token expired");
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("salt.reject_key {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_key(
    salt: &SaltAPI,
    salt_token: &SaltToken,
    state: &SaltKeyState,
    id: &str,
) -> Result<(), StatusCode> {
    match salt.delete_key(salt_token, state, id).await {
        Ok(()) => Ok(()),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            error!("Salt token expired");
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("salt.delete_key {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
