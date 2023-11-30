mod util;
use once_cell::sync::Lazy;
use util::generate_random_token;
pub use util::strip_quotes;

static SYSTEM_TOKEN_FALLBACK: Lazy<String> = Lazy::new(generate_random_token);

enum ResaltConfigKey {
    AuthForwardEnabled,
    AuthSessionLifespan,
    DatabaseType,
    DatabaseUsername,
    DatabasePassword,
    DatabaseHost,
    DatabasePort,
    DatabaseDatabase,
    MetricsEnabled,
    SaltApiUrl,
    SaltApiTlsSkipverify,
    SaltApiSystemServiceToken,
    HttpPort,
    HttpFrontendThemeEnabled,
    HttpFrontendThemeColor,
}

impl ResaltConfigKey {
    fn key(&self) -> &'static str {
        match self {
            ResaltConfigKey::AuthForwardEnabled => "RESALT_AUTH_FORWARD_ENABLED",
            ResaltConfigKey::AuthSessionLifespan => "RESALT_AUTH_SESSION_LIFESPAN",
            ResaltConfigKey::DatabaseType => "RESALT_DATABASE_TYPE",
            ResaltConfigKey::DatabaseUsername => "RESALT_DATABASE_USERNAME",
            ResaltConfigKey::DatabasePassword => "RESALT_DATABASE_PASSWORD",
            ResaltConfigKey::DatabaseHost => "RESALT_DATABASE_HOST",
            ResaltConfigKey::DatabasePort => "RESALT_DATABASE_PORT",
            ResaltConfigKey::DatabaseDatabase => "RESALT_DATABASE_DATABASE",
            ResaltConfigKey::MetricsEnabled => "RESALT_METRICS_ENABLED",
            ResaltConfigKey::SaltApiUrl => "RESALT_SALT_API_URL",
            ResaltConfigKey::SaltApiTlsSkipverify => "RESALT_SALT_API_TLS_SKIPVERIFY",
            ResaltConfigKey::SaltApiSystemServiceToken => "RESALT_SALT_API_TOKEN",
            ResaltConfigKey::HttpPort => "RESALT_HTTP_PORT",
            ResaltConfigKey::HttpFrontendThemeEnabled => "RESALT_HTTP_FRONTEND_THEME_ENABLED",
            ResaltConfigKey::HttpFrontendThemeColor => "RESALT_HTTP_FRONTEND_THEME_COLOR",
        }
    }

    fn fallback(&self) -> &'static str {
        match self {
            ResaltConfigKey::AuthForwardEnabled => "false",
            ResaltConfigKey::AuthSessionLifespan => "43200",
            ResaltConfigKey::DatabaseType => "files",
            ResaltConfigKey::DatabaseUsername => "default",
            ResaltConfigKey::DatabasePassword => "resalt",
            ResaltConfigKey::DatabaseHost => "docs/docker/filesdb",
            ResaltConfigKey::DatabasePort => "6379",
            ResaltConfigKey::DatabaseDatabase => "0",
            ResaltConfigKey::MetricsEnabled => "true",
            ResaltConfigKey::SaltApiUrl => "http://localhost:8080",
            ResaltConfigKey::SaltApiTlsSkipverify => "false",
            ResaltConfigKey::SaltApiSystemServiceToken => SYSTEM_TOKEN_FALLBACK.as_str(),
            ResaltConfigKey::HttpPort => "8000",
            ResaltConfigKey::HttpFrontendThemeEnabled => "true",
            ResaltConfigKey::HttpFrontendThemeColor => "primary",
        }
    }
}

#[must_use]
fn conf<T: std::str::FromStr>(rck: ResaltConfigKey) -> T
where
    T::Err: std::fmt::Debug,
{
    // Check if key_FILE env variable is set
    let key = rck.key();
    let key_file = format!("{}_FILE", key);
    let key_file = std::env::var(key_file).ok();
    println!("key_file: {:?}", key_file);
    if let Some(key_file) = key_file {
        if !key_file.is_empty() {
            let data = match std::fs::read_to_string(strip_quotes(&key_file)) {
                Ok(data) => data,
                Err(e) => {
                    panic!("Error reading {}: {}", &key_file, e);
                }
            };
            let data = data.trim();
            return match data.parse() {
                Ok(data) => data,
                Err(e) => {
                    panic!("Error parsing \"{}\": {:?}", data, e);
                }
            };
        }
    }
    // Fallback to key env variable
    let fallback = rck.fallback();
    std::env::var(key)
        .ok()
        .map(|value| strip_quotes(&value))
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse().ok())
        .unwrap_or(fallback.parse().unwrap())
}

#[allow(non_snake_case)]
pub mod ResaltConfig {
    use crate::ResaltConfigInternal;
    use once_cell::sync::Lazy;

