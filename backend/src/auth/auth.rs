use crate::prelude::*;
use log::*;

pub async fn validate_auth_token(
    data: &Storage,
    token: &str,
) -> Result<Option<AuthStatus>, actix_web::Error> {
    if token.len() < 20 {
        return Ok(None);
    }

    let authtoken = match data.get_authtoken_by_id(&token).await {
        Ok(authtoken) => match authtoken {
            Some(authtoken) => authtoken,
            None => return Ok(None),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };

    if !authtoken.success {
        return Ok(None);
    }

    let session_lifespan = SConfig::user_session_lifespan();

    if (authtoken.timestamp.timestamp() as u64) + session_lifespan
        < chrono::Utc::now().timestamp() as u64
    {
        return Ok(None);
    }

    return Ok(Some(AuthStatus {
        user_id: authtoken.user_id,
        salt_token: match authtoken.salt_token {
            Some(v) => match serde_json::from_str::<SaltToken>(&v) {
                Ok(v) => Some(v),
                Err(e) => {
                    error!("Failed parsing authtoken.salt_token {:?}", e);
                    None
                }
            },
            None => None,
        },
    }));
}
