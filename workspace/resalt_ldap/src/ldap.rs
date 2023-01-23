use std::sync::Arc;

use ldap3::{Ldap, LdapConnAsync, LdapConnSettings, LdapError, Scope, SearchEntry};
use log::*;
use resalt_config::SConfig;
use rustls::ClientConfig;
use rustls_native_certs::load_native_certs;

pub struct LdapUser {
    pub dn: String,
    pub username: String,
    pub email: String,
    pub groups: Vec<String>,
}

lazy_static::lazy_static! {
    static ref LDAP_TLS_CONFIG: ClientConfig = {
        let certs = load_native_certs().unwrap();

        // Convert Vec<rustls_native_certs::Certificate> to RootCertStore
        let mut root_store = rustls::RootCertStore::empty();
        for cert in certs {
            root_store.add(&rustls::Certificate(cert.0)).unwrap();
        }

        let mut config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        if SConfig::auth_ldap_skip_tls_verify() {
            config
                .dangerous()
                .set_certificate_verifier(Arc::new(resalt_config::danger::NoCertificateVerification));
        }

        config
    };

    static ref LDAP_SETTINGS: LdapConnSettings = LdapConnSettings::new()
        .set_starttls(SConfig::auth_ldap_start_tls())
        .set_no_tls_verify(SConfig::auth_ldap_skip_tls_verify())
        .set_config(Arc::new(LDAP_TLS_CONFIG.to_owned()));
}

pub struct LdapHandler {}

impl LdapHandler {
    pub fn is_enabled() -> bool {
        SConfig::auth_ldap_enabled()
    }

    async fn create_connection() -> Result<Ldap, LdapError> {
        let ldap_url = SConfig::auth_ldap_url();
        warn!("Connecting to LDAP server: {}", ldap_url);
        let (conn, ldap) =
            LdapConnAsync::with_settings(LDAP_SETTINGS.to_owned(), &ldap_url).await?;
        ldap3::drive!(conn);

        Ok(ldap)
    }

    async fn create_system_connection() -> Result<Ldap, LdapError> {
        let mut ldap = match LdapHandler::create_connection().await {
            Ok(ldap) => ldap,
            Err(e) => {
                error!("Failed to connect to LDAP server: {:?}", e);
                return Err(e);
            }
        };

        let ldap_bind_dn = SConfig::auth_ldap_bind_dn();
        let ldap_bind_password = SConfig::auth_ldap_bind_password();

        match ldap.simple_bind(&ldap_bind_dn, &ldap_bind_password).await {
            Ok(res) => match res.success() {
                Ok(_) => {
                    debug!("Successfully connected with system account to LDAP.");
                }
                Err(e) => {
                    error!("Failed to bind to LDAP server: {:?}", e);
                    return Err(e);
                }
            },
            Err(e) => {
                error!("Failed to bind to LDAP server: {:?}", e);
                return Err(e);
            }
        };

        Ok(ldap)
    }

    async fn lookup_user(user_filter: Vec<String>) -> Result<Vec<LdapUser>, LdapError> {
        let mut service_ldap = LdapHandler::create_system_connection().await?;

        let base_dn = SConfig::auth_ldap_base_dn();
        let user_attribute = SConfig::auth_ldap_user_attribute();
        let email_attribute = String::from("mail"); // Same in both OpenLDAP and Active Directory
        let group_attribute = String::from("memberOf"); // Same in both OpenLDAP and Active Directory

        let mut users = Vec::new();

        for filter in user_filter {
            debug!("Searching LDAP for user with filter {}", &filter);
            let (rs, _res) = service_ldap
                .search(
                    &base_dn,
                    Scope::Subtree,
                    &filter,
                    vec![&user_attribute, &email_attribute, &group_attribute],
                )
                .await?
                .success()?;

            if rs.is_empty() {
                continue;
            }
            let entry = rs.get(0).unwrap().clone();
            let entry = SearchEntry::construct(entry);
            let dn = (&entry.dn).to_string();
            let username = entry
                .attrs
                .get(&user_attribute)
                .unwrap()
                .get(0)
                .unwrap()
                .clone();
            let email = entry
                .attrs
                .get(&email_attribute)
                .unwrap()
                .get(0)
                .unwrap()
                .clone();
            let groups = entry.attrs.get(&group_attribute).unwrap().clone();

            users.push(LdapUser {
                dn,
                username,
                email,
                groups,
            });
        }

        // Close Service connection
        service_ldap.unbind().await?;

        Ok(users)
    }

    /// Lookup a user by their username
    pub async fn lookup_user_by_username(username: &str) -> Result<Option<LdapUser>, LdapError> {
        let user_filter = SConfig::auth_ldap_user_filter().replace("%s", username);
        let users = LdapHandler::lookup_user(vec![user_filter]).await?;
        let user: Option<LdapUser> = users.into_iter().next();
        Ok(user)
    }

    /// Lookup a user by their DN
    pub async fn lookup_user_by_dn(dn: &str) -> Result<Option<LdapUser>, LdapError> {
        let user_filter = format!("(distinguishedName={})", dn);
        let users = LdapHandler::lookup_user(vec![user_filter]).await?;
        let user: Option<LdapUser> = users.into_iter().next();
        Ok(user)
    }

    /// Lookup a list of users by their DN
    pub async fn lookup_users_by_dn(dns: Vec<String>) -> Result<Vec<LdapUser>, LdapError> {
        let mut user_filter = Vec::new();
        for dn in dns {
            user_filter.push(format!("(distinguishedName={})", dn));
        }
        let users = LdapHandler::lookup_user(user_filter).await?;
        Ok(users)
    }

    /// Authenticate a user by their username and password
    pub async fn authenticate(
        username: &str,
        password: &str,
    ) -> Result<Option<LdapUser>, LdapError> {
        if !LdapHandler::is_enabled() {
            return Ok(None);
        }

        // Find user full DN
        let user = match LdapHandler::lookup_user_by_username(username).await? {
            Some(dn) => dn,
            None => return Ok(None),
        };

        // Bind with user credentials
        let mut user_ldap = LdapHandler::create_connection().await?;
        match user_ldap.simple_bind(&user.dn, password).await?.success() {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to authenticate user with DN {}: {:?}", user.dn, e);
                return Ok(None);
            }
        };
        debug!("Successfully authenticated user with DN {}", user.dn);

        // Close User connection
        user_ldap.unbind().await?;

        Ok(Some(user))
    }
}