    pub static AUTH_FORWARD_ENABLED: Lazy<bool> =
        Lazy::new(ResaltConfigInternal::auth_forward_enabled);
    pub static AUTH_SESSION_LIFESPAN: Lazy<u64> =
        Lazy::new(ResaltConfigInternal::auth_session_lifespan);
    pub static DATABASE_TYPE: Lazy<String> =
        Lazy::new(|| ResaltConfigInternal::database_type().to_lowercase());
    pub static DATABASE_USERNAME: Lazy<String> = Lazy::new(ResaltConfigInternal::database_username);
    pub static DATABASE_PASSWORD: Lazy<String> = Lazy::new(ResaltConfigInternal::database_password);
    pub static DATABASE_HOST: Lazy<String> = Lazy::new(ResaltConfigInternal::database_host);
    pub static DATABASE_PORT: Lazy<u16> = Lazy::new(ResaltConfigInternal::database_port);
    pub static DATABASE_DATABASE: Lazy<String> = Lazy::new(ResaltConfigInternal::database_database);
    pub static METRICS_ENABLED: Lazy<bool> = Lazy::new(ResaltConfigInternal::metrics_enabled);
    pub static SALT_API_URL: Lazy<String> = Lazy::new(ResaltConfigInternal::salt_api_url);
    pub static SALT_API_TLS_SKIPVERIFY: Lazy<bool> =
        Lazy::new(ResaltConfigInternal::salt_api_tls_skipverify);
    pub static SALT_API_SYSTEM_SERVICE_TOKEN: Lazy<String> =
        Lazy::new(ResaltConfigInternal::salt_api_system_service_token);
    pub static HTTP_PORT: Lazy<u16> = Lazy::new(ResaltConfigInternal::http_port);
    pub static HTTP_FRONTEND_THEME_ENABLED: Lazy<bool> =
        Lazy::new(ResaltConfigInternal::http_frontend_theme_enabled);
    pub static HTTP_FRONTEND_THEME_COLOR: Lazy<String> =
        Lazy::new(ResaltConfigInternal::http_frontend_theme_color);
}

pub struct ResaltConfigInternal {}
impl ResaltConfigInternal {
    fn auth_forward_enabled() -> bool {
        conf::<bool>(ResaltConfigKey::AuthForwardEnabled)
    }

    fn auth_session_lifespan() -> u64 {
        conf::<u64>(ResaltConfigKey::AuthSessionLifespan)
    }

    fn database_type() -> String {
        conf::<String>(ResaltConfigKey::DatabaseType)
    }

    fn database_username() -> String {
        conf::<String>(ResaltConfigKey::DatabaseUsername)
    }

    fn database_password() -> String {
        conf::<String>(ResaltConfigKey::DatabasePassword)
    }

    fn database_host() -> String {
        conf::<String>(ResaltConfigKey::DatabaseHost)
    }

    fn database_port() -> u16 {
        conf::<u16>(ResaltConfigKey::DatabasePort)
    }

    fn database_database() -> String {
        conf::<String>(ResaltConfigKey::DatabaseDatabase)
    }

    fn metrics_enabled() -> bool {
        conf::<bool>(ResaltConfigKey::MetricsEnabled)
    }

    fn salt_api_url() -> String {
        conf::<String>(ResaltConfigKey::SaltApiUrl)
    }

    fn salt_api_tls_skipverify() -> bool {
        conf::<bool>(ResaltConfigKey::SaltApiTlsSkipverify)
    }

    fn salt_api_system_service_token() -> String {
        conf::<String>(ResaltConfigKey::SaltApiSystemServiceToken)
    }

    fn http_port() -> u16 {
        conf::<u16>(ResaltConfigKey::HttpPort)
    }

    fn http_frontend_theme_enabled() -> bool {
        conf::<bool>(ResaltConfigKey::HttpFrontendThemeEnabled)
    }

    fn http_frontend_theme_color() -> String {
        conf::<String>(ResaltConfigKey::HttpFrontendThemeColor)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basic() {
        #![allow(clippy::bool_assert_comparison)]
        assert_eq!(*ResaltConfig::AUTH_FORWARD_ENABLED, false);
        assert_eq!(*ResaltConfig::AUTH_SESSION_LIFESPAN, 43200);
        assert_eq!(*ResaltConfig::DATABASE_TYPE, "files");
    }

    #[test]
    fn test_file_override() {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let tmp_path = &format!("/tmp/resalt-config-test-{}", time);

        std::env::set_var("RESALT_HTTP_PORT_FILE", tmp_path);

        std::fs::write(tmp_path, "1234").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100)); // Wait for file to be written

        let result = ResaltConfigInternal::http_port();

        std::fs::remove_file(tmp_path).unwrap();
        std::env::remove_var("RESALT_HTTP_PORT_FILE");

        assert_eq!(result, 1234);
    }

    #[test]
    fn test_fallback() {
        std::env::remove_var("RESALT_DATABASE_DATABASE");
        std::env::remove_var("RESALT_DATABASE_DATABASE_FILE");
        assert_eq!(ResaltConfigInternal::database_database(), "0");
    }
}
