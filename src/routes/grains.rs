use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::*;
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct MinionsListGetQuery {
    query: String,
    filter: Option<String>, // URL-encoded JSON
}

pub async fn route_grains_get(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Query<MinionsListGetQuery>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&auth.perms, P_MINION_GRAINEXPLORER)? {
        return Err(ApiError::Forbidden);
    }

    // Args
    let query = match urlencoding::decode(info.query.as_str()) {
        Ok(query) => query.to_string(),
        Err(e) => {
            error!("Failed to decode query: {}", e);
            return Err(ApiError::InvalidRequest);
        }
    };
    let query = query
        .starts_with('$')
        .then(|| query.clone())
        .unwrap_or(format!("$.{}", query));

    let filter = match info.filter.clone() {
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

        let grains: Vec<Value> = match jsonpath_lib::select(&grains, &query) {
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

    Ok(web::Json(results))
}
