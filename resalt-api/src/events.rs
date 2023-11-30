use log::*;
use resalt_models::{ApiError, Event, Paginate};
use resalt_storage::StorageImpl;

pub async fn get_events(
    paginate: Paginate,
    data: Box<dyn StorageImpl>,
) -> Result<Vec<Event>, ApiError> {
    match data.list_events(paginate) {
        Ok(events) => Ok(events),
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::DatabaseError)
        }
    }
}
