use actix_web::{web, Responder, Result};
use log::error;
use serde::Serialize;

use crate::update;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct ApiConfig {
    currentVersion: String,
    latestVersion: String,
}

pub async fn route_config_get() -> Result<impl Responder> {
    let config = ApiConfig {
        currentVersion: update::CURRENT_VERSION.to_string(),
        latestVersion: match update::get_remote_version().await {
            Ok(version) => version,
            Err(e) => {
                error!("{}", e);
                "unknown".to_string()
            }
        },
    };
    Ok(web::Json(config))
}
