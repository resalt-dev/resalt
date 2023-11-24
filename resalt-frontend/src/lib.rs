use axum::http::{header, HeaderName};
use include_dir::{include_dir, Dir};

static FRONTEND_PUBLIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/build");

pub fn frontend_get(path: String) -> ([(HeaderName, String); 2], &'static [u8]) {
    // Fetch file from FRONTEND_PUBLIC_DIR based on request URL

    // If path starts with /, trim it
    let path = if let Some(path) = path.strip_prefix('/') {
        path
    } else {
        &path
    };

    // If path is empty, serve index.html
    let path = if path.is_empty() { "index.html" } else { path };

    // Fetch using FRONTEND_PUBLIC_DIR.get_file
    let file = match FRONTEND_PUBLIC_DIR.get_file(path) {
        Some(file) => file,
        None => FRONTEND_PUBLIC_DIR.get_file("index.html").unwrap(),
    };

    let body = file.contents();
    let file_path = file.path().to_str().unwrap();
    let mime_type = match_mime(file_path);

    // log::debug!("Serving file {} with mime type {}", file_path, mime_type);

    // Add content type header
    (
        [
            (header::CONTENT_TYPE, mime_type),
            (header::CACHE_CONTROL, "no-cache".to_owned()),
        ],
        body,
    )
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
