use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
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
    if !has_resalt_permission(&data, &auth.user_id, P_ADMIN_SUPERADMIN)? {
        return Err(ApiError::Forbidden);
    }

    // TODO: implement logic

    Ok(web::Json(input))
}
