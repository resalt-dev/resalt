use crate::prelude::*;
use actix_web::{web, Responder, Result};
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventsListGetQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub(crate) async fn route_events_get(
    data: web::Data<Storage>,
    query: web::Query<EventsListGetQuery>,
) -> Result<impl Responder> {
    let limit = query.limit;
    let offset = query.offset;

    let events = match data.list_events(limit, offset) {
        Ok(events) => events,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    Ok(web::Json(events))
}
