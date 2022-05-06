use crate::prelude::*;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MinionsGetQuery {
    refresh: Option<bool>,
}

#[derive(Serialize, Debug)]
struct MinionsResponse {
    minions: Vec<Minion>,
    refresh: bool,
}

pub async fn route_minions_get(
    data: web::Data<Storage>,
    salt: web::Data<SaltAPI>,
    query: web::Query<MinionsGetQuery>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ext = req.extensions_mut();
    let auth = ext.get::<AuthStatus>().unwrap();
    let refresh = query.refresh.unwrap_or(false);

    if refresh {
        let salt_token = match &auth.salt_token {
            Some(salt_token) => salt_token,
            None => {
                error!("No salt token found");
                return Err(api_error_unauthorized());
            }
        };
        match salt.refresh_minions(&salt_token).await {
            Ok(_) => (),
            Err(e) => {
                error!("{:?}", e);
                return Err(api_error_internal_error());
            }
        };
    }

    let minions = match data.list_minions().await {
        Ok(minions) => minions,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    let response = MinionsResponse { minions, refresh };
    Ok(web::Json(response))
}
