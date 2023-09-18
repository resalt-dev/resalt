use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use resalt_models::{ApiError, AuthStatus, User};
use resalt_security::{has_resalt_permission, P_ADMIN_SUPERADMIN};
use resalt_storage::StorageImpl;
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct SettingsExport {
    users: Vec<User>,
}

pub async fn route_settings_export_get(
    data: web::Data<Box<dyn StorageImpl>>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_ADMIN_SUPERADMIN)? {
        return Err(ApiError::Forbidden);
    }

    let users = Vec::<User>::new();
    let config = SettingsExport { users };
    Ok(web::Json(config))
}
