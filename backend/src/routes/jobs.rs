use crate::prelude::*;
use actix_web::{web, Responder, Result};
use chrono::NaiveDateTime;
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JobsGetQuery {
    user: Option<String>,
    start_date: Option<String>, // NaiveDateTime
    end_date: Option<String>,   // NaiveDateTime
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Serialize, Debug)]
struct JobsResponse {
    jobs: Vec<Job>,
}

pub async fn route_jobs_get(
    data: web::Data<Storage>,
    query: web::Query<JobsGetQuery>,
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

    let response = JobsResponse { jobs };
    Ok(web::Json(response))
}
