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
        // Check if user exists, if so, delete
        match data.get_user_by_id(&user.id) {
            Ok(_) => {
                info!("route_settings_import_post user exists, deleting");
                match data.delete_user(&user.id) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("route_settings_import_post delete_user {:?}", e);
                        return Err(ApiError::DatabaseError);
                    }
                };
            }
            Err(e) => {
                error!("route_settings_import_post get_user {:?}", e);
                return Err(ApiError::DatabaseError);
            }
        };
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

    // // Import groups
    // for group in input.groups {
    //     data.create_group(
    //         group.name,
    //         group.perms,
    //         group.users,
    //         group.minions,
    //         group.ldap_sync,
    //     )?;
    // }

    Ok(web::Json(input))
}
