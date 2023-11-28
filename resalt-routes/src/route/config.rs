use axum::response::{IntoResponse, Json};
use log::error;
use resalt_config::SConfig;
use resalt_models::ApiError;
use resalt_updater::{get_update_cache, CURRENT_VERSION};
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct ApiConfig {
    #[serde(rename = "authForwardEnabled")]
    auth_forward_enabled: bool,
    #[serde(rename = "currentVersion")]
    current_version: String,
    #[serde(rename = "latestVersion")]
    latest_version: String,
    #[serde(rename = "latestNews")]
    latest_news: Vec<String>,
    #[serde(rename = "themeDefaultColor")]
    theme_default_color: String,
    #[serde(rename = "themeEnableSwitching")]
    theme_enable_switching: bool,
}

pub async fn route_config_get() -> Result<impl IntoResponse, ApiError> {
    let update_info = get_update_cache();
    let config = ApiConfig {
        auth_forward_enabled: SConfig::auth_forward_enabled(),
        current_version: CURRENT_VERSION.to_string(),
        latest_version: match update_info.version {
            Some(version) => version,
            None => {
                error!("Error getting latest version");
                "unknown".to_string()
            }
        },
        latest_news: match update_info.news {
            Some(news) => news,
            None => {
                error!("Error getting latest news");
                Vec::new()
            }
        },
        theme_default_color: SConfig::http_frontend_theme_color(),
        theme_enable_switching: SConfig::http_frontend_theme_enabled(),
    };
    Ok(Json(config))
}