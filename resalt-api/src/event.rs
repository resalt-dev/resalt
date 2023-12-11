use axum::http::StatusCode;
use log::*;
use resalt_models::{Event, Paginate, StorageImpl};
use resalt_storage::Storage;

pub async fn get_events(paginate: Paginate, data: Storage) -> Result<Vec<Event>, StatusCode> {
    match data.list_events(paginate) {
        Ok(events) => Ok(events),
        Err(e) => {
            error!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
