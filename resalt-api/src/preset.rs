use log::error;
use resalt_models::{ApiError, MinionPreset, StorageImpl};
use resalt_storage::Storage;

pub fn create_minion_preset(
    data: &Storage,
    id: Option<String>,
    name: &str,
    filter: &str,
) -> Result<String, ApiError> {
    data.insert_minion_preset(id, name, filter).map_err(|e| {
        error!("api.create_minion_preset {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn get_minion_presets(data: &Storage) -> Result<Vec<MinionPreset>, ApiError> {
    data.list_minion_presets().map_err(|e| {
        error!("api.get_minion_presets {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn get_minion_preset(data: &Storage, id: &str) -> Result<Option<MinionPreset>, ApiError> {
    data.get_minion_preset_by_id(id).map_err(|e| {
        error!("api.get_minion_preset {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn update_minion_preset(data: &Storage, minion_preset: &MinionPreset) -> Result<(), ApiError> {
    data.update_minion_preset(minion_preset).map_err(|e| {
        error!("api.update_minion_preset {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn delete_minion_preset(data: &Storage, id: &str) -> Result<(), ApiError> {
    data.delete_minion_preset(id).map_err(|e| {
        error!("api.delete_minion_preset {:?}", e);
        ApiError::DatabaseError
    })
}
