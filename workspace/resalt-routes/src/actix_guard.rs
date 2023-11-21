use actix_web::guard;
use resalt_models::AuthStatus;

pub fn guard_require_auth(ctx: &guard::GuardContext<'_>) -> bool {
    // Check if req has extension AuthStatus{}
    match ctx.req_data().get::<AuthStatus>() {
        Some(auth_status) => auth_status.salt_token.is_some(),
        None => false,
    }
}
