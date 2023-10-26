use ldap3::{Ldap, LdapConnAsync, LdapConnSettings, LdapError, Scope, SearchEntry};
use log::*;
use resalt_config::SConfig;
use resalt_models::{ApiError, User};
use resalt_storage::StorageImpl;

pub struct LdapUser {
    pub dn: String,
    pub username: String,
    pub email: String,
    pub groups: Vec<String>,
}

lazy_static::lazy_static! {
    static ref LDAP_SETTINGS: LdapConnSettings = LdapConnSettings::new()
        .set_starttls(SConfig::auth_ldap_start_tls())
        .set_no_tls_verify(SConfig::auth_ldap_skip_tls_verify());
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
            let dn = entry.dn.to_string();
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

#[allow(clippy::borrowed_box)]
pub fn sync_ldap_groups(
    data: &Box<dyn StorageImpl>,
    user: &User,
    ldap_user: Option<&LdapUser>,
) -> Result<(), ApiError> {
    let mut user_permission_groups = match data.list_permission_groups_by_user_id(&user.id) {
        Ok(groups) => groups,
        Err(e) => {
            error!(
                "Failed to get permission groups for user {}: {:?}",
                user.username, e
            );
            return Err(ApiError::DatabaseError);
        }
    };
    user_permission_groups.retain(|pg| pg.ldap_sync.is_some());

    let mut changed = false;
    if let Some(ldap_user) = ldap_user {
        // Add user to the groups he SHOULD be in
        for ldap_group_dn in &ldap_user.groups {
            // Check if they are in the group by looping over user_permission_groups
            let pgu = user_permission_groups
                .iter()
                .find(|pg| pg.name == ldap_group_dn.clone());

            // User is not in the group, try add them
            if pgu.is_none() {
                let pg = match data.get_permission_group_by_ldap_sync(ldap_group_dn) {
                    Ok(pg) => pg,
                    Err(e) => {
                        error!(
                            "Failed to get permission group for LDAP group {}: {:?}",
                            ldap_group_dn, e
                        );
                        return Err(ApiError::DatabaseError);
                    }
                };
                if let Some(pg) = pg {
                    match data.insert_permission_group_user(&user.id, &pg.id) {
                        Ok(_) => {
                            info!("Added user {} to group {}", user.username, pg.name);
                            changed = true;
                        }
                        Err(e) => {
                            error!(
                                "Failed to add user {} to group {}: {:?}",
                                user.username, pg.name, e
                            );
                            return Err(ApiError::DatabaseError);
                        }
                    }
                } else {
                    // They are in an LDAP group which doesn't exist in our system, do nothing.
                }
            }
        }

        // Remove the user from the groups they SHOULD NOT be in
        for pg in user_permission_groups {
            if !ldap_user.groups.contains(&pg.ldap_sync.unwrap()) {
                match data.delete_permission_group_user(&user.id, &pg.id) {
                    Ok(_) => {
                        info!("Removed user {} from group {}", user.username, pg.name);
                        changed = true;
                    }
                    Err(e) => {
                        error!(
                            "Failed to remove user {} from group {}: {:?}",
                            user.username, pg.name, e
                        );
                        return Err(ApiError::DatabaseError);
                    }
                }
            }
        }
    } else {
        // User not found in LDAP, remove all their groups
        warn!(
            "User {} not found in LDAP, removing all their groups",
            user.username
        );
        for pg in user_permission_groups {
            match data.delete_permission_group_user(&user.id, &pg.id) {
                Ok(_) => {
                    info!("Removed user {} from group {}", user.username, pg.name);
                    changed = true;
                }
                Err(e) => {
                    error!(
                        "Failed to remove user {} from group {}: {:?}",
                        user.username, pg.name, e
                    );
                }
            }
        }
    }

    if changed {
        // Update user-cached permissions
        match data.refresh_user_permissions(user) {
            Ok(_) => {}
            Err(e) => {
                error!(
                    "Failed to update user {} permissions: {:?}",
                    user.username, e
                );
                return Err(e);
            }
        }
    }

    Ok(())
}
