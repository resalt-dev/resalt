use std::collections::HashMap;

use crate::prelude::*;
use actix_web::{web, HttpResponse, Responder, Result};
use log::*;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct TokenValidateRequest {
    username: String,
    password: String,
}

pub async fn route_auth_token_post(
    data: web::Data<Storage>,
    input: web::Form<TokenValidateRequest>,
) -> Result<impl Responder> {
    let db = data;
    let username = input.username.to_lowercase();
    let token = input.password.clone();

    info!("Token validation for {:?} with token {:?}", username, token);

    if username == HIBIKE_SALT_SYSTEM_SERVICE_USERNAME {
        if token == SConfig::salt_api_system_service_token() {
            info!("System service token OK");
            return Ok(HttpResponse::Ok().json(HashMap::from([(
                username,
                HashMap::from([
                    (".*".to_string(), Value::Null),
                    ("@runner".to_string(), Value::Null),
                    ("@wheel".to_string(), Value::Null),
                ]),
            )])));
        } else {
            return Err(api_error_unauthorized());
        }
    }

    match validate_auth_token(&db, &token).await {
        Ok(Some(auth_status)) => {
            info!("Token validated for {:?}", auth_status.user_id);
            // send {username: None} as JSON response
            Ok(HttpResponse::Ok().json(HashMap::from([(username, serde_json::Value::Null)])))
        }
        Ok(None) => {
            info!("Invalid token from Salt validation");
            Err(api_error_unauthorized())
        }
        Err(e) => {
            error!("{:?}", e);
            Err(api_error_database())
        }
    }
}
