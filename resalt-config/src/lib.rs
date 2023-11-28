use once_cell::sync::Lazy;
use rand::Rng;

pub fn strip_quotes(s: &str) -> String {
    #[allow(clippy::if_same_then_else)]
    if s.starts_with('"') && s.ends_with('"') {
        s[1..s.len() - 1].to_string()
    } else if s.starts_with('\'') && s.ends_with('\'') {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

static SYSTEM_TOKEN_FALLBACK: Lazy<String> = Lazy::new(|| {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(512)
        .map(|c| c.to_string())
        .collect::<String>()
});

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

pub struct SConfig {}
impl SConfig {
    pub fn auth_forward_enabled() -> bool {
        conf("RESALT_AUTH_FORWARD_ENABLED", false)
    }

    pub fn auth_session_lifespan() -> u64 {
        conf("RESALT_AUTH_SESSION_LIFESPAN", 43200)
    }

    pub fn database_type() -> String {
        conf("RESALT_DATABASE_TYPE", "redis".to_string())
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
        conf("RESALT_DATABASE_HOST", "redis".to_string())
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
        conf("RESALT_SALT_API_URL", "https://master:8080".to_string())
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
