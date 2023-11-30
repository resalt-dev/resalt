use axum::{
    extract::ConnectInfo,
    http::{header, Request},
    middleware::Next,
    response::Response,
    Extension,
};
use log::*;
use resalt_models::{AuthStatus, ResaltTime};
use std::net::SocketAddr;

pub async fn middleware_logging<B>(
    ConnectInfo(socket): ConnectInfo<SocketAddr>,
    auth: Option<Extension<AuthStatus>>,
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    req: Request<B>,
    next: Next<B>,
) -> Response {
    //
    // PRE-REQUEST PROCESSING
    //

    // CLF data collection
    let ip = socket.ip();
    let date = ResaltTime::now().format("%d/%b/%Y:%H:%M:%S %z");
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let proto = req.version();
    let user = match auth {
        Some(auth) => auth.user_id.clone(),
        None => "-".to_string(),
    };

    //
    // MAIN
    //
    let start_time = ResaltTime::now();
    let response = next.run(req).await;
    let end_time = ResaltTime::now();

    //
    // POST-REQUEST PROCESSING
    //

    // Log the request (CLF)
    let status = response.status();
    let body_size = response
        .headers()
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let _time = end_time - start_time;

    info!(
        "{} - {} [{}] \"{} {} {:?}\" {} {}",
        ip, user, date, method, path, proto, status, body_size
    );

    response
}
