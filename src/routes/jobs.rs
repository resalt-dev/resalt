use crate::prelude::*;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use chrono::NaiveDateTime;
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JobsListGetQuery {
    user: Option<String>,
    start_date: Option<String>, // NaiveDateTime
    end_date: Option<String>,   // NaiveDateTime
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn route_jobs_get(
    data: web::Data<Storage>,
    query: web::Query<JobsListGetQuery>,
) -> Result<impl Responder> {
    // Filtering
    let user = query.user.clone();
    let start_date = match query.start_date.clone() {
        Some(date) => match NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S.%fZ") {
            Ok(d) => Some(d),
            Err(e) => {
                error!("Failed to parse start_date: {}", e);
                return Err(api_error_invalid_request());
            }
        },
        None => None,
    };
    let end_date = match query.end_date.clone() {
        Some(date) => match NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S.%fZ") {
            Ok(d) => Some(d),
            Err(e) => {
                error!("Failed to parse end_date: {}", e);
                return Err(api_error_invalid_request());
            }
        },
        None => None,
    };

    // Pagination
    let limit = query.limit;
    let offset = query.offset;

    let jobs = match data.list_jobs(user, start_date, end_date, limit, offset) {
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
    #[serde(rename = "batch_size")]
    batch_size: String,
    timeout: Option<u64>,
}

pub async fn route_jobs_post(
    salt: web::Data<SaltAPI>,
    input: web::Json<JobRunRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

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
                input.timeout,
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
    data: web::Data<Storage>,
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
