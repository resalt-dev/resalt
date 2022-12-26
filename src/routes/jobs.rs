use crate::{
    auth::{has_permission, P_RUN_LIVE},
    components::*,
};
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::*;
use resalt_salt::*;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JobsListGetQuery {
    sort: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_jobs_get(
    data: web::Data<Box<dyn StorageImpl>>,
    query: web::Query<JobsListGetQuery>,
) -> Result<impl Responder> {
    let sort = query.sort.clone();
    let limit = query.limit;
    let offset = query.offset;

    let jobs = match data.list_jobs(sort, limit, offset) {
        Ok(jobs) => jobs,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    Ok(web::Json(jobs))
}

#[derive(Deserialize)]
pub struct JobRunRequest {
    client: SaltClientType,
    #[serde(rename = "tgtType")]
    tgt_type: SaltTgtType,
    tgt: String,
    fun: String,
    arg: Vec<String>,
    kwarg: Dictionary,
    #[serde(rename = "batchSize")]
    batch_size: String,
}

pub async fn route_jobs_post(
    salt: web::Data<SaltAPI>,
    data: web::Data<Box<dyn StorageImpl>>,
    input: web::Json<JobRunRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    // Validate permission
    if !has_permission(&data, &auth.user_id, P_RUN_LIVE)? {
        return Err(api_error_forbidden());
    }

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(api_error_unauthorized());
        }
    };

    let res = match input.client {
        SaltClientType::Local => {
            salt.run_job_local(
                salt_token,
                input.tgt.clone(),
                input.fun.clone(),
                Some(input.arg.clone().into_iter().map(|a| SV::S(a)).collect()),
                Some(input.tgt_type.clone()),
                Some(input.kwarg.clone()),
            )
            .await
        }
        SaltClientType::LocalAsync => {
            salt.run_job_local_async(
                salt_token,
                input.tgt.clone(),
                input.fun.clone(),
                Some(input.arg.clone().into_iter().map(|a| SV::S(a)).collect()),
                Some(input.tgt_type.clone()),
                Some(input.kwarg.clone()),
            )
            .await
        }
        SaltClientType::LocalBatch => {
            salt.run_job_local_batch(
                salt_token,
                input.tgt.clone(),
                input.fun.clone(),
                Some(input.arg.clone().into_iter().map(|a| SV::S(a)).collect()),
                Some(input.tgt_type.clone()),
                Some(input.kwarg.clone()),
                input.batch_size.clone(),
            )
            .await
        }
        SaltClientType::Runner => {
            salt.run_job_runner(
                salt_token,
                input.fun.clone(),
                Some(input.arg.clone().into_iter().map(|a| SV::S(a)).collect()),
                Some(input.kwarg.clone()),
            )
            .await
        }
        SaltClientType::RunnerAsync => {
            salt.run_job_runner_async(
                salt_token,
                input.fun.clone(),
                Some(input.arg.clone().into_iter().map(|a| SV::S(a)).collect()),
                Some(input.kwarg.clone()),
            )
            .await
        }
        SaltClientType::Wheel => {
            salt.run_job_wheel(
                salt_token,
                input.fun.clone(),
                Some(input.arg.clone().into_iter().map(|a| SV::S(a)).collect()),
                Some(input.kwarg.clone()),
            )
            .await
        }
        SaltClientType::WheelAsync => {
            salt.run_job_wheel_async(
                salt_token,
                input.fun.clone(),
                Some(input.arg.clone().into_iter().map(|a| SV::S(a)).collect()),
                Some(input.kwarg.clone()),
            )
            .await
        }
    };

    match res {
        Ok(job) => Ok(web::Json(job)),
        Err(e) => {
            error!("{:?}", e);
            Err(api_error_internal_error())
        }
    }
}

#[derive(Deserialize)]
pub struct JobGetInfo {
    jid: String,
}

#[derive(Serialize)]
pub struct JobGetResponse {
    job: Job,
    returns: Vec<Event>,
}

pub async fn route_job_get(
    data: web::Data<Box<dyn StorageImpl>>,
    info: web::Path<JobGetInfo>,
) -> Result<impl Responder> {
    let job = match data.get_job_by_jid(&info.jid) {
        Ok(job) => job,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let job = match job {
        Some(job) => job,
        None => {
            return Err(api_error_not_found());
        }
    };

    let returns = match data.get_job_returns_by_job(&job) {
        Ok(returns) => returns,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    Ok(web::Json(JobGetResponse { job, returns }))
}
