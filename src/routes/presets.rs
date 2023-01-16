use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::{ApiError, AuthStatus, MinionPreset};
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

use crate::auth::{has_resalt_permission, P_MINION_PRESETS_LIST};

#[derive(Serialize, Deserialize, Debug)]
pub struct PresetsListQuery {
    search: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_presets_get(
    data: web::Data<Box<dyn StorageImpl>>,
    query: web::Query<PresetsListQuery>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_MINION_PRESETS_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let search = query.search.clone();
    let limit = query.limit;
    let offset = query.offset;

    let presets = match data.list_minion_presets(search, limit, offset) {
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
    if !has_resalt_permission(&data, &auth.user_id, P_MINION_PRESETS_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let name = body.name.clone();
    let filter = body.filter.clone();

    if name.is_empty() || filter.is_empty() || name.len() > 100 || filter.len() > 65535 {
        return Err(ApiError::InvalidRequest);
    }

    let preset = match data.insert_minion_preset(&name, &filter) {
        Ok(preset) => preset,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
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
    if !has_resalt_permission(&data, &auth.user_id, P_MINION_PRESETS_LIST)? {
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
    if !has_resalt_permission(&data, &auth.user_id, P_MINION_PRESETS_LIST)? {
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
    if !has_resalt_permission(&data, &auth.user_id, P_MINION_PRESETS_LIST)? {
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
