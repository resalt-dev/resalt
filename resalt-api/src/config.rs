use axum::http::StatusCode;
use log::error;
use resalt_config::ResaltConfig;
use resalt_update::{get_update_info, CURRENT_VERSION};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiConfig {
    #[serde(rename = "authForwardEnabled")]
    auth_forward_enabled: bool,
    #[serde(rename = "currentVersion")]
    current_version: String,
    #[serde(rename = "latestVersion")]
    latest_version: String,
    #[serde(rename = "latestNews")]
    latest_news: Vec<String>,
}

pub async fn get_config(use_cache: bool) -> Result<ApiConfig, StatusCode> {
    let update_info = match get_update_info(use_cache).await {
        Ok(update_info) => update_info,
        Err(e) => {
            error!("Error getting update info: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let config = ApiConfig {
        auth_forward_enabled: *ResaltConfig::AUTH_FORWARD_ENABLED,
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
    };
    Ok(config)
}
