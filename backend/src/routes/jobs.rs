use crate::prelude::*;
use actix_web::{web, Responder, Result};
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
    let limit = query.limit.clone();
    let offset = query.offset.clone();

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
pub struct JobGetInfo {
    jid: String,
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

    if job.is_none() {
        return Err(api_error_not_found());
    }

    Ok(web::Json(job))
}
