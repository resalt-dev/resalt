use axum::extract::FromRef;
use resalt_salt::{SaltAPI, SaltEventListenerStatus};
use resalt_storage::Storage;

#[derive(Clone)]
pub struct AppState {
    pub data: Storage,
    pub salt_api: SaltAPI,
    pub listener_status: SaltEventListenerStatus,
}

impl FromRef<AppState> for Storage {
    fn from_ref(app_state: &AppState) -> Storage {
        Clone::clone(&app_state.data)
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
