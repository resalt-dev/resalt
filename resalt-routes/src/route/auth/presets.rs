use crate::permission::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_api::preset::{
    create_minion_preset, delete_minion_preset, get_minion_preset, get_minion_presets,
    update_minion_preset,
};
use resalt_models::{AuthStatus, MinionPreset};
use resalt_storage::Storage;
use serde::{Deserialize, Serialize};

pub async fn route_presets_get(
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_PRESETS_LIST)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // API
    get_minion_presets(&data).map(Json)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresetsCreateRequest {
    name: String,
    filter: String,
}

pub async fn route_presets_post(
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<PresetsCreateRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_PRESETS_MANAGE)? {
        return Err(StatusCode::FORBIDDEN);
    }

    let name = input.name.clone();
    let filter = input.filter.clone(); // TODO: validate filter

    if name.is_empty() || filter.is_empty() || name.len() > 100 || filter.len() > 65535 {
        return Err(StatusCode::BAD_REQUEST);
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
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn route_preset_get(
    Path(preset_id): Path<String>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_PRESETS_LIST)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // API
    match get_minion_preset(&data, &preset_id) {
        Ok(Some(preset)) => Ok(Json(preset)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
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
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<PresetUpdateRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_PRESETS_MANAGE)? {
        return Err(StatusCode::FORBIDDEN);
    }

    let id = preset_id.clone();
    let name = input.name.clone();
    let filter = input.filter.clone(); // TODO: validate filter

    if name.is_empty() || filter.is_empty() || name.len() > 100 || filter.len() > 65535 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if it exists
    match get_minion_preset(&data, &id) {
        Ok(Some(_)) => (),
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let preset = MinionPreset { id, name, filter };

    // API
    update_minion_preset(&data, &preset).map(|_| Json(preset))
}

pub async fn route_preset_delete(
    Path(preset_id): Path<String>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_PRESETS_MANAGE)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // Check if it exists
    let preset = match get_minion_preset(&data, &preset_id) {
        Ok(Some(preset)) => Some(preset),
        Ok(None) => None,
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // API
    delete_minion_preset(&data, &preset_id).map(|_| Json(preset))
}
