use crate::PaginateQuery;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use resalt_api::events::get_events;
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

    // API
    match get_events(paginate, data).await {
        Ok(events) => Ok(Json(events)),
        Err(e) => Err(e),
    }
}
