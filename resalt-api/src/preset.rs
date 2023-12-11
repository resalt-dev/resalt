use axum::http::StatusCode;
use log::error;
use resalt_models::{MinionPreset, StorageImpl};
use resalt_storage::Storage;

pub fn create_minion_preset(
    data: &Storage,
    id: Option<String>,
    name: &str,
    filter: &str,
) -> Result<String, StatusCode> {
    data.insert_minion_preset(id, name, filter).map_err(|e| {
        error!("api.create_minion_preset {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn get_minion_presets(data: &Storage) -> Result<Vec<MinionPreset>, StatusCode> {
    data.list_minion_presets().map_err(|e| {
        error!("api.get_minion_presets {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn get_minion_preset(data: &Storage, id: &str) -> Result<Option<MinionPreset>, StatusCode> {
    data.get_minion_preset_by_id(id).map_err(|e| {
        error!("api.get_minion_preset {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn update_minion_preset(
    data: &Storage,
    minion_preset: &MinionPreset,
) -> Result<(), StatusCode> {
    data.update_minion_preset(minion_preset).map_err(|e| {
        error!("api.update_minion_preset {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn delete_minion_preset(data: &Storage, id: &str) -> Result<(), StatusCode> {
    data.delete_minion_preset(id).map_err(|e| {
        error!("api.delete_minion_preset {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
