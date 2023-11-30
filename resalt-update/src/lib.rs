use std::thread;
use std::time::Duration;

use log::*;
use once_cell::sync::Lazy;
use reqwest::Client;

const UPDATE_URL: &str = "https://secure.resalt.dev/Cargo.toml";

#[derive(Default, Debug, Clone)]
pub struct UpdateInfo {
    pub version: Option<String>,
    pub news: Option<Vec<String>>,
}

static CACHE: Lazy<std::sync::Mutex<UpdateInfo>> = Lazy::new(|| {
    std::sync::Mutex::new(UpdateInfo {
        version: None,
        news: None,
    })
});

pub static CURRENT_VERSION: Lazy<String> = Lazy::new(|| {
    include_str!("../../Cargo.toml")
        .lines()
        .find(|line| line.starts_with("version = "))
        .and_then(|line| line.split('=').nth(1))
        .map(|v| v.trim())
        .map(|v| v.trim_matches('"'))
        .unwrap_or("unknown")
        .to_string()
});

async fn fetch_update_info(client: &Client) -> Result<UpdateInfo, String> {
    let resp = client
        .get(UPDATE_URL)
        .send()
        .await
        .map_err(|e| format!("Error sending request: {}", e))?;

    let body = match resp.text().await {
        Ok(body) => body,
        Err(e) => {
            return Err(format!(
                "Error reading body when checking latest version: {}",
                e
            ))
        }
    };

    let cargo = match toml::from_str::<toml::Value>(&body) {
        Ok(cargo) => cargo,
        Err(e) => return Err(format!("Error parsing TOML: {}", e)),
    };

    let package = match cargo.get("package") {
        Some(v) => v,
        None => return Err("Error parsing TOML: no package".to_string()),
    };

    let version: String = match package.get("version") {
        Some(version) => match version.as_str() {
            Some(version) => version.to_string(),
            None => return Err("Error parsing TOML: version is not a string".to_string()),
        },
        None => return Err("Error parsing TOML: no version".to_string()),
    };

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
    })
}

pub async fn update_loop() {
    let client = Client::new();
    loop {
        let update_info = match fetch_update_info(&client).await {
            Ok(update_info) => update_info,
            Err(e) => {
                error!("Error fetching remote version: {}", e);
                UpdateInfo::default()
            }
        };

        // Update the cache
        let mut cache = CACHE.lock().unwrap();
        *cache = update_info;

        // Sleep for some time before checking for updates again
        thread::sleep(Duration::from_secs(60 * 60));
    }
}

pub async fn get_update_info(use_cache: bool) -> Result<UpdateInfo, String> {
    if use_cache {
        let update_info = CACHE.lock().unwrap();
        Ok(update_info.clone())
    } else {
        let client = Client::new();
        match fetch_update_info(&client).await {
            Ok(update_info) => Ok(update_info),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_version() {
        assert!(CURRENT_VERSION.len() >= 5); // x.x.x
    }

    #[tokio::test]
    async fn test_fetch_update_info() {
        let client = Client::new();
        let update_info = fetch_update_info(&client).await.unwrap();
        assert!(update_info.version.is_some());
        assert!(update_info.version.unwrap().len() >= 5); // x.x.x
        assert!(update_info.news.is_some());
        assert!(!update_info.news.unwrap().is_empty());
    }
}
