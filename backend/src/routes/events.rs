use crate::prelude::*;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventsGetQuery {
    // refresh: Option<bool>,
}

#[derive(Serialize, Debug)]
struct EventsResponse {
    events: Vec<Event>,
}

pub async fn route_events_get(
    data: web::Data<Storage>,
    //salt: web::Data<SaltAPI>,
    //query: web::Query<EventsGetQuery>,
    //req: HttpRequest,
) -> Result<impl Responder> {
    //let ext = req.extensions_mut();
    //let auth = ext.get::<AuthStatus>().unwrap();

    let events = match data.list_events() {
        Ok(events) => events,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let response = EventsResponse { events };
    Ok(web::Json(response))
}
