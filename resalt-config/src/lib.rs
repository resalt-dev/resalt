mod util;
use once_cell::sync::Lazy;
use util::generate_random_token;
pub use util::strip_quotes;

static SYSTEM_TOKEN_FALLBACK: Lazy<String> = Lazy::new(generate_random_token);

// enum ResaltConfigKey {
//     AuthForwardEnabled,
//     AuthSessionLifespan,
//     DatabaseType,
//     DatabaseUsername,
//     DatabasePassword,
//     DatabaseHost,
//     DatabasePort,
//     DatabaseDatabase,
//     MetricsEnabled,
//     SaltApiUrl,
//     SaltApiTlsSkipverify,
//     SaltApiSystemServiceToken,
//     HttpPort,
//     HttpFrontendThemeEnabled,
//     HttpFrontendThemeColor,
// }

// impl ResaltConfigKey {
//     fn key(&self) -> &'static str {
//         match self {
//             ResaltConfigKey::AuthForwardEnabled => "RESALT_AUTH_FORWARD_ENABLED",
//             ResaltConfigKey::AuthSessionLifespan => "RESALT_AUTH_SESSION_LIFESPAN",
//             ResaltConfigKey::DatabaseType => "RESALT_DATABASE_TYPE",
//             ResaltConfigKey::DatabaseUsername => "RESALT_DATABASE_USERNAME",
//             ResaltConfigKey::DatabasePassword => "RESALT_DATABASE_PASSWORD",
//             ResaltConfigKey::DatabaseHost => "RESALT_DATABASE_HOST",
//             ResaltConfigKey::DatabasePort => "RESALT_DATABASE_PORT",
//             ResaltConfigKey::DatabaseDatabase => "RESALT_DATABASE_DATABASE",
//             ResaltConfigKey::MetricsEnabled => "RESALT_METRICS_ENABLED",
//             ResaltConfigKey::SaltApiUrl => "RESALT_SALT_API_URL",
//             ResaltConfigKey::SaltApiTlsSkipverify => "RESALT_SALT_API_TLS_SKIPVERIFY",
//             ResaltConfigKey::SaltApiSystemServiceToken => "RESALT_SALT_API_TOKEN",
//             ResaltConfigKey::HttpPort => "RESALT_HTTP_PORT",
//             ResaltConfigKey::HttpFrontendThemeEnabled => "RESALT_HTTP_FRONTEND_THEME_ENABLED",
//             ResaltConfigKey::HttpFrontendThemeColor => "RESALT_HTTP_FRONTEND_THEME_COLOR",
//         }
//     }
// }

#[inline]
#[must_use]
fn conf<T: std::str::FromStr>(key: &str, fallback: T) -> T
where
    T::Err: std::fmt::Debug,
{
    std::env::var(key)
        .ok()
        .map(|value| strip_quotes(&value))
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse().ok())
        .unwrap_or(fallback)
}

pub struct ResaltConfig {}
impl ResaltConfig {
    pub fn auth_forward_enabled() -> bool {
        conf("RESALT_AUTH_FORWARD_ENABLED", false)
    }

    pub fn auth_session_lifespan() -> u64 {
        conf("RESALT_AUTH_SESSION_LIFESPAN", 43200)
    }

    pub fn database_type() -> String {
        conf("RESALT_DATABASE_TYPE", "files".to_string())
    }

    pub fn database_username() -> String {
        conf("RESALT_DATABASE_USERNAME", "default".to_string())
    }

    pub fn database_password() -> String {
        let password_file = conf("RESALT_DATABASE_PASSWORDFILE", "".to_string());
        match password_file.len() {
            0 => conf("RESALT_DATABASE_PASSWORD", "resalt".to_string()),
            _ => std::fs::read_to_string(password_file)
                .unwrap()
                .trim()
                .to_string(),
        }
    }

    pub fn database_host() -> String {
        conf("RESALT_DATABASE_HOST", "docs/docker/filesdb".to_string())
    }

    pub fn database_port() -> u16 {
        conf("RESALT_DATABASE_PORT", 6379)
    }

    pub fn database_database() -> String {
        conf("RESALT_DATABASE_DATABASE", "0".to_string())
    }

    pub fn metrics_enabled() -> bool {
        conf("RESALT_METRICS_ENABLED", false)
    }

    pub fn salt_api_url() -> String {
        conf("RESALT_SALT_API_URL", "http://127.0.0.1:8080".to_string())
    }

    pub fn salt_api_tls_skipverify() -> bool {
        conf("RESALT_SALT_API_TLS_SKIPVERIFY", true)
    }

    pub fn salt_api_system_service_token() -> String {
        let token_file = conf("RESALT_SALT_API_TOKEN_FILE", "".to_string());
        match token_file.clone().len() {
            0 => {
                let token = conf("RESALT_SALT_API_TOKEN", "".to_string());
                if token.is_empty() {
                    SYSTEM_TOKEN_FALLBACK.clone()
                } else {
                    token
                }
            }
            _ => std::fs::read_to_string(token_file)
                .unwrap()
                .trim()
                .to_string(),
        }
    }

    pub fn http_port() -> u16 {
        conf("RESALT_HTTP_PORT", 8000)
    }

    pub fn http_frontend_theme_enabled() -> bool {
        conf("RESALT_HTTP_FRONTEND_THEME_ENABLED", true)
    }

    pub fn http_frontend_theme_color() -> String {
        conf("RESALT_HTTP_FRONTEND_THEME_COLOR", "primary".to_string())
    }
}
