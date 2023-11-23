use std::sync::RwLock;

use config::{Config, Environment, File, FileFormat};
use once_cell::sync::Lazy;
use rand::Rng;

/// Strip beginning and ending quote if both exist
#[macro_export]
macro_rules! inner_strip_quotes {
    ($s:expr) => {
        if $s.starts_with('"') && $s.ends_with('"') {
            $s[1..$s.len() - 1].to_string()
        } else if $s.starts_with('\'') && $s.ends_with('\'') {
            $s[1..$s.len() - 1].to_string()
        } else {
            $s.to_string()
        }
    };
}
pub use inner_strip_quotes as strip_quotes;

macro_rules! conf {
    ($s:expr) => {
        strip_quotes!(SETTINGS.read().unwrap().get_string($s).unwrap())
    };
}

static SYSTEM_TOKEN_FALLBACK: Lazy<String> = Lazy::new(|| {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(512)
        .map(|c| c.to_string())
        .collect::<String>()
});

static SETTINGS: Lazy<RwLock<Config>> = Lazy::new(|| {
    RwLock::new(
        Config::builder()
            // load defaults from resalt.default.toml via include_str!
            .add_source(File::from_str(
                include_str!("../../../resalt.default.toml"),
                FileFormat::Toml,
            ))
            .add_source(File::with_name("/etc/resalt/resalt").required(false))
            .add_source(File::with_name("resalt").required(false))
            // Add in settings from the environment (with a prefix of RESALT)
            // Eg.. `RESALT_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(
                Environment::with_prefix("resalt")
                    .separator("_")
                    .ignore_empty(true),
            )
            .set_default("salt.api.token", SYSTEM_TOKEN_FALLBACK.clone())
            .unwrap()
            .build()
            .unwrap(),
    )
});

static AUTH_FORWARD_ENABLED: Lazy<bool> =
    Lazy::new(|| conf!("auth.forward.enabled").parse().unwrap());

static AUTH_SESSION_LIFESPAN: Lazy<u64> =
    Lazy::new(|| conf!("auth.session.lifespan").parse().unwrap());

static DATABASE_TYPE: Lazy<String> = Lazy::new(|| conf!("database.type"));
static DATABASE_USERNAME: Lazy<String> = Lazy::new(|| conf!("database.username"));
static DATABASE_PASSWORDFILE: Lazy<String> = Lazy::new(|| conf!("database.passwordfile"));
static DATABASE_PASSWORD: Lazy<String> = Lazy::new(|| match DATABASE_PASSWORDFILE.clone().len() {
    0 => conf!("database.password"),
    _ => std::fs::read_to_string(DATABASE_PASSWORDFILE.clone())
        .unwrap()
        .trim()
        .to_string(),
});
static DATABASE_HOST: Lazy<String> = Lazy::new(|| conf!("database.host"));
static DATABASE_PORT: Lazy<u16> = Lazy::new(|| conf!("database.port").parse().unwrap());
static DATABASE_DATABASE: Lazy<String> = Lazy::new(|| conf!("database.database"));

static METRICS_ENABLED: Lazy<bool> = Lazy::new(|| conf!("metrics.enabled").parse().unwrap());

static SALT_API_URL: Lazy<String> = Lazy::new(|| conf!("salt.api.url"));
static SALT_API_TLS_SKIPVERIFY: Lazy<bool> =
    Lazy::new(|| conf!("salt.api.tls.skipverify").parse().unwrap());
// salt.api.token
static SALT_API_TOKENFILE: Lazy<String> = Lazy::new(|| conf!("salt.api.tokenfile"));
static SALT_API_TOKEN: Lazy<String> = Lazy::new(|| match SALT_API_TOKENFILE.clone().len() {
    0 => conf!("salt.api.token"),
    _ => std::fs::read_to_string(SALT_API_TOKENFILE.clone())
        .unwrap()
        .trim()
        .to_string(),
});
static HTTP_PORT: Lazy<u16> = Lazy::new(|| conf!("http.port").parse().unwrap());
static HTTP_FRONTEND_THEME_ENABLED: Lazy<bool> =
    Lazy::new(|| conf!("http.frontend.theme.enabled").parse().unwrap());
static HTTP_FRONTEND_THEME_COLOR: Lazy<String> = Lazy::new(|| conf!("http.frontend.theme.color"));

pub struct SConfig {}
impl SConfig {
    pub fn auth_forward_enabled() -> bool {
        *AUTH_FORWARD_ENABLED
    }

    pub fn auth_session_lifespan() -> u64 {
        *AUTH_SESSION_LIFESPAN
    }

    pub fn database_type() -> String {
        DATABASE_TYPE.clone()
    }

    pub fn database_username() -> String {
        DATABASE_USERNAME.clone()
    }

    pub fn database_password() -> String {
        DATABASE_PASSWORD.clone()
    }

    pub fn database_host() -> String {
        DATABASE_HOST.clone()
    }

    pub fn database_port() -> u16 {
        *DATABASE_PORT
    }

    pub fn database_database() -> String {
        DATABASE_DATABASE.clone()
    }

    pub fn metrics_enabled() -> bool {
        *METRICS_ENABLED
    }

    pub fn salt_api_url() -> String {
        SALT_API_URL.clone()
    }

    pub fn salt_api_tls_skipverify() -> bool {
        *SALT_API_TLS_SKIPVERIFY
    }

    pub fn salt_api_system_service_token() -> String {
        SALT_API_TOKEN.clone()
    }

    pub fn http_port() -> u16 {
        *HTTP_PORT
    }

    pub fn http_frontend_theme_enabled() -> bool {
        *HTTP_FRONTEND_THEME_ENABLED
    }

    pub fn http_frontend_theme_color() -> String {
        HTTP_FRONTEND_THEME_COLOR.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        // Ensure all the default configs parsed without error.
        // If this test fails, it means the config file is missing a default value.
        SConfig::auth_forward_enabled();
        SConfig::auth_session_lifespan();
        SConfig::database_type();
        SConfig::database_username();
        SConfig::database_password();
        SConfig::database_host();
        SConfig::database_port();
        SConfig::database_database();
        SConfig::metrics_enabled();
        SConfig::salt_api_url();
        SConfig::salt_api_tls_skipverify();
        SConfig::salt_api_system_service_token();
        SConfig::http_port();
        SConfig::http_frontend_theme_enabled();
        SConfig::http_frontend_theme_color();
    }
}
