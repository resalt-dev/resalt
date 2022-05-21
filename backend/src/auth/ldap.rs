use crate::prelude::*;
use ldap3::{Ldap, LdapConnAsync, LdapConnSettings, LdapError, Scope, SearchEntry};
use log::*;
use std::sync::{Arc, Mutex};

async fn create_connection() -> Result<Ldap, LdapError> {
    let ldap_url = SConfig::auth_ldap_url();
    let ldap_start_tls = SConfig::auth_ldap_start_tls();
    let ldap_skip_tls_verify = SConfig::auth_ldap_skip_tls_verify();

    let settings: LdapConnSettings = LdapConnSettings::new()
        .set_starttls(ldap_start_tls)
        .set_no_tls_verify(ldap_skip_tls_verify);

    let (conn, ldap) = LdapConnAsync::with_settings(settings, &ldap_url).await?;
    ldap3::drive!(conn);

    return Ok(ldap);
}

#[derive(Clone, Default)]
pub struct LdapHandler {
    service_ldap: Option<Arc<Mutex<Ldap>>>,
}

impl LdapHandler {
    pub async fn new() -> Self {
        let ldap_enabled = SConfig::auth_ldap_enabled();
        if !ldap_enabled {
            return LdapHandler::default();
        }

        let mut ldap = match create_connection().await {
            Ok(ldap) => ldap,
            Err(e) => {
                error!("Failed to connect to LDAP server: {:?}", e);
                error!("LDAP authentication is disabled!");
                return LdapHandler::default();
            }
        };

        let ldap_bind_dn = SConfig::auth_ldap_bind_dn();
        let ldap_bind_password = SConfig::auth_ldap_bind_password();

        match ldap.simple_bind(&ldap_bind_dn, &ldap_bind_password).await {
            Ok(res) => match res.success() {
                Ok(_) => {
                    info!("Successfully connected to LDAP server!");
                }
                Err(e) => {
                    error!("Failed to bind to LDAP server: {:?}", e);
                    error!("LDAP authentication is disabled!");
                    return LdapHandler::default();
                }
            },
            Err(e) => {
                error!("Failed to bind to LDAP server: {:?}", e);
                error!("LDAP authentication is disabled!");
                return LdapHandler::default();
            }
        };

        LdapHandler {
            service_ldap: Some(Arc::new(Mutex::new(ldap))),
        }
    }

    pub fn is_enabled(&self) -> bool {
        let ldap_enabled = SConfig::auth_ldap_enabled();
        return ldap_enabled && self.service_ldap.is_some();
    }

    /**
     * Authenticate a user with LDAP.
     *
     * @param username The username of the user.
     * @param password The password of the user.
     * @returns The user's unique identifier if authenticated, None otherwise.
     */
    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<String>, LdapError> {
        if !self.is_enabled() {
            return Ok(None);
        }
        let mut service_ldap = self.service_ldap.as_ref().unwrap().lock().unwrap();

        let base_dn = SConfig::auth_ldap_base_dn();
        let user_filter = SConfig::auth_ldap_user_filter();
        let user_filter = user_filter.replace("%s", username);
        let user_attribute = SConfig::auth_ldap_user_attribute();

        debug!("Searching LDAP for user with filter {}", &user_filter);

        let (rs, _res) = service_ldap
            .search(
                &base_dn,
                Scope::Subtree,
                &user_filter,
                vec![&user_attribute],
            )
            .await?
            .success()?;

        if rs.is_empty() {
            return Ok(None);
        }

        // Get first element
        let entry = rs.get(0).unwrap().clone();
        let entry = SearchEntry::construct(entry);
        let dn = &entry.dn;

        let mut user_ldap = create_connection().await?;
        match user_ldap.simple_bind(dn, password).await?.success() {
            Ok(_) => {
                debug!("Successfully authenticated user with DN {}", dn);
                let user_attribute = entry.attrs.get(&user_attribute).unwrap();
                return Ok(Some(user_attribute.get(0).unwrap().clone()));
            }
            Err(e) => {
                error!("Failed to authenticate user with DN {}: {:?}", dn, e);
                return Ok(None);
            }
        };
    }
}
