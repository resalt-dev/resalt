use actix_web::{dev::ServiceRequest, dev::ServiceResponse, HttpResponse, Result};

pub async fn route_fallback_404(
    service_request: ServiceRequest,
) -> Result<ServiceResponse, actix_web::Error> {
    let (req, _payload) = service_request.into_parts();

    let res = HttpResponse::NotFound().body("Not found");

    Ok(ServiceResponse::new(req, res))
}
