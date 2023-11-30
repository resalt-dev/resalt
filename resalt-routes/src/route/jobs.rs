use crate::PaginateQuery;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_api::jobs::get_jobs;
use resalt_auth::renew_token_salt_token;
use resalt_models::*;
use resalt_salt::*;
use resalt_security::*;
use resalt_storage::StorageImpl;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct JobsListGetQuery {
    sort: Option<String>,
    // Include fields from PaginateQuery
    #[serde(flatten)]
    paginate_query: PaginateQuery,
}

pub async fn route_jobs_get(
    query: Query<JobsListGetQuery>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_JOB_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let sort = query.sort.clone();
    // Pagination
    let paginate: Paginate = query.paginate_query.parse_query();

    // API
    Ok(Json(get_jobs(data, paginate, sort)?))
}

#[derive(Deserialize)]
pub struct JobRunRequest {
    client: SaltClientType,
    #[serde(rename = "tgtType")]
    tgt_type: SaltTgtType,
    tgt: String,
    fun: String,
    arg: Vec<String>,
    kwarg: HashMap<String, String>,
    #[serde(rename = "batchSize")]
    batch_size: String,
}

pub async fn route_jobs_post(
    State(salt): State<SaltAPI>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<JobRunRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_RUN_LIVE)? {
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
        salt: SaltAPI,
        input: &JobRunRequest,
        salt_token: &SaltToken,
    ) -> Result<Value, SaltError> {
        match input.client {
            SaltClientType::Local => {
                salt.run_job_local(
                    salt_token,
                    input.tgt.clone(),
                    input.fun.clone(),
                    Some(input.arg.clone().into_iter().map(SV::S).collect()),
                    Some(input.tgt_type),
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
                    Some(input.tgt_type),
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
                    Some(input.tgt_type),
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
        Ok(job) => Ok(Json(job)),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(ApiError::InternalError);
            }
            error!("Salt token expired, renewing and retrying");
            let auth =
                renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token).await?;
            match run(salt, &input, &auth.salt_token.unwrap()).await {
                Ok(job) => Ok(Json(job)),
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

#[derive(Serialize)]
pub struct JobGetResponse {
    job: Job,
    returns: Vec<JobReturn>,
}

pub async fn route_job_get(
    Path(jid): Path<String>,
    State(data): State<Box<dyn StorageImpl>>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate permission
    if !has_resalt_permission(&auth, P_JOB_LIST)? {
        return Err(ApiError::Forbidden);
    }

    let job = match data.get_job_by_jid(&jid) {
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

    Ok(Json(JobGetResponse { job, returns }))
}
