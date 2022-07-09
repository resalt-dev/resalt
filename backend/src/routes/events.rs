use crate::prelude::*;
use actix_web::{web, Responder, Result};
use log::*;

pub async fn route_events_get(data: web::Data<Storage>) -> Result<impl Responder> {
    let events = match data.list_events() {
        Ok(events) => events,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    Ok(web::Json(events))
}
