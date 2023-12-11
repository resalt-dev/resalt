use http::StatusCode;
use log::*;
use resalt_models::*;
use resalt_salt::{SaltAPI, SaltError};
use resalt_storage::Storage;
use serde_json::Value;

pub fn get_jobs(
    data: Storage,
    paginate: Paginate,
    sort: Option<JobSort>,
) -> Result<Vec<Job>, StatusCode> {
    data.list_jobs(sort, paginate).map_err(|e| {
        error!("api.get_jobs {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub async fn create_job(
    salt: &SaltAPI,
    salt_token: &SaltToken,
    run_job: &SaltRunJob,
) -> Result<Value, SaltError> {
    salt.run_job(salt_token, run_job).await
}

pub fn get_job(data: Storage, jid: &str) -> Result<Option<Job>, StatusCode> {
    data.get_job_by_jid(jid).map_err(|e| {
        error!("api.get_job {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn get_job_returns_by_job(data: Storage, job: &Job) -> Result<Vec<JobReturn>, StatusCode> {
    data.get_job_returns_by_job(job).map_err(|e| {
        error!("api.get_job_returns_by_job {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
