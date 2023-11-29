use include_dir::{include_dir, Dir};

static FRONTEND_PUBLIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/build");

pub fn frontend_get(path: String) -> (&'static str, &'static [u8]) {
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

    (file_path, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontend_get() {
        let (file_path, body) = frontend_get("index.html".to_owned());
        assert_eq!(file_path, "index.html");
        assert_eq!(body, include_bytes!("../build/index.html"));

        let (file_path, body) = frontend_get("/".to_owned());
        assert_eq!(file_path, "index.html");
        assert_eq!(body, include_bytes!("../build/index.html"));

        let (file_path, body) = frontend_get("".to_owned());
        assert_eq!(file_path, "index.html");
        assert_eq!(body, include_bytes!("../build/index.html"));

        let (file_path, body) = frontend_get("favicon.png".to_owned());
        assert_eq!(file_path, "favicon.png");
        assert_eq!(body, include_bytes!("../build/favicon.png"));
    }
}
