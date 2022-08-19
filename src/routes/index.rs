use actix_web::{web, Responder};

pub(crate) async fn route_index_get() -> impl Responder {
    web::Json("Hello API")
}
