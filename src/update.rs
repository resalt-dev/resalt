use std::sync::{Arc, Mutex};

use awc::Client;
use cargo_toml::Manifest;
use lazy_static::lazy_static;
use log::warn;

pub(crate) static UPDATE_URL: &str =
    "https://raw.githubusercontent.com/Foorack/resalt/main/Cargo.toml";

struct UpdateCache {
    version: Option<String>,
}

lazy_static! {
    static ref CACHE: Arc<Mutex<UpdateCache>> = Arc::new(Mutex::new(UpdateCache { version: None }));
    static ref CURRENT_VERSION: String = env!("CARGO_PKG_VERSION").to_string();
}

async fn fetch_remote_version() -> Result<String, String> {
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
    let toml = match Manifest::from_str(&body_str) {
        Ok(toml) => toml,
        Err(e) => return Err(format!("Error parsing TOML: {}", e)),
    };

    // Get version
    let version = match toml.package {
        Some(package) => package.version,
        None => return Err(format!("Error parsing TOML: no package")),
    };

    if !version.eq(CURRENT_VERSION.as_str()) {
        warn!("========================================================");
        warn!("==  You are running an outdated version of Resalt!   ===");
        warn!("==  Please update to the latest version! Understand, ===");
        warn!("==  running an older version entails SECURITY risk.  ===");
        warn!("========================================================");
    }

    Ok(version)
}

pub(crate) async fn get_remote_version() -> Result<String, String> {
    let mut cache = CACHE.lock().unwrap();
    if let Some(version) = cache.version.clone() {
        return Ok(version);
    }
    let version = fetch_remote_version().await?;
    cache.version = Some(version.clone());
    Ok(version)
}
