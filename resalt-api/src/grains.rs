use std::collections::HashMap;

use log::*;
use resalt_models::{ApiError, Filter};
use resalt_storage::StorageImpl;
use serde_json::Value;

pub fn search_grains(
    data: &Box<dyn StorageImpl>,
    path: String,
    filter: Option<String>,
) -> Result<HashMap<String, Value>, ApiError> {
    let path = path
        .starts_with('$')
        .then(|| path.clone())
        .unwrap_or(format!("$.{}", path));

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

    let minions = match data.list_minions(filters, None, None) {
        Ok(minions) => minions,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
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
        return Err(ApiError::InvalidRequest);
    }

    Ok(results)
}
