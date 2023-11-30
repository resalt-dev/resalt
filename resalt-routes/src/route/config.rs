use axum::response::{IntoResponse, Json};
use resalt_models::ApiError;

pub async fn route_config_get() -> Result<impl IntoResponse, ApiError> {
    // API
    match resalt_api::config::get_config(true).await {
        Ok(config) => Ok(Json(config)),
        Err(e) => Err(e),
    }
}
