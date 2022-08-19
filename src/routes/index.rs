use actix_web::{web, Responder};

pub async fn route_index_get() -> impl Responder {
    web::Json("Hello API")
}
