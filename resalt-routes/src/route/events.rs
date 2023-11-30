use crate::PaginateQuery;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_models::{ApiError, AuthStatus};
use resalt_security::*;
use resalt_storage::StorageImpl;

pub async fn route_events_get(
    query: Query<PaginateQuery>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_EVENT_LIST)? {
        return Err(ApiError::Forbidden);
    }

    // Pagination
    let paginate = query.parse_query();

    let events = match data.list_events(paginate) {
        Ok(events) => events,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(Json(events))
}
