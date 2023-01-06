use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::AuthStatus;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{has_permission, P_EVENT_LIST},
    components::ApiError,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventsListGetQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_events_get(
    data: web::Data<Box<dyn StorageImpl>>,
    query: web::Query<EventsListGetQuery>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_permission(&data, &auth.user_id, P_EVENT_LIST)? {
        return Err(ApiError::Forbidden);
    }

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
