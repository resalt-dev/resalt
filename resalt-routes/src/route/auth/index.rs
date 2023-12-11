use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

pub async fn route_index_get() -> Result<impl IntoResponse, StatusCode> {
    Ok(Json("Hello API".to_owned()))
}
