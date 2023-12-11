use crate::permission::*;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use resalt_api::setting::{export_backup, import_backup, DataDump};
use resalt_models::AuthStatus;
use resalt_storage::Storage;

pub async fn route_settings_import_post(
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<DataDump>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_SUPERADMIN)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // API
    import_backup(&data, &input).map(|_| ())
}

pub async fn route_settings_export_get(
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_ADMIN_SUPERADMIN)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // API
    export_backup(&data).map(Json)
}
