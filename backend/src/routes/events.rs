use crate::broadcast::Broadcaster;
use actix_web::{http::header, web::Data, HttpResponse, Responder, Result};

pub async fn route_events_get(broadcaster: Data<Broadcaster>) -> Result<impl Responder> {
    let rx = broadcaster.new_client();

    Ok(HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "text/event-stream"))
        .streaming(rx))
}
