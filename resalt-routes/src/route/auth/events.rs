use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use resalt_api::event::get_events;
use resalt_models::{ApiError, AuthStatus, PaginateQuery};
use resalt_security::*;
use resalt_storage::Storage;

pub async fn route_events_get(
    query: Query<PaginateQuery>,
    State(data): State<Storage>,
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
