use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use resalt_models::{ApiError, AuthStatus, Minion, PermissionGroup, User};
use resalt_security::{has_resalt_permission, P_ADMIN_SUPERADMIN};
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsExport {
    pub users: Vec<User>,
    pub groups: Vec<PermissionGroup>,
    pub memberships: std::collections::HashMap<String, Vec<String>>,
    pub minions: Vec<Minion>,
}

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

    let config = SettingsExport {
        users,
        groups,
        memberships: users_by_group_id,
        minions,
    };
    Ok(web::Json(config))
}
