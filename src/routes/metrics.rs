use crate::components::*;
use actix_web::{web, Responder, Result};
use log::*;
use resalt_storage::StorageImpl;

pub async fn route_metrics_get(data: web::Data<Box<dyn StorageImpl>>) -> Result<impl Responder> {
    let results = match data.get_metric_results() {
        Ok(results) => results,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_internal_error());
        }
    };

    Ok(web::Json(results))
}
