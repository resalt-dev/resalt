use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::{ApiError, AuthStatus, MinionPreset};
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

pub async fn route_presets_get(
    data: web::Data<Box<dyn StorageImpl>>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

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

    Ok(web::Json(presets))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresetsCreateRequest {
    name: String,
    filter: String,
}

pub async fn route_presets_post(
    data: web::Data<Box<dyn StorageImpl>>,
    req: HttpRequest,
    body: web::Json<PresetsCreateRequest>,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_PRESETS_MANAGE)? {
        return Err(ApiError::Forbidden);
    }

    let name = body.name.clone();
    let filter = body.filter.clone();

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

    Ok(web::Json(preset))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresetInfo {
    id: String,
}

pub async fn route_preset_get(
    data: web::Data<Box<dyn StorageImpl>>,
    req: HttpRequest,
    info: web::Path<PresetInfo>,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_PRESETS_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let id = info.id.clone();

    let preset = match data.get_minion_preset_by_id(&id) {
        Ok(preset) => preset,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    match preset {
        Some(preset) => Ok(web::Json(preset)),
        None => Err(ApiError::NotFound),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PresetUpdateRequest {
    name: String,
    filter: String,
}

pub async fn route_preset_put(
    data: web::Data<Box<dyn StorageImpl>>,
    req: HttpRequest,
    info: web::Path<PresetInfo>,
    body: web::Json<PresetUpdateRequest>,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_PRESETS_MANAGE)? {
        return Err(ApiError::Forbidden);
    }

    let id = info.id.clone();
    let name = body.name.clone();
    let filter = body.filter.clone();

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

    Ok(web::Json(preset))
}

pub async fn route_preset_delete(
    data: web::Data<Box<dyn StorageImpl>>,
    req: HttpRequest,
    info: web::Path<PresetInfo>,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_PRESETS_MANAGE)? {
        return Err(ApiError::Forbidden);
    }

    let id = info.id.clone();

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

    match data.delete_minion_preset(&id) {
        Ok(_) => {}
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(web::Json(()))
}
