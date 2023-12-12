use axum::{
    extract::OriginalUri,
    http::{header, StatusCode},
    response::IntoResponse,
};
use resalt_frontend::frontend_get;

pub async fn route_frontend_get(
    OriginalUri(uri): OriginalUri,
) -> Result<impl IntoResponse, StatusCode> {
    let path = uri.path().to_owned();

    let (mime_type, body) = frontend_get(path);

    // log::debug!("Serving file {} with mime type {}", file_path, mime_type);

    Ok((
        [
            // Add content type header
            (header::CONTENT_TYPE, mime_type),
            // Disable caching
            (header::CACHE_CONTROL, "no-cache".to_owned()),
        ],
        body,
    )
        .into_response())
}

#[cfg(test)]
mod tests {
    use axum::http::Uri;

    use super::*;

    #[tokio::test]
    async fn test_route_frontend_get() {
        // Test route_frontend_get
        let uri = OriginalUri("/".parse::<Uri>().unwrap());
        let (parts, _body) = route_frontend_get(uri)
            .await
            .unwrap()
            .into_response()
            .into_parts();

        // Check Version
        assert_eq!(parts.version, axum::http::Version::HTTP_11);

        // Check Status
        assert_eq!(parts.status, axum::http::StatusCode::OK);

        // Check Headers
        assert_eq!(
            parts.headers.get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            parts.headers.get(header::CACHE_CONTROL).unwrap(),
            "no-cache"
        );
    }
}
