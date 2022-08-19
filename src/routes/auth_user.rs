use crate::prelude::*;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;

pub async fn route_auth_user_get(
    data: web::Data<Storage>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let db = data;
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();

    let user = match db.get_user_by_id(&auth.user_id) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(api_error_unauthorized()),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    Ok(web::Json(user.public()))
}
