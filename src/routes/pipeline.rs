use actix_web::{
    http::header, web::Data, HttpMessage, HttpRequest, HttpResponse, Responder, Result,
};
use resalt_models::AuthStatus;
use resalt_pipeline::PipelineServer;

pub async fn route_pipeline_get(
    pipeline: Data<PipelineServer>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();
    let rx = pipeline.new_client(auth.user_id);

    Ok(HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "text/event-stream"))
        .streaming(rx))
}
