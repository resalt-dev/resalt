use actix_web::{web, Responder, Result};
use log::*;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

use crate::components::ApiError;

#[derive(Serialize, Deserialize, Debug)]
pub struct EventsListGetQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_events_get(
    data: web::Data<Box<dyn StorageImpl>>,
    query: web::Query<EventsListGetQuery>,
) -> Result<impl Responder, ApiError> {
    let limit = query.limit;
    let offset = query.offset;

    let events = match data.list_events(limit, offset) {
        Ok(events) => events,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(web::Json(events))
}
