use serde::{Deserialize, Serialize};

use crate::{strip_quotes, ResaltTime};

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
    pub state: SaltKeyState,
    pub finger: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SaltKeyState {
    #[default]
    #[serde(rename = "minions")]
    Accepted,
    #[serde(rename = "minions_pre")]
    Pending,
    #[serde(rename = "minions_rejected")]
    Rejected,
    #[serde(rename = "minions_denied")]
    Denied,
}

impl ToString for SaltKeyState {
    fn to_string(&self) -> String {
        strip_quotes(serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum SaltClientType {
    #[default]
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "local_async")]
    LocalAsync,
    #[serde(rename = "local_batch")]
    LocalBatch,
    #[serde(rename = "runner")]
    Runner,
    #[serde(rename = "runner_async")]
    RunnerAsync,
    #[serde(rename = "wheel")]
    Wheel,
    #[serde(rename = "wheel_async")]
    WheelAsync,
}

impl ToString for SaltClientType {
    fn to_string(&self) -> String {
        strip_quotes(serde_json::to_string(self).unwrap())
    }
}
