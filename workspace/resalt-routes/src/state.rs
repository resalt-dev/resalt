use axum::extract::FromRef;
use resalt_salt::{SaltAPI, SaltEventListenerStatus};
use resalt_storage::StorageImpl;

#[derive(Clone)]
pub struct AppState {
    pub data: Box<dyn StorageImpl>,
    pub salt_api: SaltAPI,
    pub listener_status: SaltEventListenerStatus,
}

impl FromRef<AppState> for Box<dyn StorageImpl> {
    fn from_ref(app_state: &AppState) -> Box<dyn StorageImpl> {
        app_state.data.clone()
    }
}

impl FromRef<AppState> for SaltAPI {
    fn from_ref(app_state: &AppState) -> SaltAPI {
        app_state.salt_api.clone()
    }
}

impl FromRef<AppState> for SaltEventListenerStatus {
    fn from_ref(app_state: &AppState) -> SaltEventListenerStatus {
        app_state.listener_status.clone()
    }
}
