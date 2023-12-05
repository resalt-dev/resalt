use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_api::preset::{
    create_minion_preset, delete_minion_preset, get_minion_preset, get_minion_presets,
    update_minion_preset,
};
use resalt_models::{ApiError, AuthStatus, MinionPreset};
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

pub async fn route_presets_get(
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_PRESETS_LIST)? {
        return Err(ApiError::Forbidden);
    }

    // API
    get_minion_presets(&data).map(|presets| Json(presets))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresetsCreateRequest {
    name: String,
    filter: String,
}

pub async fn route_presets_post(
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<PresetsCreateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_PRESETS_MANAGE)? {
        return Err(ApiError::Forbidden);
    }

    let name = input.name.clone();
    let filter = input.filter.clone(); // TODO: validate filter

    if name.is_empty() || filter.is_empty() || name.len() > 100 || filter.len() > 65535 {
        return Err(ApiError::InvalidRequest);
    }

    // API
    match create_minion_preset(&data, None, &name, &filter) {
        Ok(preset_id) => Ok(Json(MinionPreset {
            id: preset_id,
            name,
            filter,
        })),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    }
}

pub async fn route_preset_get(
    Path(preset_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_PRESETS_LIST)? {
        return Err(ApiError::Forbidden);
    }

    // API
    match get_minion_preset(&data, &preset_id) {
        Ok(Some(preset)) => Ok(Json(preset)),
        Ok(None) => Err(ApiError::NotFound),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresetUpdateRequest {
    name: String,
    filter: String,
}

pub async fn route_preset_put(
    Path(preset_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<PresetUpdateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_PRESETS_MANAGE)? {
        return Err(ApiError::Forbidden);
    }

    let id = preset_id.clone();
    let name = input.name.clone();
    let filter = input.filter.clone(); // TODO: validate filter

    if name.is_empty() || filter.is_empty() || name.len() > 100 || filter.len() > 65535 {
        return Err(ApiError::InvalidRequest);
    }

    // Check if it exists
    match get_minion_preset(&data, &id) {
        Ok(Some(_)) => (),
        Ok(None) => return Err(ApiError::NotFound),
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let preset = MinionPreset { id, name, filter };

    // API
    update_minion_preset(&data, &preset).map(|_| Json(preset))
}

pub async fn route_preset_delete(
    Path(preset_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_PRESETS_MANAGE)? {
        return Err(ApiError::Forbidden);
    }

    // Check if it exists
    let preset = match get_minion_preset(&data, &preset_id) {
        Ok(Some(preset)) => Some(preset),
        Ok(None) => None,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    // API
    delete_minion_preset(&data, &preset_id).map(|_| Json(preset))
}
