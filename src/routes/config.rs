use actix_web::{web, Responder, Result};
use serde::Serialize;

use crate::update;

#[derive(Debug, Serialize)]
struct ApiConfig {
    currentVersion: String,
    latestVersion: String,
}

pub(crate) async fn route_config_get() -> Result<impl Responder> {
    let config = ApiConfig {
        currentVersion: update::CURRENT_VERSION.to_string(),
        latestVersion: update::get_remote_version().await?,
    };
    Ok(web::Json(config))
}
