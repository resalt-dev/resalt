use axum::{http::Request, middleware::Next, response::Response};
use tower_http::normalize_path::NormalizePath;

#[allow(clippy::let_and_return)]
pub async fn middleware_normalize_path<B>(
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    //
    // PRE-REQUEST PROCESSING
    //

    // normalize_trailing_slash
    NormalizePath::trim_trailing_slash(&mut req);

    //
    // MAIN
    //
    let response = next.run(req).await;

    //
    // POST-REQUEST PROCESSING
    //

    response
}
