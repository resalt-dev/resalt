use crate::prelude::*;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    HttpResponse,
};
use log::error;

pub async fn route_fallback_get(
    service_request: ServiceRequest,
) -> Result<ServiceResponse, actix_web::Error> {
    if SConfig::reverse_proxy() {
        return route_fallback_proxy_get(service_request).await;
    } else {
        return route_fallback_static_get(service_request).await;
    }
}

pub async fn route_fallback_proxy_get(
    service_request: ServiceRequest,
) -> Result<ServiceResponse, actix_web::Error> {
    let (req, _payload) = service_request.into_parts();

    let sub_path = SConfig::sub_path();

    // Send request to PROXY_TARGET using awc::Client
    let target = req.uri().path().replace(&sub_path, "");
    let target = format!("{}{}", SConfig::reverse_proxy_target(), target);
    let mut res = match awc::Client::new().get(&target).send().await {
        Ok(res) => res,
        Err(err) => {
            error!("Error sending request to {}: {}", target, err);
            return Err(api_error_internal_error());
        }
    };

    // Return response from proxy target
    // convert from ClientResponse<Decoder<Payload>> to actix_web::dev::ServiceResponse
    let mut response = HttpResponse::build(res.status());
    // Keep headers
    for (key, value) in res.headers().iter() {
        response.insert_header((key, value));
    }
    let response = response.body(match res.body().limit(1000000000).await {
        Ok(body) => body,
        Err(err) => {
            error!("Error reading response body: {}", err);
            return Err(api_error_internal_error());
        }
    });

    return Ok(ServiceResponse::new(req.clone(), response));
}

pub async fn route_fallback_static_get(
    _service_request: ServiceRequest,
) -> Result<ServiceResponse, actix_web::Error> {
    return Err(api_error_internal_error());
}
