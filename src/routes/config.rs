use actix_web::{web, Responder, Result};
use log::error;
use resalt_config::SConfig;
use serde::Serialize;

use crate::{components::ApiError, update};

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct ApiConfig {
    authForwardEnabled: bool,
    currentVersion: String,
    latestVersion: String,
    defaultThemeColor: String,
    defaultThemeDark: bool,
    enableThemeSwitching: bool,
}

pub async fn route_config_get() -> Result<impl Responder, ApiError> {
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
        defaultThemeDark: SConfig::http_frontend_theme_dark(),
        enableThemeSwitching: SConfig::http_frontend_theme_enabled(),
    };
    Ok(web::Json(config))
}
