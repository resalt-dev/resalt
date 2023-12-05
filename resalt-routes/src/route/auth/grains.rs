use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_api::grain::search_grains;
use resalt_models::*;
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GrainsGetQuery {
    query: String,
    filter: Option<String>, // URL-encoded JSON
}

pub async fn route_grains_get(
    query: Query<GrainsGetQuery>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_GRAINEXPLORER)? {
        return Err(ApiError::Forbidden);
    }

    // Args
    let path = match urlencoding::decode(query.query.as_str()) {
        Ok(q) => q.to_string(),
        Err(e) => {
            error!("Failed to decode q: {}", e);
            return Err(ApiError::InvalidRequest);
        }
    };
    let filter = match &query.filter {
        Some(filter) => Some(match urlencoding::decode(filter.as_str()) {
            Ok(filter) => filter.to_string(),
            Err(e) => {
                error!("Failed to decode filter: {}", e);
                return Err(ApiError::InvalidRequest);
            }
        }),
        None => None,
    };

    // API
    let results = search_grains(&data, path, filter)?;

    Ok(Json(results))
}
