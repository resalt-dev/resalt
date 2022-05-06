use actix_web::{web, Responder, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ApiConfig {
    health: String,
}

pub async fn route_config_get() -> Result<impl Responder> {
    let config = ApiConfig {
        health: "ok".to_string(),
    };
    Ok(web::Json(config))
}
