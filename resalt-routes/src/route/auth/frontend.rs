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

    let (file_path, body) = frontend_get(path);

    let mime_type = match_mime(file_path);

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

#[cfg(test)]
mod tests {
    use axum::{body::HttpBody, http::Uri};

    use super::*;

    #[tokio::test]
    async fn test_route_frontend_get() {
        // Test route_frontend_get
        let uri = OriginalUri("/".parse::<Uri>().unwrap());
        let (parts, mut body) = route_frontend_get(uri)
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

        // Check Body length > 0
        let body_bytes = body.data().await.unwrap().unwrap();
        assert!(!body_bytes.is_empty());
    }

    #[test]
    fn test_match_mime() {
        assert_eq!(match_mime("test.html"), "text/html; charset=utf-8");
        assert_eq!(match_mime("test.css"), "text/css; charset=utf-8");
        assert_eq!(
            match_mime("test.js"),
            "application/javascript; charset=utf-8"
        );
        assert_eq!(match_mime("test.map"), "application/json; charset=utf-8");
        assert_eq!(match_mime("test.png"), "image/png");
        assert_eq!(match_mime("test.jpg"), "image/jpeg");
        assert_eq!(match_mime("test.jpeg"), "image/jpeg");
        assert_eq!(match_mime("test.gif"), "image/gif");
        assert_eq!(match_mime("test.svg"), "image/svg+xml");
        assert_eq!(match_mime("test.ico"), "image/x-icon");
        assert_eq!(match_mime("test.woff"), "font/woff");
        assert_eq!(match_mime("test.woff2"), "font/woff2");
        assert_eq!(match_mime("test.ttf"), "font/ttf");
        assert_eq!(match_mime("test.otf"), "font/otf");
        assert_eq!(match_mime("test.eot"), "application/vnd.ms-fontobject");
        assert_eq!(match_mime("test.txt"), "text/plain; charset=utf-8");
        assert_eq!(match_mime("test.csv"), "text/csv; charset=utf-8");
        assert_eq!(match_mime("test.pdf"), "application/pdf");
        assert_eq!(match_mime("test.json"), "application/json; charset=utf-8");
        assert_eq!(match_mime("test.zip"), "application/zip");
        assert_eq!(match_mime("test"), "application/octet-stream");
    }
}
