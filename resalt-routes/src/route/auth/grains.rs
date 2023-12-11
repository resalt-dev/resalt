use crate::permission::*;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_api::{grain::search_grains, StatusCode};
use resalt_models::*;
use resalt_storage::Storage;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GrainsGetQuery {
    query: String,
    filter: Option<String>, // URL-encoded JSON
}

pub async fn route_grains_get(
    query: Query<GrainsGetQuery>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_MINION_GRAINEXPLORER)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // Args
    let path = match urlencoding::decode(query.query.as_str()) {
        Ok(q) => q.to_string(),
        Err(e) => {
            error!("Failed to decode q: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let filter = match &query.filter {
        Some(filter) => Some(match urlencoding::decode(filter.as_str()) {
            Ok(filter) => filter.to_string(),
            Err(e) => {
                error!("Failed to decode filter: {}", e);
                return Err(StatusCode::BAD_REQUEST);
            }
        }),
        None => None,
    };

    // API
    let results = search_grains(&data, path, filter)?;

    Ok(Json(results))
}
