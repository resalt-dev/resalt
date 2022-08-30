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
    .add_source(config::Environment::with_prefix("resalt").separator("_").ignore_empty(true))
    .set_default("salt.api.token", SYSTEM_TOKEN_FALLBACK.clone()).unwrap()
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
        let host = SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.host")
            .unwrap();
        let port = SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.port")
            .unwrap();
        let ldaps = SETTINGS
            .read()
            .unwrap()
            .get_bool("auth.ldap.ldaps")
            .unwrap();
        let proto = if ldaps { "ldaps" } else { "ldap" };
        format!("{}://{}:{}", proto, host, port)
    }

    pub fn auth_ldap_base_dn() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.basedn")
            .unwrap()
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
        SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.bind.dn")
            .unwrap()
    }

    pub fn auth_ldap_bind_password() -> String {
        let mut password = SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.bind.password")
            .unwrap();
        let password_file = SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.bind.passwordfile")
            .unwrap();
        match password_file.len() {
            0 => {}
            _ => {
                password = std::fs::read_to_string(password_file).unwrap();
            }
        }
        password
    }

    pub fn auth_ldap_user_filter() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.user.filter")
            .unwrap()
    }

    pub fn auth_ldap_user_attribute() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("auth.ldap.user.attribute")
            .unwrap()
    }

    pub fn auth_session_lifespan() -> u64 {
        SETTINGS
            .read()
            .unwrap()
            .get_int("auth.session.lifespan")
            .unwrap() as u64
    }

    pub fn database_url() -> String {
        // print all settings
        // log::info!("{:?}", SETTINGS.read().unwrap());

        // print out all env vars
        // for (key, value) in std::env::vars() {
        //     log::info!("{}={}", key, value);
        // }

        let username = SETTINGS
            .read()
            .unwrap()
            .get_string("database.username")
            .unwrap();
        let mut password = SETTINGS
            .read()
            .unwrap()
            .get_string("database.password")
            .unwrap();
        let password_file = SETTINGS
            .read()
            .unwrap()
            .get_string("database.passwordfile")
            .unwrap();
        match password_file.len() {
            0 => {}
            _ => {
                password = std::fs::read_to_string(password_file).unwrap();
            }
        }
        let host = SETTINGS
            .read()
            .unwrap()
            .get_string("database.host")
            .unwrap();
        let port = SETTINGS
            .read()
            .unwrap()
            .get_string("database.port")
            .unwrap();
        let database = SETTINGS
            .read()
            .unwrap()
            .get_string("database.database")
            .unwrap();

        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            username, password, host, port, database
        );
        return url;
    }

    pub fn salt_api_url() -> String {
        SETTINGS.read().unwrap().get_string("salt.api.url").unwrap()
    }

    pub fn salt_api_tls_skipverify() -> bool {
        SETTINGS
            .read()
            .unwrap()
            .get_bool("salt.api.tls.skipverify")
            .unwrap()
    }

    pub fn salt_api_system_service_token() -> String {
        let mut token = SETTINGS
            .read()
            .unwrap()
            .get_string("salt.api.token")
            .unwrap();
        let token_file = SETTINGS
            .read()
            .unwrap()
            .get_string("salt.api.tokenfile")
            .unwrap();
        match token_file.len() {
            0 => {}
            _ => {
                token = std::fs::read_to_string(token_file).unwrap();
            }
        }
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

    pub fn http_frontend_theme_color() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("http.frontend.theme.color")
            .unwrap()
    }

    pub fn http_frontend_proxy_target() -> String {
        SETTINGS
            .read()
            .unwrap()
            .get_string("http.frontend.proxy.target")
            .unwrap()
    }

    pub fn sub_path() -> String {
        "/resalt".to_string()
    }
}
