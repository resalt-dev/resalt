use crate::prelude::*;
use actix_web::{http::header, web::Data, HttpResponse, Responder, Result};

pub async fn route_pipeline_get(pipeline: Data<PipelineServer>) -> Result<impl Responder> {
    let rx = pipeline.new_client();

    Ok(HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "text/event-stream"))
        .streaming(rx))
}
