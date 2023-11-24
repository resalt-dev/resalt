use axum::{extract::OriginalUri, response::IntoResponse};
use resalt_frontend::frontend_get;
use resalt_models::ApiError;

pub async fn route_frontend_get(
    OriginalUri(uri): OriginalUri,
) -> Result<impl IntoResponse, ApiError> {
    let path = uri.path().to_owned();
    let frontend = frontend_get(path);
    Ok(frontend.into_response())
}
