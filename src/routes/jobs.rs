use crate::{auth::*, components::*};
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::*;
use resalt_salt::*;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct JobsListGetQuery {
    sort: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_jobs_get(
    data: web::Data<Box<dyn StorageImpl>>,
    query: web::Query<JobsListGetQuery>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_JOB_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let sort = query.sort.clone();
    let limit = query.limit;
    let offset = query.offset;

    let jobs = match data.list_jobs(sort, limit, offset) {
        Ok(jobs) => jobs,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
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
) -> Result<impl Responder, ApiError> {
    let mut auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_RUN_LIVE)? {
        return Err(ApiError::Forbidden);
    }

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(ApiError::Unauthorized);
        }
    };

    async fn run(
        salt: web::Data<SaltAPI>,
        input: &web::Json<JobRunRequest>,
        salt_token: &SaltToken,
    ) -> Result<Value, SaltError> {
        match input.client {
            SaltClientType::Local => {
                salt.run_job_local(
                    salt_token,
                    input.tgt.clone(),
                    input.fun.clone(),
                    Some(input.arg.clone().into_iter().map(SV::S).collect()),
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
                    Some(input.arg.clone().into_iter().map(SV::S).collect()),
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
                    Some(input.arg.clone().into_iter().map(SV::S).collect()),
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
                    Some(input.arg.clone().into_iter().map(SV::S).collect()),
                    Some(input.kwarg.clone()),
                )
                .await
            }
            SaltClientType::RunnerAsync => {
                salt.run_job_runner_async(
                    salt_token,
                    input.fun.clone(),
                    Some(input.arg.clone().into_iter().map(SV::S).collect()),
                    Some(input.kwarg.clone()),
                )
                .await
            }
            SaltClientType::Wheel => {
                salt.run_job_wheel(
                    salt_token,
                    input.fun.clone(),
                    Some(input.arg.clone().into_iter().map(SV::S).collect()),
                    Some(input.kwarg.clone()),
                )
                .await
            }
            SaltClientType::WheelAsync => {
                salt.run_job_wheel_async(
                    salt_token,
                    input.fun.clone(),
                    Some(input.arg.clone().into_iter().map(SV::S).collect()),
                    Some(input.kwarg.clone()),
                )
                .await
            }
        }
    }

    match run(salt.clone(), &input, salt_token).await {
        Ok(job) => Ok(web::Json(job)),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(ApiError::InternalError);
            }
            error!("Salt token expired, renewing and retrying");
            auth = renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token).await?;
            match run(salt, &input, &auth.salt_token.unwrap()).await {
                Ok(job) => Ok(web::Json(job)),
                Err(e) => {
                    error!("{:?}", e);
                    Err(ApiError::InternalError)
                }
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::InternalError)
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
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

    // Validate permission
    if !has_resalt_permission(&data, &auth.user_id, P_JOB_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let job = match data.get_job_by_jid(&info.jid) {
        Ok(job) => job,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    let job = match job {
        Some(job) => job,
        None => {
            return Err(ApiError::NotFound);
        }
    };

    let returns = match data.get_job_returns_by_job(&job) {
        Ok(returns) => returns,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };

    Ok(web::Json(JobGetResponse { job, returns }))
}
