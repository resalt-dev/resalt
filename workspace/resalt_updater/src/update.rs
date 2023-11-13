use std::sync::{Arc, Mutex};

use awc::Client;
use lazy_static::lazy_static;
use log::*;
use toml::Table;

const UPDATE_URL: &str = "https://secure.resalt.dev/Cargo.toml";

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub version: Option<String>,
    pub news: Option<Vec<String>>,
    fetch_time: i64,
}

lazy_static! {
    static ref CACHE: Arc<Mutex<UpdateInfo>> = Arc::new(Mutex::new(UpdateInfo {
        version: None,
        news: None,
        fetch_time: 0,
    }));
    pub static ref CURRENT_VERSION: String = env!("CARGO_PKG_VERSION").to_string();
}

async fn fetch_remote_info() -> Result<UpdateInfo, String> {
    // Use awc client
    let client = Client::new();

    let mut resp = match client.get(UPDATE_URL).send().await {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Error checking latest version: {}", e)),
    };
    let body = match resp.body().await {
        Ok(body) => body,
        Err(e) => {
            return Err(format!(
                "Error reading body when checking latest version: {}",
                e
            ))
        }
    };
    let body_str = match String::from_utf8(body.to_vec()) {
        Ok(body_str) => body_str,
        Err(e) => {
            return Err(format!(
                "Error decoding body when checking latest version: {}",
                e
            ))
        }
    };
    // Parse TOML
    let cargo = match body_str.parse::<Table>() {
        Ok(cargo) => cargo,
        Err(e) => return Err(format!("Error parsing TOML: {}", e)),
    };

    // Get version
    let package = match cargo.get("package") {
        Some(package) => package,
        None => return Err("Error parsing TOML: no package".to_string()),
    };
    let version: String = match package.get("version") {
        Some(version) => match version.as_str() {
            Some(version) => version.to_string(),
            None => return Err("Error parsing TOML: version is not a string".to_string()),
        },
        None => return Err("Error parsing TOML: no version".to_string()),
    };

    // Get news (package.metadata.resalt.news)
    let metadata = match package.get("metadata") {
        Some(metadata) => metadata,
        None => return Err("Error parsing TOML: no metadata".to_string()),
    };
    let resalt = match metadata.get("resalt") {
        Some(resalt) => resalt,
        None => return Err("Error parsing TOML: no \"resalt\" metadata".to_string()),
    };
    let news = match resalt.get("news") {
        Some(news) => news,
        None => return Err("Error parsing TOML: no \"resalt.news\" metadata".to_string()),
    };
    // News is a string array
    let news = match news.as_array() {
        Some(news) => news,
        None => return Err("Error parsing TOML: \"resalt.news\" is not an array".to_string()),
    };
    let news: Vec<String> = news
        .iter()
        .map(|news| match news.as_str() {
            Some(news) => news.to_string(),
            None => "Error parsing TOML: \"resalt.news\" is not an array of strings".to_string(),
        })
        .collect();

    if !version.eq(CURRENT_VERSION.as_str()) {
        warn!("========================================================");
        warn!("==  You are running an outdated version of Resalt!   ===");
        warn!("==  Please update to the latest version! Understand, ===");
        warn!("==  running an older version entails SECURITY risk.  ===");
        warn!("========================================================");
        warn!("==  Current version: {}", CURRENT_VERSION.as_str());
        warn!("==  Latest version: {}", version);
        warn!("========================================================");
    }

    Ok(UpdateInfo {
        version: Some(version),
        news: Some(news),
        // current timestamp
        fetch_time: chrono::Utc::now().timestamp(),
    })
}

pub async fn get_update_cache(force: bool) -> UpdateInfo {
    // Drop CACHE MutexGuard lock inbetween of fetching remote version and setting it in the cache
    if !force {
        let update_info = CACHE.lock().unwrap();
        let current_time = chrono::Utc::now().timestamp();
        let time_diff = current_time - update_info.fetch_time;
        // Check if 1 hour + 5 minutes has passed
        if update_info.version.is_some() && update_info.news.is_some() && time_diff < 3900 {
            return update_info.clone();
        }
    }
    let update_info = match fetch_remote_info().await {
        Ok(update_info) => update_info,
        Err(e) => {
            error!("Error fetching remote version: {}", e);
            return UpdateInfo {
                version: None,
                news: None,
                fetch_time: 0,
            };
        }
    };
    {
        let mut cache = CACHE.lock().unwrap();
        cache.version = update_info.version.clone();
        cache.news = update_info.news.clone();
    }
    update_info
}
