use actix_web::{get, web, Responder};

#[get("/")]
pub async fn route_index_get() -> impl Responder {
    web::Json("Hello API")
}
