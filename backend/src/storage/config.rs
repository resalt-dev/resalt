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
    // load defaults from hibike.default.toml via include_str!
    .add_source(config::File::from_str(include_str!("../../hibike.default.toml"), config::FileFormat::Toml))
    .add_source(config::File::with_name("/etc/hibike/config").required(false))
    .add_source(config::File::with_name("hibike").required(false))
    // Add in settings from the environment (with a prefix of HIBIKE)
    // Eg.. `HIBIKE_DEBUG=1 ./target/app` would set the `debug` key
    .add_source(config::Environment::with_prefix("HIBIKE"))
    .set_default("salt.api.system_service_token", SYSTEM_TOKEN_FALLBACK.clone()).unwrap()
    .build()
    .unwrap());
}

#[allow(dead_code)]
impl SConfig {
    pub fn database_url() -> String {
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

    pub fn reverse_proxy() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("frontend.reverse_proxy")
            .unwrap()
    }

    pub fn reverse_proxy_target() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("frontend.reverse_proxy_target")
            .unwrap()
    }

    pub fn sub_path() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("http.sub_path")
            .unwrap()
    }

    pub fn user_session_lifespan() -> u64 {
        SETTINGS
            .read()
            .unwrap()
            .get_int("user.session_lifespan")
            .unwrap() as u64
    }

    pub fn user_login_max_tries() -> u64 {
        SETTINGS
            .read()
            .unwrap()
            .get_int("user.login_max_tries")
            .unwrap() as u64
    }

    pub fn user_login_timeout() -> u64 {
        SETTINGS
            .read()
            .unwrap()
            .get_int("user.login_timeout")
            .unwrap() as u64
    }
}
