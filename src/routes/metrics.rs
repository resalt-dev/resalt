use crate::components::*;
use actix_web::{web, Responder, Result};
use log::*;
use resalt_storage::StorageImpl;

pub async fn route_metrics_get(
    data: web::Data<Box<dyn StorageImpl>>,
) -> Result<impl Responder, ApiError> {
    let results = match data.get_metric_results() {
        Ok(results) => results,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::InternalError);
        }
    };

    Ok(web::Json(results))
}
