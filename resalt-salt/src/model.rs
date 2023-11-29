use serde_json::{json, Value};

#[derive(Clone, Debug, Default)]
pub struct SaltEvent {
    pub tag: String,
    pub data: String,
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
