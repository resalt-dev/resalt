use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_models::{ApiError, AuthStatus};
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventsListGetQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_events_get(
    query: Query<EventsListGetQuery>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth.perms, P_EVENT_LIST)? {
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

    Ok(Json(events))
}
