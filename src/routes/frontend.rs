use crate::prelude::*;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    HttpResponse,
};
use include_dir::{include_dir, Dir};
use log::{error, warn};

static FRONTEND_PUBLIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/frontend/public");

pub async fn route_frontend_get(
    service_request: ServiceRequest,
) -> Result<ServiceResponse, actix_web::Error> {
    if SConfig::frontend_proxy_enabled() {
        return route_frontend_proxy_get(service_request).await;
    } else {
        return route_frontend_static_get(service_request).await;
    }
}

pub async fn route_frontend_proxy_get(
    service_request: ServiceRequest,
) -> Result<ServiceResponse, actix_web::Error> {
    let (req, _payload) = service_request.into_parts();

    // Send request to PROXY_TARGET using awc::Client
    let target = req.uri().path();
    let target = target
        .chars()
        .into_iter()
        .skip(SConfig::sub_path().len())
        .collect::<String>();
    let target = format!("{}{}", SConfig::frontend_proxy_target(), target);
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

    Ok(ServiceResponse::new(req.clone(), response))
}

pub async fn route_frontend_static_get(
    service_request: ServiceRequest,
) -> Result<ServiceResponse, actix_web::Error> {
    let (req, _payload) = service_request.into_parts();

    // fetch file from FRONTEND_PUBLIC_DIR based on request URL
    let path = req.uri().path();
    let path = path
        .chars()
        .into_iter()
        .skip(SConfig::sub_path().len())
        .collect::<String>();

    // fetch using FRONTEND_PUBLIC_DIR.get_file
    let mut file = FRONTEND_PUBLIC_DIR.get_file(&path);
    if file.is_none() {
        file = FRONTEND_PUBLIC_DIR.get_file("index.html");
    }
    let file = match file {
        Some(file) => file,
        None => {
            warn!("File not found: {}", path);
            return Err(api_error_not_found());
        }
    };

    let body = match file.contents_utf8() {
        Some(body) => body,
        None => {
            error!("Error reading file {}, no content", path);
            return Err(api_error_internal_error());
        }
    };

    Ok(ServiceResponse::new(
        req.clone(),
        HttpResponse::Ok().body(body),
    ))
}
