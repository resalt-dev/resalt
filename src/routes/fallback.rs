use crate::prelude::*;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, http::header, HttpResponse, Result};

pub async fn route_fallback_redirect(
    service_request: ServiceRequest,
) -> Result<ServiceResponse, actix_web::Error> {
    let (req, _payload) = service_request.into_parts();

    let res = HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, SConfig::sub_path()))
        .finish();

    Ok(ServiceResponse::new(req, res))
}

pub async fn route_fallback_404(
    service_request: ServiceRequest,
) -> Result<ServiceResponse, actix_web::Error> {
    let (req, _payload) = service_request.into_parts();

    let res = HttpResponse::NotFound().body("Not found");

    Ok(ServiceResponse::new(req, res))
}
