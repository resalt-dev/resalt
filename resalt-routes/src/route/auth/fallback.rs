use axum::response::IntoResponse;
use resalt_models::ApiError;

pub async fn route_fallback_404() -> impl IntoResponse {
    ApiError::NotFound
}
