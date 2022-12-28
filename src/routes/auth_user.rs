use crate::components::*;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use resalt_models::AuthStatus;
use resalt_storage::StorageImpl;

pub async fn route_auth_user_get(
    data: web::Data<Box<dyn StorageImpl>>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let db = data;
    let auth = req.extensions_mut().get::<AuthStatus>().unwrap().clone();

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

    let permission_groups = match db.list_permission_groups_by_user_id(&user.id) {
        Ok(permission_groups) => permission_groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };
    Ok(web::Json(user.public(permission_groups)))
}
