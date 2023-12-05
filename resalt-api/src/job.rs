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
    data.list_jobs(sort, paginate).map_err(|e| {
        error!("api.get_jobs {:?}", e);
        ApiError::DatabaseError
    })
}

pub async fn create_job(
    salt: &SaltAPI,
    salt_token: &SaltToken,
    run_job: &SaltRunJob,
) -> Result<Value, SaltError> {
    salt.run_job(salt_token, run_job).await
}

pub fn get_job(data: Box<dyn StorageImpl>, jid: &str) -> Result<Option<Job>, ApiError> {
    data.get_job_by_jid(jid).map_err(|e| {
        error!("api.get_job {:?}", e);
        ApiError::DatabaseError
    })
}

pub fn get_job_returns_by_job(
    data: Box<dyn StorageImpl>,
    job: &Job,
) -> Result<Vec<JobReturn>, ApiError> {
    data.get_job_returns_by_job(job).map_err(|e| {
        error!("api.get_job_returns_by_job {:?}", e);
        ApiError::DatabaseError
    })
}
