use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_models::{ApiError, AuthStatus, MinionPreset};
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

pub async fn route_presets_get(
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_PRESETS_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let presets = match data.list_minion_presets() {
        Ok(presets) => presets,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(Json(presets))
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
    if !has_resalt_permission(&auth.perms, P_MINION_PRESETS_MANAGE)? {
        return Err(ApiError::Forbidden);
    }

    let name = input.name.clone();
    let filter = input.filter.clone();

    if name.is_empty() || filter.is_empty() || name.len() > 100 || filter.len() > 65535 {
        return Err(ApiError::InvalidRequest);
    }

    let preset_id = match data.insert_minion_preset(None, &name, &filter) {
        Ok(preset) => preset,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let preset = MinionPreset {
        id: preset_id,
        name,
        filter,
    };

    Ok(Json(preset))
}

pub async fn route_preset_get(
    Path(preset_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_PRESETS_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let preset = match data.get_minion_preset_by_id(&preset_id) {
        Ok(preset) => preset,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    match preset {
        Some(preset) => Ok(Json(preset)),
        None => Err(ApiError::NotFound),
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
    if !has_resalt_permission(&auth.perms, P_MINION_PRESETS_MANAGE)? {
        return Err(ApiError::Forbidden);
    }

    let id = preset_id.clone();
    let name = input.name.clone();
    let filter = input.filter.clone();

    if name.is_empty() || filter.is_empty() || name.len() > 100 || filter.len() > 65535 {
        return Err(ApiError::InvalidRequest);
    }

    // Check if it exists
    let preset = match data.get_minion_preset_by_id(&id) {
        Ok(preset) => preset,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    match preset {
        Some(_) => {}
        None => return Err(ApiError::NotFound),
    }

    let preset = MinionPreset { id, name, filter };

    match data.update_minion_preset(&preset) {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(Json(preset))
}

pub async fn route_preset_delete(
    Path(preset_id): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_PRESETS_MANAGE)? {
        return Err(ApiError::Forbidden);
    }

    // Check if it exists
    let preset = match data.get_minion_preset_by_id(&preset_id) {
        Ok(preset) => preset,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    match preset {
        Some(_) => {}
        None => return Err(ApiError::NotFound),
    }

    match data.delete_minion_preset(&preset_id) {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(Json(()))
}
