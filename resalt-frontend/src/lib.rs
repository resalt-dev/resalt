use include_dir::{include_dir, Dir};

static FRONTEND_PUBLIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/output");

pub fn frontend_get(path: String) -> (String, Vec<u8>) {
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

    let body = file.contents().to_vec();
    let file_path = file.path().to_str().unwrap();
    let mime_type = match_mime(file_path).to_owned();

    (mime_type, body)
}

pub fn match_mime(filename: &str) -> &str {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontend_get() {
        let (mime_type, body) = frontend_get("index.html".to_owned());
        assert_eq!(mime_type, "text/html; charset=utf-8");
        assert_eq!(body, include_bytes!("../output/index.html"));

        let (mime_type, body) = frontend_get("/".to_owned());
        assert_eq!(mime_type, "text/html; charset=utf-8");
        assert_eq!(body, include_bytes!("../output/index.html"));

        let (mime_type, body) = frontend_get("".to_owned());
        assert_eq!(mime_type, "text/html; charset=utf-8");
        assert_eq!(body, include_bytes!("../output/index.html"));

        let (mime_type, body) = frontend_get("favicon.png".to_owned());
        assert_eq!(mime_type, "image/png");
        assert_eq!(body, include_bytes!("../output/favicon.png"));
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
