use serde::{Deserialize, Serialize};

use crate::ResaltTime;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaltToken {
    pub token: String,
    pub start: f64,
    pub expire: f64,
    pub user: String,
    pub eauth: String,
    pub perms: serde_json::Value,
}

impl SaltToken {
    pub fn expired(&self) -> bool {
        // Example: 1673048623.7256165
        // Check if time has passed minus 5 seconds
        let now = ResaltTime::now().timestamp() as f64;
        now > self.expire - 5.0
    }

    pub fn matured(&self) -> bool {
        // Example: 1673048623.7256165
        // Check if time since issued is greater than 10 minutes
        let now = ResaltTime::now().timestamp() as f64;
        now > self.start + 600.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuthStatus {
    pub user_id: String,
    pub perms: String,
    pub auth_token: String,
    pub salt_token: Option<SaltToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaltMinionKey {
    pub id: String,
    pub state: String,
    pub finger: String,
}
