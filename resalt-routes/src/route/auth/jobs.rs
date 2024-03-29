use crate::{login::renew_token_salt_token, permission::*};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use log::*;
use resalt_api::job::{create_job, get_job, get_job_returns_by_job, get_jobs};
use resalt_models::*;
use resalt_salt::*;
use resalt_storage::Storage;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct JobsListGetQuery {
    sort: Option<JobSort>,
    // Include fields from PaginateQuery
    #[serde(flatten)]
    paginate_query: PaginateQuery,
}

pub async fn route_jobs_get(
    query: Query<JobsListGetQuery>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_JOB_LIST)? {
        return Err(StatusCode::FORBIDDEN);
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
    arg: Vec<Value>,
    kwarg: HashMap<String, String>,
    #[serde(rename = "batchSize")]
    batch_size: String,
}

pub fn map_client_to_runjob(request: JobRunRequest) -> SaltRunJob {
    match request.client {
        SaltClientType::Local => SaltRunJob::Local {
            tgt: request.tgt,
            fun: request.fun,
            arg: Some(request.arg),
            tgt_type: Some(request.tgt_type),
            kwarg: Some(request.kwarg),
        },
        SaltClientType::LocalAsync => SaltRunJob::LocalAsync {
            tgt: request.tgt,
            fun: request.fun,
            arg: Some(request.arg),
            tgt_type: Some(request.tgt_type),
            kwarg: Some(request.kwarg),
        },
        SaltClientType::LocalBatch => SaltRunJob::LocalBatch {
            tgt: request.tgt,
            fun: request.fun,
            arg: Some(request.arg),
            tgt_type: Some(request.tgt_type),
            kwarg: Some(request.kwarg),
            batch_size: request.batch_size,
        },
        SaltClientType::Runner => SaltRunJob::Runner {
            fun: request.fun,
            arg: Some(request.arg),
            kwarg: Some(request.kwarg),
        },
        SaltClientType::RunnerAsync => SaltRunJob::RunnerAsync {
            fun: request.fun,
            arg: Some(request.arg),
            kwarg: Some(request.kwarg),
        },
        SaltClientType::Wheel => SaltRunJob::Wheel {
            fun: request.fun,
            arg: Some(request.arg),
            kwarg: Some(request.kwarg),
        },
        SaltClientType::WheelAsync => SaltRunJob::WheelAsync {
            fun: request.fun,
            arg: Some(request.arg),
            kwarg: Some(request.kwarg),
        },
    }
}

pub async fn route_jobs_post(
    State(salt): State<SaltAPI>,
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
    Json(input): Json<JobRunRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_RUN_LIVE)? {
        return Err(StatusCode::FORBIDDEN);
    }

    let salt_token = match &auth.salt_token {
        Some(salt_token) => salt_token,
        None => {
            error!("No salt token found");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let run_job = map_client_to_runjob(input);

    // API
    match create_job(&salt, salt_token, &run_job).await {
        Ok(job) => Ok(Json(job)),
        Err(SaltError::Unauthorized) => {
            if !salt_token.matured() {
                error!("Salt token unauthorized, but not matured");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            // TODO: Remove this complex logic, this is more of a hack
            error!("Salt token expired, renewing and retrying");
            let auth =
                renew_token_salt_token(&data, &salt, &auth.user_id, &auth.auth_token).await?;
            match create_job(&salt, &auth.salt_token.unwrap(), &run_job).await {
                Ok(job) => Ok(Json(job)),
                Err(e) => {
                    error!("route_jobs_post {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            error!("route_jobs_post {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
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
    State(data): State<Storage>,
    Extension(auth): Extension<AuthStatus>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate permission
    if !has_resalt_permission(&auth, P_JOB_LIST)? {
        return Err(StatusCode::FORBIDDEN);
    }

    // API
    let job = match get_job(Clone::clone(&data), &jid) {
        Ok(job) => job,
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let job: Job = match job {
        Some(job) => job,
        None => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let returns = match get_job_returns_by_job(data, &job) {
        Ok(returns) => returns,
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(JobGetResponse { job, returns }))
}
