use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::{ApiError, AuthStatus};
use resalt_security::{has_resalt_permission, P_ADMIN_SUPERADMIN};
use resalt_storage::StorageImpl;

use super::SettingsExport;

pub async fn route_settings_import_post(
    data: web::Data<Box<dyn StorageImpl>>,
    input: web::Json<SettingsExport>,
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
                user.last_login.clone(),
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
        let group_exists = match data.get_permission_group_by_name(&group.name) {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(e) => {
                error!(
                    "route_settings_import_post get_permission_group_by_name {:?}",
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
            minion.last_seen.clone().into(),
            minion.grains.clone(),
            minion.pillars.clone(),
            minion.pkgs.clone(),
            minion.conformity.clone(),
            minion.conformity_success,
            minion.conformity_incorrect,
            minion.conformity_error,
            minion.last_updated_grains.clone(),
            minion.last_updated_pillars.clone(),
            minion.last_updated_pkgs.clone(),
            minion.last_updated_conformity.clone(),
        ) {
            Ok(_) => {}
            Err(e) => {
                error!("route_settings_import_post update_minion {:?}", e);
                return Err(ApiError::DatabaseError);
            }
        };
    }

    Ok(web::Json(()))
}
