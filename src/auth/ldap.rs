use crate::prelude::*;
use ldap3::{Ldap, LdapConnAsync, LdapConnSettings, LdapError, Scope, SearchEntry};
use log::*;

pub struct LdapHandler {}

impl LdapHandler {
    pub fn is_enabled() -> bool {
        SConfig::auth_ldap_enabled()
    }

    async fn create_connection() -> Result<Ldap, LdapError> {
        let ldap_url = SConfig::auth_ldap_url();
        let ldap_start_tls = SConfig::auth_ldap_start_tls();
        let ldap_skip_tls_verify = SConfig::auth_ldap_skip_tls_verify();

        let settings: LdapConnSettings = LdapConnSettings::new()
            // .set_conn_timeout(None)
            .set_starttls(ldap_start_tls)
            .set_no_tls_verify(ldap_skip_tls_verify);

        let (conn, ldap) = LdapConnAsync::with_settings(settings, &ldap_url).await?;
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

    async fn find_dn(username: &str) -> Result<Option<(String, String)>, LdapError> {
        let mut service_ldap = LdapHandler::create_system_connection().await?;

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
        let entry = rs.get(0).unwrap().clone();
        let entry = SearchEntry::construct(entry);
        let dn = (&entry.dn).to_string();
        let user_id = entry
            .attrs
            .get(&user_attribute)
            .unwrap()
            .get(0)
            .unwrap()
            .clone();
        Ok(Some((dn, user_id)))
    }

    /**
     * Authenticate a user with LDAP.
     *
     * @param username The username of the user.
     * @param password The password of the user.
     * @returns The user's unique identifier if authenticated, None otherwise.
     */
    pub async fn authenticate(username: &str, password: &str) -> Result<Option<String>, LdapError> {
        if !LdapHandler::is_enabled() {
            return Ok(None);
        }

        let (dn, user_id) = match LdapHandler::find_dn(username).await? {
            Some(dn) => dn,
            None => return Ok(None),
        };

        let mut user_ldap = LdapHandler::create_connection().await?;
        let user_id = match user_ldap.simple_bind(&dn, password).await?.success() {
            Ok(_) => {
                debug!("Successfully authenticated user with DN {}", dn);
                Some(user_id)
            }
            Err(e) => {
                error!("Failed to authenticate user with DN {}: {:?}", dn, e);
                None
            }
        };

        // Close User connection
        user_ldap.unbind().await?;

        Ok(user_id)
    }
}
