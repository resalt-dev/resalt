use log::error;
use resalt_models::{ApiError, User};
use resalt_storage::StorageImpl;

pub fn get_user_by_id(
    data: &Box<dyn StorageImpl>,
    user_id: &str,
) -> Result<Option<User>, ApiError> {
    data.get_user_by_id(user_id).map_err(|e| {
        error!("api.get_user_by_id {:?}", e);
        ApiError::DatabaseError
    })
}
