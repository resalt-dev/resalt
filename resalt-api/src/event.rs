use log::*;
use resalt_models::{ApiError, Event, Paginate, StorageImpl};
use resalt_storage::Storage;

pub async fn get_events(paginate: Paginate, data: Storage) -> Result<Vec<Event>, ApiError> {
    match data.list_events(paginate) {
        Ok(events) => Ok(events),
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::DatabaseError)
        }
    }
}
