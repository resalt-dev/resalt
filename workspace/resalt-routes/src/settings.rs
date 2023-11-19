use actix_web::{
    get, post,
    web::{self},
    HttpMessage, HttpRequest, Responder, Result,
};
use log::error;
use resalt_models::{ApiError, AuthStatus, Minion, MinionPreset, PermissionGroup, User};
use resalt_security::{has_resalt_permission, P_ADMIN_SUPERADMIN};
use resalt_storage::StorageImpl;
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

#[post("/settings/import")]
pub async fn route_settings_import_post(
    data: web::Data<Box<dyn StorageImpl>>,
    input: web::Json<DataDump>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&auth.perms, P_ADMIN_SUPERADMIN)? {
        return Err(ApiError::Forbidden);
    }

    // Import users
    for user in &input.users {
        // Check if user exists
        let user_exists = match data.get_user_by_username(&user.username) {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(e) => {
                error!("route_settings_import_post get_user_by_username {:?}", e);
                return Err(ApiError::DatabaseError);
            }
        };
        if user_exists {
            match data.update_user(user) {
                Ok(_) => {}
                Err(e) => {
                    error!("route_settings_import_post update_user {:?}", e);
                    return Err(ApiError::DatabaseError);
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
                user.ldap_sync.clone(),
            ) {
                Ok(_) => {}
                Err(e) => {
                    error!("route_settings_import_post create_user_hashed {:?}", e);
                    return Err(ApiError::DatabaseError);
                }
            };
        }
    }

    // Import groups
    for group in &input.groups {
        // Check if group exists
        let group_exists = match data.get_permission_group_by_id(&group.id) {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(e) => {
                error!(
                    "route_settings_import_post get_permission_group_by_id {:?}",
                    e
                );
                return Err(ApiError::DatabaseError);
            }
        };
        if group_exists {
            match data.update_permission_group(group) {
                Ok(_) => {}
                Err(e) => {
                    error!("route_settings_import_post update_permission_group {:?}", e);
                    return Err(ApiError::DatabaseError);
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
                    return Err(ApiError::DatabaseError);
                }
            };
        }
    }

    // Import memberships
    for (group_id, user_ids) in &input.memberships {
        for user_id in user_ids {
            match data.insert_permission_group_user(user_id, group_id) {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        "route_settings_import_post update_permission_group_memberships {:?}",
                        e
                    );
                    return Err(ApiError::DatabaseError);
                }
            };
        }
    }

    // Import minions
    for minion in &input.minions {
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
                return Err(ApiError::DatabaseError);
            }
        };
    }

    // Import minion presets
    for preset in &input.minion_presets {
        match data.insert_minion_preset(Some(preset.id.clone()), &preset.name, &preset.filter) {
            Ok(_) => {}
            Err(e) => {
                error!("route_settings_import_post update_minion_preset {:?}", e);
                return Err(ApiError::DatabaseError);
            }
        };
    }

    Ok(web::Json(()))
}

#[get("/settings/export")]
pub async fn route_settings_export_get(
    data: web::Data<Box<dyn StorageImpl>>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&auth.perms, P_ADMIN_SUPERADMIN)? {
        return Err(ApiError::Forbidden);
    }

    // Get all users (Warning: This will include passwords!)
    let users = match data.list_users(Some(i64::MAX), Some(0)) {
        Ok(users) => users,
        Err(_) => return Err(ApiError::DatabaseError),
    };
    // Get all permission groups
    let groups = match data.list_permission_groups(Some(i64::MAX), Some(0)) {
        Ok(groups) => groups,
        Err(_) => return Err(ApiError::DatabaseError),
    };
    // Get all permission group memberships
    let mut users_by_group_id: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for group in &groups {
        let users = match data.list_users_by_permission_group_id(&group.id) {
            Ok(users) => users,
            Err(_) => return Err(ApiError::DatabaseError),
        };
        let users: Vec<String> = users.iter().map(|u| u.id.clone()).collect();
        users_by_group_id.insert(group.id.clone(), users);
    }

    // Get all minions
    let minions = match data.list_minions(vec![], None, Some(i64::MAX), Some(0)) {
        Ok(minions) => minions,
        Err(_) => return Err(ApiError::DatabaseError),
    };

    // Get all minion presets
    let minion_presets = match data.list_minion_presets() {
        Ok(minion_presets) => minion_presets,
        Err(_) => return Err(ApiError::DatabaseError),
    };

    let config = DataDump {
        users,
        groups,
        memberships: users_by_group_id,
        minions,
        minion_presets,
    };
    Ok(web::Json(config))
}
