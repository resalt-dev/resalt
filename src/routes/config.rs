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
    latestNews: Vec<String>,
    defaultThemeColor: String,
    defaultThemeDark: bool,
    enableThemeSwitching: bool,
}

pub async fn route_config_get() -> Result<impl Responder, ApiError> {
    let update_info = update::get_update_cache().await;
    let config = ApiConfig {
        authForwardEnabled: SConfig::auth_forward_enabled(),
        currentVersion: update::CURRENT_VERSION.to_string(),
        latestVersion: match update_info.version {
            Some(version) => version,
            None => {
                error!("Error getting latest version");
                "unknown".to_string()
            }
        },
        latestNews: match update_info.news {
            Some(news) => news,
            None => {
                error!("Error getting latest news");
                Vec::new()
            }
        },
        defaultThemeColor: SConfig::http_frontend_theme_color(),
        defaultThemeDark: SConfig::http_frontend_theme_dark(),
        enableThemeSwitching: SConfig::http_frontend_theme_enabled(),
    };
    Ok(web::Json(config))
}
