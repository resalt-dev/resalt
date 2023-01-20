use log::*;
use resalt_ldap::LdapUser;
use resalt_models::{ApiError, User};
use resalt_storage::StorageImpl;
use serde_json::Value;

pub fn refresh_user_permissions(data: &Box<dyn StorageImpl>, user: &User) -> Result<(), ApiError> {
    let groups = match data.list_permission_groups_by_user_id(&user.id) {
        Ok(groups) => groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    let mut perms: Vec<Value> = Vec::new();
    for group in groups {
        // Parse group.perms as json array
        let serdegroup: serde_json::Value = match serde_json::from_str(&group.perms) {
            Ok(serdegroup) => serdegroup,
            Err(e) => {
                error!("{:?}", e);
                return Err(ApiError::DatabaseError);
            }
        };
        let group_perms = match serdegroup.as_array() {
            Some(group_perms) => group_perms,
            None => continue,
        };
        for group_perm in group_perms {
            perms.push(group_perm.clone());
        }
    }
    let perms = Value::Array(perms);
    let perms = serde_json::to_string(&perms).unwrap();
    let mut user: User = user.clone();
    user.perms = perms;
    match data.update_user(&user) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::DatabaseError)
        }
    }
}

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
        match refresh_user_permissions(&data, &user) {
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

    return Ok(());
}
