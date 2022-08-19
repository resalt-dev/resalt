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
    if SConfig::http_frontend_proxy_enabled() {
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
    let target = format!("{}{}", SConfig::http_frontend_proxy_target(), target);
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
        // skip prefix and the next /
        .skip(SConfig::sub_path().len() + 1)
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

    let body = file.contents();
    let file_path = file.path().to_str().unwrap();
    let mime_type = match_mime(file_path);

    // log::debug!("Serving file {} with mime type {}", file_path, mime_type);

    // Add content type header
    let mut response = HttpResponse::build(actix_web::http::StatusCode::OK);
    response.insert_header((actix_web::http::header::CONTENT_TYPE, mime_type));
    let response = response.body(body);
    Ok(ServiceResponse::new(req.clone(), response))
}

fn match_mime(filename: &str) -> String {
    let ext = filename.split('.').last().unwrap_or("").to_lowercase();
    match ext.as_str() {
        // code
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "map" => "application/json; charset=utf-8", // js.map
        // images
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        // fonts
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "eot" => "application/vnd.ms-fontobject",
        // other
        "txt" => "text/plain; charset=utf-8",
        "csv" => "text/csv; charset=utf-8",
        "pdf" => "application/pdf",
        "json" => "application/json; charset=utf-8",
        "zip" => "application/zip",
        _ => "application/octet-stream",
    }
    .to_string()
}
