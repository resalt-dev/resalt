use axum::response::{IntoResponse, Json};
use resalt_models::ApiError;

pub async fn route_index_get() -> Result<impl IntoResponse, ApiError> {
    Ok(Json("Hello API".to_owned()))
}
