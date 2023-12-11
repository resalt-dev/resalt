use axum::http::StatusCode;
use log::*;
use resalt_models::*;
use resalt_storage::Storage;
use serde_json::Value;
use std::collections::HashMap;

pub fn search_grains(
    data: &Storage,
    path: String,
    filter: Option<String>,
) -> Result<HashMap<String, Value>, StatusCode> {
    let path = path
        .starts_with('$')
        .then(|| path.clone())
        .unwrap_or(format!("$.{}", path));

    let filters: Vec<Filter> = match filter {
        Some(filter) => match serde_json::from_str(&filter) {
            Ok(filters) => filters,
            Err(e) => {
                error!("Failed to parse filter: {}", e);
                return Err(StatusCode::BAD_REQUEST);
            }
        },
        None => vec![],
    };

    let minions = match data.list_minions(filters, None, None) {
        Ok(minions) => minions,
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut results: HashMap<String, Value> = HashMap::new();

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

        results.insert(minion.id, Value::Array(grains));
    }

    // Check if results is empty object && error
    if results.is_empty() && error {
        error!("Failed to extract grains");
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(results)
}
