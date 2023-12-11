use axum::http::StatusCode;

pub async fn route_fallback_404() -> StatusCode {
    StatusCode::NOT_FOUND
}
