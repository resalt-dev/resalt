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
