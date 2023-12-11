use axum::http::StatusCode;
use log::error;
use resalt_models::*;
use resalt_storage::Storage;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DataDump {
    pub users: Vec<User>,
    pub groups: Vec<PermissionGroup>,
    pub memberships: std::collections::HashMap<String, Vec<String>>,
    pub minions: Vec<Minion>,
    #[serde(rename = "minionPresets")]
    pub minion_presets: Vec<MinionPreset>,
}

pub fn import_backup(data: &Storage, config: &DataDump) -> Result<(), StatusCode> {
    // Import users
    for user in &config.users {
        // Check if user exists
        let user_exists = match data.get_user_by_username(&user.username) {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(e) => {
                error!("route_settings_import_post get_user_by_username {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        if user_exists {
            match data.update_user(user) {
                Ok(_) => {}
                Err(e) => {
                    error!("route_settings_import_post update_user {:?}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
        } else {
            // Create user
            match data.create_user_hashed(
                Some(user.id.clone()),
                user.username.clone(),
                user.password.clone(),
                user.perms.clone(),
                user.last_login,
                user.email.clone(),
            ) {
                Ok(_) => {}
                Err(e) => {
                    error!("route_settings_import_post create_user_hashed {:?}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
        }
    }

    // Import groups
    for group in &config.groups {
        // Check if group exists
        let group_exists = match data.get_permission_group_by_id(&group.id) {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(e) => {
                error!(
                    "route_settings_import_post get_permission_group_by_id {:?}",
                    e
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        if group_exists {
            match data.update_permission_group(group) {
                Ok(_) => {}
                Err(e) => {
                    error!("route_settings_import_post update_permission_group {:?}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
        } else {
            // Create group
            match data.create_permission_group(
                Some(group.id.clone()),
                &group.name.clone(),
                Some(group.perms.clone()),
            ) {
                Ok(_) => {}
                Err(e) => {
                    error!("route_settings_import_post create_permission_group {:?}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
        }
    }

    // Import memberships
    for (group_id, user_ids) in &config.memberships {
        for user_id in user_ids {
            match data.insert_permission_group_user(user_id, group_id) {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        "route_settings_import_post update_permission_group_memberships {:?}",
                        e
                    );
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
        }
    }

    // Import minions
    for minion in &config.minions {
        match data.update_minion(
            minion.id.clone(),
            minion.last_seen,
            minion.grains.clone(),
            minion.pillars.clone(),
            minion.pkgs.clone(),
            minion.conformity.clone(),
            minion.conformity_success,
            minion.conformity_incorrect,
            minion.conformity_error,
            minion.last_updated_grains,
            minion.last_updated_pillars,
            minion.last_updated_pkgs,
            minion.last_updated_conformity,
        ) {
            Ok(_) => {}
            Err(e) => {
                error!("route_settings_import_post update_minion {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    }

    // Import minion presets
    for preset in &config.minion_presets {
        match data.insert_minion_preset(Some(preset.id.clone()), &preset.name, &preset.filter) {
            Ok(_) => {}
            Err(e) => {
                error!("route_settings_import_post update_minion_preset {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    }

    Ok(())
}

pub fn export_backup(data: &Storage) -> Result<DataDump, StatusCode> {
    // Get all users (Warning: This will include passwords!)
    let users = data.list_users(None).map_err(|e| {
        error!("route_settings_export_get list_users {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    // Get all permission groups
    let groups = data.list_permission_groups(None).map_err(|e| {
        error!("route_settings_export_get list_permission_groups {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    // Get all permission group memberships
    let mut memberships: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for group in &groups {
        let users = data
            .list_users_by_permission_group_id(&group.id)
            .map_err(|e| {
                error!(
                    "route_settings_export_get list_users_by_permission_group_id {:?}",
                    e
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        let users: Vec<String> = users.iter().map(|u| u.id.clone()).collect();
        memberships.insert(group.id.clone(), users);
    }
    // Get all minions
    let minions = data
        .list_minions(Vec::new(), None, Paginate::None)
        .map_err(|e| {
            error!("route_settings_export_get list_minions {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    // Get all minion presets
    let minion_presets = data.list_minion_presets().map_err(|e| {
        error!("route_settings_export_get list_minion_presets {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(DataDump {
        users,
        groups,
        memberships,
        minions,
        minion_presets,
    })
}
