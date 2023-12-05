use log::error;
use resalt_models::*;
use resalt_salt::{SaltAPI, SaltError};
use resalt_storage::StorageImpl;

pub fn get_minions(
    data: &Box<dyn StorageImpl>,
    filters: Vec<Filter>,
    sort: Option<String>,
    paginate: Paginate,
) -> Result<Vec<Minion>, ApiError> {
    data.list_minions(filters, sort, paginate).map_err(|e| {
        error!("api.get_minions {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn get_minion(
    data: &Box<dyn StorageImpl>,
    minion_id: &str,
) -> Result<Option<Minion>, ApiError> {
    data.get_minion_by_id(minion_id).map_err(|e| {
        error!("api.get_minion {:?}", e);
        ApiError::DatabaseError
    })
}

pub async fn refresh_minion(
    salt: &SaltAPI,
    salt_token: &SaltToken,
    minion_id: &str,
) -> Result<(), SaltError> {
    salt.refresh_minion(&salt_token, minion_id)
        .await
        .map_err(|e| {
            error!("api.refresh_minion {:?}", e);
            e
        })
}
