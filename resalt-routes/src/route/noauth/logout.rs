use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn route_logout_post() -> Result<impl IntoResponse, StatusCode> {
    // TODO: Check if user is logged in
    // TODO: Invalidate the token in Storage

    // Unset cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        "Set-Cookie",
        "resalt-auth=; Path=/; HttpOnly; Expires=Thu, 01 Jan 1970 00:00:00 GMT"
            .parse()
            .unwrap(),
    );

    // Return
    Ok((headers, Json(())))
}
