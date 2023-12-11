use http::StatusCode;
use log::error;
use resalt_models::*;
use resalt_salt::{SaltAPI, SaltError};
use resalt_storage::Storage;

pub fn get_minions(
    data: &Storage,
    filters: Vec<Filter>,
    sort: Option<MinionSort>,
    paginate: Paginate,
) -> Result<Vec<Minion>, StatusCode> {
    data.list_minions(filters, Some(sort.unwrap_or_default()), paginate)
        .map_err(|e| {
            error!("api.get_minions {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub fn get_minion(data: &Storage, minion_id: &str) -> Result<Option<Minion>, StatusCode> {
    data.get_minion_by_id(minion_id).map_err(|e| {
        error!("api.get_minion {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub async fn refresh_minion(
    salt: &SaltAPI,
    salt_token: &SaltToken,
    minion_id: &str,
) -> Result<(), SaltError> {
    salt.refresh_minion(salt_token, minion_id)
        .await
        .map_err(|e| {
            error!("api.refresh_minion {:?}", e);
            e
        })
}
