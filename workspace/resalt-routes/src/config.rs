use actix_web::{get, web, Responder, Result};
use log::error;
use resalt_config::SConfig;
use resalt_models::ApiError;
use resalt_updater::{get_update_cache, CURRENT_VERSION};
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct ApiConfig {
    authForwardEnabled: bool,
    currentVersion: String,
    latestVersion: String,
    latestNews: Vec<String>,
    themeDefaultColor: String,
    themeEnableSwitching: bool,
}

#[get("/config")]
pub async fn route_config_get() -> Result<impl Responder, ApiError> {
    let update_info = get_update_cache(false).await;
    let config = ApiConfig {
        authForwardEnabled: SConfig::auth_forward_enabled(),
        currentVersion: CURRENT_VERSION.to_string(),
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
        themeDefaultColor: SConfig::http_frontend_theme_color(),
        themeEnableSwitching: SConfig::http_frontend_theme_enabled(),
    };
    Ok(web::Json(config))
}
