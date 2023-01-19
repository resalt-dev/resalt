use ldap3::{Ldap, LdapConnAsync, LdapConnSettings, LdapError, Scope, SearchEntry};
use log::*;
use resalt_config::SConfig;

pub struct LdapUser {
    pub dn: String,
    pub username: String,
    pub email: String,
    pub groups: Vec<String>,
}

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

    async fn lookup_user(user_filter: String) -> Result<Option<LdapUser>, LdapError> {
        let mut service_ldap = LdapHandler::create_system_connection().await?;

        let base_dn = SConfig::auth_ldap_base_dn();
        let user_attribute = SConfig::auth_ldap_user_attribute();
        let email_attribute = String::from("mail"); // Same in both OpenLDAP and Active Directory
        let group_attribute = String::from("memberOf"); // Same in both OpenLDAP and Active Directory

        debug!("Searching LDAP for user with filter {}", &user_filter);
        let (rs, _res) = service_ldap
            .search(
                &base_dn,
                Scope::Subtree,
                &user_filter,
                vec![&user_attribute, &email_attribute, &group_attribute],
            )
            .await?
            .success()?;

        if rs.is_empty() {
            return Ok(None);
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

        // Close Service connection
        service_ldap.unbind().await?;

        Ok(Some(LdapUser {
            dn,
            username,
            email,
            groups,
        }))
    }

    /// Lookup a user by their username
    pub async fn lookup_user_by_username(query: &str) -> Result<Option<LdapUser>, LdapError> {
        let user_filter = SConfig::auth_ldap_user_filter().replace("%s", query);
        LdapHandler::lookup_user(user_filter).await
    }

    /// Lookup a user by their DN
    pub async fn lookup_user_by_dn(query: &str) -> Result<Option<LdapUser>, LdapError> {
        let user_filter = format!("(distinguishedName={})", query);
        LdapHandler::lookup_user(user_filter).await
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
