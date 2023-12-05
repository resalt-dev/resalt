use axum::{extract::State, response::IntoResponse, Extension, Json};
use resalt_api::setting::{export_backup, import_backup, DataDump};
use resalt_models::{ApiError, AuthStatus};
use resalt_security::{has_resalt_permission, P_ADMIN_SUPERADMIN};
use resalt_storage::StorageImpl;

pub async fn route_settings_import_post(
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<DataDump>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_SUPERADMIN)? {
        return Err(ApiError::Forbidden);
    }

    // API
    import_backup(&data, &input).map(|_| ())
}

pub async fn route_settings_export_get(
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_SUPERADMIN)? {
        return Err(ApiError::Forbidden);
    }

    // API
    export_backup(&data).map(Json)
}
