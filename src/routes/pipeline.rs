use crate::prelude::*;
use actix_web::{
    http::header, web::Data, HttpMessage, HttpRequest, HttpResponse, Responder, Result,
};

pub async fn route_pipeline_get(
    pipeline: Data<PipelineServer>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();
    let rx = pipeline.new_client(auth.user_id.clone());

    Ok(HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "text/event-stream"))
        .streaming(rx))
}
