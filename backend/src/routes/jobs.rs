use crate::prelude::*;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
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
    req: HttpRequest,
) -> Result<impl Responder> {
    let user = query.user.clone();
    let start_date = query
        .start_date
        .clone()
        .map(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap());
    let end_date = query
        .end_date
        .clone()
        .map(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap());
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
