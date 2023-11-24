use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_models::*;
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::Deserialize;
use serde_json::{json, Value};

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
    if !has_resalt_permission(&auth.perms, P_MINION_GRAINEXPLORER)? {
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
    let path = path
        .starts_with('$')
        .then(|| path.clone())
        .unwrap_or(format!("$.{}", path));

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

    let filters: Vec<Filter> = match filter {
        Some(filter) => match serde_json::from_str(&filter) {
            Ok(filters) => filters,
            Err(e) => {
                error!("Failed to parse filter: {}", e);
                return Err(ApiError::InvalidRequest);
            }
        },
        None => vec![],
    };

    let minions = match data.list_minions(filters, None, Some(0), None) {
        Ok(minions) => minions,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let mut results: Value = json!({});

    let mut error = false;
    for minion in minions {
        let grains = match minion.grains {
            Some(ref grains) => grains,
            None => continue,
        };

        let grains: Value = match serde_json::from_str(grains) {
            Ok(grains) => grains,
            Err(e) => {
                error!("Failed to parse grains: {}", e);
                continue;
            }
        };

        let grains: Vec<Value> = match jsonpath_lib::select(&grains, &path) {
            Ok(grains) => grains.into_iter().map(|v| v.to_owned()).collect(),
            Err(e) => {
                warn!("Failed to extract grains: {}", e);
                error = true;
                continue;
            }
        };

        results
            .as_object_mut()
            .unwrap()
            .insert(minion.id, Value::Array(grains));
    }

    // Check if results is empty object && error
    if results.is_object() && results.as_object().unwrap().is_empty() && error {
        return Err(ApiError::InternalErrorMessage(
            "Failed to extract grains".to_string(),
        ));
    }

    Ok(Json(results))
}
