use std::sync::RwLock;

use log::*;
use rand::Rng;

pub struct SConfig {}

lazy_static::lazy_static! {
    static ref SYSTEM_TOKEN_FALLBACK: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(512)
                .map(|c| c.to_string())
                .collect::<String>();
    static ref SETTINGS: RwLock<config::Config> = RwLock::new(config::Config::builder()
    // load defaults from resalt.default.toml via include_str!
    .add_source(config::File::from_str(include_str!("../../../resalt.default.toml"), config::FileFormat::Toml))
    .add_source(config::File::with_name("/etc/resalt/resalt").required(false))
    .add_source(config::File::with_name("resalt").required(false))
    // Add in settings from the environment (with a prefix of RESALT)
    // Eg.. `RESALT_DEBUG=1 ./target/app` would set the `debug` key
    .add_source(config::Environment::with_prefix("resalt").separator("_").ignore_empty(true))
    .set_default("salt.api.token", SYSTEM_TOKEN_FALLBACK.clone()).unwrap()
    .build()
    .unwrap());
}

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

#[allow(dead_code)]
impl SConfig {
    pub fn auth_forward_enabled() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("auth.forward.enabled")
            .unwrap()
    }

    pub fn auth_ldap_enabled() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("auth.ldap.enabled")
            .unwrap()
    }

    pub fn auth_ldap_url() -> String {
        let host = strip_quotes!(SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.host")
            .unwrap());
        let port = strip_quotes!(SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.port")
            .unwrap());
        let ldaps = SETTINGS
            .read()
            .unwrap()
            .get_bool("auth.ldap.tls.ldaps")
            .unwrap();
        let proto = if ldaps { "ldaps" } else { "ldap" };
        let url = format!("{}://{}:{}", proto, host, port);
        warn!("LDAP URL: {}", url);
        return url;
    }

    pub fn auth_ldap_base_dn() -> String {
        strip_quotes!(SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.basedn")
            .unwrap())
    }

    pub fn auth_ldap_start_tls() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("auth.ldap.tls.starttls")
            .unwrap()
    }

    pub fn auth_ldap_skip_tls_verify() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("auth.ldap.tls.skipverify")
            .unwrap()
    }

    pub fn auth_ldap_bind_dn() -> String {
        strip_quotes!(SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.bind.dn")
            .unwrap())
    }

    pub fn auth_ldap_bind_password() -> String {
        // Read from passwordfile if set
        // Otherwise read from "password"

        let passwordfile = SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.bind.passwordfile")
            .unwrap();
        let password = match passwordfile.len() {
            0 => SETTINGS
                .read()
                .unwrap()
                .get_string("auth.ldap.bind.password")
                .unwrap(),
            _ => std::fs::read_to_string(passwordfile)
                .unwrap()
                .trim()
                .to_string(),
        };
        password
    }

    pub fn auth_ldap_user_filter() -> String {
        strip_quotes!(SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.user.filter")
            .unwrap())
    }

    pub fn auth_ldap_user_attribute() -> String {
        strip_quotes!(SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.user.attribute")
            .unwrap())
    }

    pub fn auth_session_lifespan() -> u64 {
        SETTINGS
            .read()
            .unwrap()
            .get_int("auth.session.lifespan")
            .unwrap() as u64
    }

    pub fn database_type() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("database.type")
            .unwrap()
    }

    pub fn database_username() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("database.username")
            .unwrap()
    }

    pub fn database_password() -> String {
        let passwordfile = strip_quotes!(SETTINGS
            .read()
            .unwrap()
            .get_string("database.passwordfile")
            .unwrap());
        match passwordfile.len() {
            0 => strip_quotes!(SETTINGS
                .read()
                .unwrap()
                .get_string("database.password")
                .unwrap()),
            _ => std::fs::read_to_string(passwordfile)
                .unwrap()
                .trim()
                .to_string(),
        }
    }

    pub fn database_host() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("database.host")
            .unwrap()
    }

    pub fn database_port() -> u16 {
        SETTINGS.read().unwrap().get_int("database.port").unwrap() as u16
    }

    pub fn database_database() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("database.database")
            .unwrap()
    }

    pub fn salt_api_url() -> String {
        strip_quotes!(SETTINGS.read().unwrap().get_string("salt.api.url").unwrap())
    }

    pub fn salt_api_tls_skipverify() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("salt.api.tls.skipverify")
            .unwrap()
    }

    pub fn salt_api_system_service_token() -> String {
        let tokenfile = strip_quotes!(SETTINGS
            .read()
            .unwrap()
            .get_string("salt.api.tokenfile")
            .unwrap());
        let token = match tokenfile.len() {
            0 => strip_quotes!(SETTINGS
                .read()
                .unwrap()
                .get_string("salt.api.token")
                .unwrap()),
            _ => std::fs::read_to_string(tokenfile)
                .unwrap()
                .trim()
                .to_string(),
        };
        token
    }

    pub fn http_port() -> u16 {
        SETTINGS.read().unwrap().get_int("http.port").unwrap() as u16
    }

    pub fn http_frontend_proxy_enabled() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("http.frontend.proxy.enabled")
            .unwrap()
    }

    pub fn http_frontend_theme_enabled() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("http.frontend.theme.enabled")
            .unwrap()
    }

    pub fn http_frontend_theme_color() -> String {
        strip_quotes!(SETTINGS
            .read()
            .unwrap()
            .get_string("http.frontend.theme.color")
            .unwrap())
    }

    pub fn http_frontend_proxy_target() -> String {
        strip_quotes!(SETTINGS
            .read()
            .unwrap()
            .get_string("http.frontend.proxy.target")
            .unwrap())
    }
}
