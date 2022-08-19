use actix_web::{web, Responder, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ApiConfig {
    version: String,
}

pub async fn route_config_get() -> Result<impl Responder> {
    let config = ApiConfig {
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    Ok(web::Json(config))
}
