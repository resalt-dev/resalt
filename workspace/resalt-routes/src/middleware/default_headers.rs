use axum::{
    http::{header, Request},
    middleware::Next,
    response::Response,
};

pub async fn middleware_default_headers<B>(
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    req: Request<B>,
    next: Next<B>,
) -> Response {
    //
    // PRE-REQUEST PROCESSING
    //

    //
    // MAIN
    //
    let mut response = next.run(req).await;

    //
    // POST-REQUEST PROCESSING
    //

    let default_headers = [
        (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        (header::X_FRAME_OPTIONS, "DENY"),
        (header::X_XSS_PROTECTION, "1; mode=block"),
    ];

    // Check if header is set, otherwise set it
    for (header, value) in default_headers.iter() {
        if !response.headers().contains_key(header) {
            response
                .headers_mut()
                .insert(header, value.parse().unwrap());
        }
    }

    response
}
