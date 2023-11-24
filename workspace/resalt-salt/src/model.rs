use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Clone, Debug, Default)]
pub struct SaltEvent {
    pub tag: String,
    pub data: String,
}

#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub enum SaltTgtType {
    #[default]
    #[serde(rename = "glob")]
    Glob,
    #[serde(rename = "pcre")]
    PCRE,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "grain")]
    Grain,
    #[serde(rename = "grain_pcre")]
    GrainPCRE,
    #[serde(rename = "pillar")]
    Pillar,
    #[serde(rename = "pillar_pcre")]
    PillarPCRE,
    #[serde(rename = "nodegroup")]
    NodeGroup,
    #[serde(rename = "range")]
    Range,
    #[serde(rename = "compound")]
    Compound,
    #[serde(rename = "ipcidr")]
    IPCIDR,
}

impl ToString for SaltTgtType {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Debug)]
pub enum SaltError {
    Unauthorized,  // 401
    Forbidden,     // 403
    FailedRequest, // Anything NOT 200
    RequestError(String),
    ResponseParseError(Option<String>),
    MissingExpectedDataError(String),
}

#[derive(Debug, Default, Deserialize)]
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
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Debug, Default, Deserialize)]
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
        format!("{:?}", self).to_lowercase()
    }
}

pub enum SV {
    S(String),
    V(Value),
}

impl SV {
    pub fn as_value(&self) -> Value {
        match self {
            SV::S(s) => json!(s),
            SV::V(v) => v.clone(),
        }
    }
}
