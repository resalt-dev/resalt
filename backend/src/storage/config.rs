use std::sync::RwLock;

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
    .add_source(config::File::from_str(include_str!("../../resalt.default.toml"), config::FileFormat::Toml))
    .add_source(config::File::with_name("/etc/resalt/resalt").required(false))
    .add_source(config::File::with_name("resalt").required(false))
    // Add in settings from the environment (with a prefix of RESALT)
    // Eg.. `RESALT_DEBUG=1 ./target/app` would set the `debug` key
    .add_source(config::Environment::with_prefix("resalt"))
    .set_default("salt.api.system_service_token", SYSTEM_TOKEN_FALLBACK.clone()).unwrap()
    .build()
    .unwrap());
}

#[allow(dead_code)]
impl SConfig {
    pub fn auth_ldap_enabled() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("auth.ldap.enabled")
            .unwrap()
    }

    pub fn auth_ldap_url() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.url")
            .unwrap()
    }

    pub fn auth_ldap_start_tls() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("auth.ldap.start_tls")
            .unwrap()
    }

    pub fn auth_ldap_skip_tls_verify() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("auth.ldap.skip_tls_verify")
            .unwrap()
    }

    pub fn auth_ldap_bind_dn() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.bind_dn")
            .unwrap()
    }

    pub fn auth_ldap_bind_password() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.bind_password")
            .unwrap()
    }

    pub fn auth_ldap_base_dn() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.base_dn")
            .unwrap()
    }

    pub fn auth_ldap_user_filter() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.user_filter")
            .unwrap()
    }

    pub fn auth_ldap_user_attribute() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.user_attribute")
            .unwrap()
    }

    pub fn auth_user_session_lifespan() -> u64 {
        SETTINGS
            .read()
            .unwrap()
            .get_int("auth.user.session_lifespan")
            .unwrap() as u64
    }

    pub fn database_url() -> String {
        // print all settings
        log::info!("{:?}", SETTINGS.read().unwrap());

        // print out all env vars
        for (key, value) in std::env::vars() {
            log::info!("{}={}", key, value);
        }

        SETTINGS.read().unwrap().get_string("database.url").unwrap()
    }

    pub fn salt_api_url() -> String {
        SETTINGS.read().unwrap().get_string("salt.api.url").unwrap()
    }

    pub fn salt_api_disable_tls_verification() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("salt.api.disable_tls_verification")
            .unwrap()
    }

    pub fn salt_api_system_service_token() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("salt.api.system_service_token")
            .unwrap()
    }

    pub fn frontend_proxy_enabled() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("frontend.proxy.enabled")
            .unwrap()
    }

    pub fn frontend_proxy_target() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("frontend.proxy.target")
            .unwrap()
    }

    pub fn http_port() -> u16 {
        SETTINGS.read().unwrap().get_int("http.port").unwrap() as u16
    }

    pub fn sub_path() -> String {
        "/resalt".to_string()
    }
}
