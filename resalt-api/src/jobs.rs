use log::*;
use resalt_models::*;
use resalt_salt::{SaltAPI, SaltError};
use resalt_storage::StorageImpl;
use serde_json::Value;

pub fn get_jobs(
    data: Box<dyn StorageImpl>,
    paginate: Paginate,
    sort: Option<String>,
) -> Result<Vec<Job>, ApiError> {
    let jobs = match data.list_jobs(sort, paginate) {
        Ok(jobs) => jobs,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(jobs)
}

pub async fn create_job(
    salt: &SaltAPI,
    salt_token: &SaltToken,
    run_job: &SaltRunJob,
) -> Result<Value, SaltError> {
    salt.run_job(salt_token, run_job).await
}
