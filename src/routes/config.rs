use actix_web::{web, Responder, Result};
use log::error;
use serde::Serialize;

use crate::{prelude::SConfig, update};

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct ApiConfig {
    authForwardEnabled: bool,
    currentVersion: String,
    latestVersion: String,
    defaultThemeColor: String,
    enableThemeSwitching: bool,
}

pub async fn route_config_get() -> Result<impl Responder> {
    let config = ApiConfig {
        authForwardEnabled: SConfig::auth_forward_enabled(),
        currentVersion: update::CURRENT_VERSION.to_string(),
        latestVersion: match update::get_remote_version().await {
            Ok(version) => version,
            Err(e) => {
                error!("{}", e);
                "unknown".to_string()
            }
        },
        defaultThemeColor: SConfig::http_frontend_theme_color(),
        enableThemeSwitching: SConfig::http_frontend_theme_enabled(),
    };
    Ok(web::Json(config))
}
