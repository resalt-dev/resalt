use crate::prelude::*;
use actix_web::{web, Responder, Result};
use log::*;

pub async fn route_metrics_get(data: web::Data<Storage>) -> Result<impl Responder> {
    let results = match data.get_metric_results() {
        Ok(results) => results,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_internal_error());
        }
    };

    log::warn!("Metrics: {:?}", results);

    Ok(web::Json(results))
}
