use actix_tls::connect::openssl::reexports::{SslConnector, SslMethod};
use actix_web::http::StatusCode;
use async_stream::stream;
use awc::{
    error::{JsonPayloadError, PayloadError, SendRequestError},
    ClientResponse, Connector,
};
use futures::StreamExt;
use log::*;
use openssl::ssl::SslVerifyMode;
use resalt_config::SConfig;
use resalt_models::*;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{collections::HashMap, time::Duration};

const X_AUTH_TOKEN: &str = "X-Auth-Token";

#[derive(Clone, Debug, Default)]
pub struct SaltEvent {
    pub tag: String,
    pub data: String,
}

#[derive(Debug)]
pub enum SaltError {
    Unauthorized, // 401
    Forbidden,    // 403
    RequestError(SendRequestError),
    ResponseParseError(Option<JsonPayloadError>),
    MissingExpectedDataError(String),
    #[allow(clippy::type_complexity)]
    FailedRequest(
        ClientResponse<
            actix_web::dev::Decompress<
                actix_web::dev::Payload<
                    std::pin::Pin<
                        Box<
                            dyn futures_core::Stream<
                                Item = Result<actix_web::web::Bytes, PayloadError>,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    ),
}

#[derive(Clone, Default, Deserialize)]
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
        match self {
            SaltTgtType::Glob => "glob".to_string(),
            SaltTgtType::PCRE => "pcre".to_string(),
            SaltTgtType::List => "list".to_string(),
            SaltTgtType::Grain => "grain".to_string(),
            SaltTgtType::GrainPCRE => "grain_pcre".to_string(),
            SaltTgtType::Pillar => "pillar".to_string(),
            SaltTgtType::PillarPCRE => "pillar_pcre".to_string(),
            SaltTgtType::NodeGroup => "nodegroup".to_string(),
            SaltTgtType::Range => "range".to_string(),
            SaltTgtType::Compound => "compound".to_string(),
            SaltTgtType::IPCIDR => "ipcidr".to_string(),
        }
    }
}

#[derive(Default, Deserialize)]
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
        match self {
            SaltClientType::Local => "local".to_string(),
            SaltClientType::LocalAsync => "local_async".to_string(),
            SaltClientType::LocalBatch => "local_batch".to_string(),
            SaltClientType::Runner => "runner".to_string(),
            SaltClientType::RunnerAsync => "runner_async".to_string(),
            SaltClientType::Wheel => "wheel".to_string(),
            SaltClientType::WheelAsync => "wheel_async".to_string(),
        }
    }
}

#[derive(Default, Deserialize)]
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
        match self {
            SaltKeyState::Accepted => "minions".to_string(),
            SaltKeyState::Pending => "minions_pre".to_string(),
            SaltKeyState::Rejected => "minions_rejected".to_string(),
            SaltKeyState::Denied => "minions_denied".to_string(),
        }
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

pub type Dictionary = HashMap<String, String>;

lazy_static::lazy_static! {
    static ref AWC_CONFIG: SslConnector = {
        let mut config = SslConnector::builder(SslMethod::tls_client()).unwrap();

        if SConfig::salt_api_tls_skipverify() {
            config.set_verify(SslVerifyMode::NONE);
        }

        config.build()
    };
}

pub fn create_awc_client() -> awc::Client {
    awc::Client::builder()
        .connector(
            Connector::new()
                .openssl(AWC_CONFIG.to_owned())
                .timeout(Duration::from_secs(3)), // Connector timeout, 3 seconds
        )
        .timeout(Duration::from_secs(60 * 20)) // Request timeout, 20 minutes
        .finish()
}

#[derive(Clone)]
pub struct SaltAPI {
    client: awc::Client,
}

impl Default for SaltAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl SaltAPI {
    pub fn new() -> Self {
        Self {
            //client: Client::default(),
            client: create_awc_client(),
        }
    }

    pub async fn login(&self, username: &str, authtoken: &str) -> Result<SaltToken, SaltError> {
        let url = format!("{}/login", &SConfig::salt_api_url());
        // Send POST request to Salt API for auth token
        // This will contact us back on the /token endpoint to validate auth token
        let mut res = match self
            .client
            .post(url)
            .send_json(&serde_json::json!({
                "eauth": "rest",
                "username": username,
                "password": authtoken,
            }))
            .await
        {
            Ok(res) => res,
            Err(e) => {
                error!("login_err {:?}", e);
                return Err(SaltError::RequestError(e));
            }
        };

        // If access denied (e.g. missing permissions)
        if res.status() == StatusCode::FORBIDDEN {
            return Err(SaltError::Forbidden);
        }
        // If status != 200, something went wrong
        if res.status() != StatusCode::OK {
            return Err(SaltError::FailedRequest(res));
        }

        // Parse response from JSON stored as {return: [SaltToken]}
        let body = match res.json::<serde_json::Value>().await {
            Ok(body) => body,
            Err(e) => {
                error!("{:?}", e);
                return Err(SaltError::ResponseParseError(Some(e)));
            }
        };

        let salt_token = match body.get("return") {
            Some(salt_token) => salt_token,
            None => {
                error!("No token returned from Salt API");
                return Err(SaltError::ResponseParseError(None));
            }
        };
        let salt_token = match salt_token.get(0) {
            Some(salt_token) => salt_token,
            None => {
                error!("No token returned from Salt API");
                return Err(SaltError::ResponseParseError(None));
            }
        };
        // Convert to SaltToken object
        let mut salt_token: SaltToken = match serde_json::from_value(salt_token.clone()) {
            Ok(salt_token) => salt_token,
            Err(e) => {
                error!("{:?}", e);
                return Err(SaltError::ResponseParseError(None));
            }
        };

        // If the array is completely empty, then Salt annoyingly returns
        // an empty Object instead of an empty array. In order to keep our
        // data more clean, convert this to an empty array instead.
        if salt_token.perms.is_object() {
            salt_token.perms = json!(Vec::<String>::new());
        }

        debug!("login {:?}", salt_token);

        Ok(salt_token)
    }

    pub fn listen_events(
        &self,
        salt_token: &SaltToken,
    ) -> impl futures_core::stream::Stream<Item = SaltEvent> {
        let url = format!(
            "{}/events?salt_token={}",
            SConfig::salt_api_url(),
            salt_token.token.clone()
        );

        let client = self.client.clone();
        stream! {
            debug!("Connecting to SSE stream: {}", &url);
            let mut stream = match client
                .get(url)
                .insert_header(("Accept", "text/event-stream"))
                .send()
                .await {
                Ok(stream) => stream,
                Err(e) => {
                    error!("Failed to connect to SSE stream: {}", e);
                    return;
                }
            };

            // Parse ServerSideEvents
            //
            // Important part is "tag" and "data".
            // Data can be split over multiple lines.
            // A message is ended with double-new line.
            //
            // Example:
            //
            // retry: 400\n
            // tag: salt/job/test/test_job_1\n
            // data: some text\n
            // \n
            // tag: salt/job/test/test_job_2\n
            // data: another message\n
            // data: with even more content spanning two lines\n
            // \n

            // Assume "stream" does not return whole lines, but only parts of lines.
            // So a buffer will need to be kept until a \n is reached

            enum SSEParsingState {
                Action,
                Retry,
                Tag,
                Data,
            }

            let mut event = SaltEvent::default();
            let mut mode = SSEParsingState::Action;
            let mut actionbuffer = String::new();

            while let Some(line) = stream.next().await {
                let line = match line {
                    Ok(line) => line,
                    Err(e) => {
                        error!("{:?}", e);
                        break;
                    }
                };
                trace!("LINE {:?}", line);

                // Loop through every byte in line
                for byte in line {
                    let mode_text = match mode {
                        SSEParsingState::Action => "Action",
                        SSEParsingState::Retry => "Retry",
                        SSEParsingState::Tag => "Tag",
                        SSEParsingState::Data => "Data",
                    };
                    // byte to char
                    let c = byte as char;
                    trace!("BYTE {:?} {:?}", c, mode_text);
                    match &mode {
                        SSEParsingState::Action => {
                            // Check if byte is newline
                            if c == '\n' {
                                // End of message!
                                // If we've found both tag and data, emit event
                                if !event.tag.is_empty() && !event.data.is_empty() {
                                    yield event.clone();
                                }
                                // Always reset
                                event = SaltEvent::default();
                                continue;
                            }
                            // Append byte to action buffer
                            actionbuffer.push(c);
                            // Check if action buffer is complete
                            match actionbuffer.as_str() {
                                "retry: " => {
                                    actionbuffer.clear();
                                    mode = SSEParsingState::Retry;
                                }
                                "tag: " => {
                                    actionbuffer.clear();
                                    mode = SSEParsingState::Tag;
                                }
                                "data: " => {
                                    actionbuffer.clear();
                                    mode = SSEParsingState::Data;
                                }
                                _ => {}
                            }
                        }
                        SSEParsingState::Retry => {
                            // Check if byte is newline
                            if c == '\n' {
                                mode = SSEParsingState::Action;
                                continue;
                            }
                            // We don't care about retry, so don't store it. Only listen until the command is done.
                        }
                        SSEParsingState::Tag => {
                            // Check if byte is newline
                            if c == '\n' {
                                mode = SSEParsingState::Action;
                                continue;
                            }
                            // Append byte to tag buffer
                            event.tag.push(c);
                        }
                        SSEParsingState::Data => {
                            // Check if byte is newline
                            if c == '\n' {
                                mode = SSEParsingState::Action;
                                continue;
                            }
                            // Append byte to data buffer
                            event.data.push(c);
                        }
                    }
                }
            }
            debug!("SSE stream closed by ending loop");
        }
    }

    async fn run_job(
        &self,
        salt_token: &SaltToken,
        data: serde_json::Value,
    ) -> Result<Value, SaltError> {
        let url = &SConfig::salt_api_url();

        // debug!("run_job data {:?}", data);

        let mut res = match self
            .client
            .post(url)
            .append_header((X_AUTH_TOKEN, salt_token.token.clone()))
            .send_json(&data)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                return Err(SaltError::RequestError(e));
            }
        };

        // If access denied (e.g. missing permissions)
        if res.status() == StatusCode::FORBIDDEN {
            return Err(SaltError::Forbidden);
        }
        // If unauthorized (e.g. invalid token)
        if res.status() == StatusCode::UNAUTHORIZED {
            return Err(SaltError::Unauthorized);
        }
        // If status != 200, something went wrong
        if res.status() != StatusCode::OK {
            return Err(SaltError::FailedRequest(res));
        }

        let body = match res.json::<serde_json::Value>().await {
            Ok(body) => body,
            Err(e) => {
                return Err(SaltError::ResponseParseError(Some(e)));
            }
        };
        debug!("run_job run body {:?}", body);

        let body = match body.get("return") {
            Some(body) => body,
            None => {
                return Err(SaltError::MissingExpectedDataError(
                    "run_job: missing return".to_string(),
                ));
            }
        };
        let body = match body.get(0) {
            Some(body) => body,
            None => {
                return Err(SaltError::MissingExpectedDataError(
                    "run_job: missing return[0]".to_owned(),
                ));
            }
        };

        Ok(body.clone())
    }

    pub async fn run_job_local<S: AsRef<str>>(
        &self,
        salt_token: &SaltToken,
        tgt: S,
        fun: S,
        arg: Option<Vec<SV>>,
        tgt_type: Option<SaltTgtType>,
        kwarg: Option<Dictionary>,
    ) -> Result<Value, SaltError> {
        let data = json!({
            "client": SaltClientType::Local.to_string(),
            "tgt": tgt.as_ref(),
            "fun": fun.as_ref(),
            // map arg to empty array if None
            "arg": arg.map(|v| {
                let mut args = Vec::new();
                for s in v.iter() {
                    args.push(s.as_value());
                }
                args
            }).unwrap_or_default(),
            "tgt_type": (tgt_type.unwrap_or_default()).to_string(),
            "kwarg": kwarg.unwrap_or_default(),
        });
        self.run_job(salt_token, data).await
    }

    pub async fn run_job_local_async<S: AsRef<str>>(
        &self,
        salt_token: &SaltToken,
        tgt: S,
        fun: S,
        arg: Option<Vec<SV>>,
        tgt_type: Option<SaltTgtType>,
        kwarg: Option<Dictionary>,
    ) -> Result<Value, SaltError> {
        let data = json!({
            "client": SaltClientType::LocalAsync.to_string(),
            "tgt": tgt.as_ref(),
            "fun": fun.as_ref(),
            "arg": arg.map(|v| {
                let mut args = Vec::new();
                for s in v.iter() {
                    args.push(s.as_value());
                }
                args
            }).unwrap_or_default(),
            "tgt_type": (tgt_type.unwrap_or_default()).to_string(),
            "kwarg": kwarg.unwrap_or_default(),
        });
        self.run_job(salt_token, data).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn run_job_local_batch<S: AsRef<str>>(
        &self,
        salt_token: &SaltToken,
        tgt: S,
        fun: S,
        arg: Option<Vec<SV>>,
        tgt_type: Option<SaltTgtType>,
        kwarg: Option<Dictionary>,
        batch: S,
    ) -> Result<Value, SaltError> {
        let data = json!({
            "client": SaltClientType::LocalBatch.to_string(),
            "tgt": tgt.as_ref(),
            "fun": fun.as_ref(),
            "arg": arg.map(|v| {
                let mut args = Vec::new();
                for s in v.iter() {
                    args.push(s.as_value());
                }
                args
            }).unwrap_or_default(),
            "tgt_type": (tgt_type.unwrap_or_default()).to_string(),
            "kwarg": kwarg.unwrap_or_default(),
            "batch": batch.as_ref(),
        });
        self.run_job(salt_token, data).await
    }

    pub async fn run_job_runner<S: AsRef<str>>(
        &self,
        salt_token: &SaltToken,
        fun: S,
        arg: Option<Vec<SV>>,
        kwarg: Option<Dictionary>,
    ) -> Result<Value, SaltError> {
        let data = json!({
            "client": SaltClientType::Runner.to_string(),
            "fun": fun.as_ref(),
            "arg": arg.map(|v| {
                let mut args = Vec::new();
                for s in v.iter() {
                    args.push(s.as_value());
                }
                args
            }).unwrap_or_default(),
            "kwarg": kwarg.unwrap_or_default(),
        });
        self.run_job(salt_token, data).await
    }

    pub async fn run_job_runner_async<S: AsRef<str>>(
        &self,
        salt_token: &SaltToken,
        fun: S,
        arg: Option<Vec<SV>>,
        kwarg: Option<Dictionary>,
    ) -> Result<Value, SaltError> {
        let data = json!({
            "client": SaltClientType::RunnerAsync.to_string(),
            "fun": fun.as_ref(),
            "arg": arg.map(|v| {
                let mut args = Vec::new();
                for s in v.iter() {
                    args.push(s.as_value());
                }
                args
            }).unwrap_or_default(),
            "kwarg": kwarg.unwrap_or_default(),
        });
        self.run_job(salt_token, data).await
    }

    pub async fn run_job_wheel<S: AsRef<str>>(
        &self,
        salt_token: &SaltToken,
        fun: S,
        arg: Option<Vec<SV>>,
        kwarg: Option<Dictionary>,
    ) -> Result<Value, SaltError> {
        let data = json!({
            "client": SaltClientType::Wheel.to_string(),
            "fun": fun.as_ref(),
            "arg": arg.map(|v| {
                let mut args = Vec::new();
                for s in v.iter() {
                    args.push(s.as_value());
                }
                args
            }).unwrap_or_default(),
            "kwarg": kwarg.unwrap_or_default(),
        });
        let data = match self.run_job(salt_token, data).await {
            Ok(res) => res,
            Err(e) => {
                return Err(e);
            }
        };
        let data = match data.get("data") {
            Some(res) => res,
            None => {
                return Err(SaltError::MissingExpectedDataError(
                    "run_job_wheel: missing ['data']".to_owned(),
                ));
            }
        };
        Ok(data.clone())
    }

    pub async fn run_job_wheel_async<S: AsRef<str>>(
        &self,
        salt_token: &SaltToken,
        fun: S,
        arg: Option<Vec<SV>>,
        kwarg: Option<Dictionary>,
    ) -> Result<Value, SaltError> {
        let data = json!({
            "client": SaltClientType::WheelAsync.to_string(),
            "fun": fun.as_ref(),
            "arg": arg.map(|v| {
                let mut args = Vec::new();
                for s in v.iter() {
                    args.push(s.as_value());
                }
                args
            }).unwrap_or_default(),
            "kwarg": kwarg.unwrap_or_default(),
        });
        self.run_job(salt_token, data).await
    }

    /// Returns host:status:finger pair
    pub async fn get_keys(&self, salt_token: &SaltToken) -> Result<Vec<SaltMinionKey>, SaltError> {
        let data = match self
            .run_job_wheel(
                salt_token,
                "key.finger",
                Some(vec![SV::S("*".to_owned())]),
                None,
            )
            .await
        {
            Ok(data) => data,
            Err(e) => return Err(e),
        };
        let data = match data.get("return") {
            Some(data) => data,
            None => {
                return Err(SaltError::MissingExpectedDataError(
                    "get_keys: missing return".to_owned(),
                ));
            }
        };
        // There are 4 string arrays: minions_rejected, minions_denied, minions_pre, minions
        // Get them, and map to (host, status) tuples.
        let minions_rejected = match data.get("minions_rejected") {
            Some(data) => match data.as_object() {
                Some(data) => Some(data),
                None => {
                    return Err(SaltError::MissingExpectedDataError(
                        "get_keys: return['minions_rejected'] is not object".to_owned(),
                    ));
                }
            },
            None => None,
        };
        let minions_denied = match data.get("minions_denied") {
            Some(data) => match data.as_object() {
                Some(data) => Some(data),
                None => {
                    return Err(SaltError::MissingExpectedDataError(
                        "get_keys: return['minions_denied'] is not object".to_owned(),
                    ));
                }
            },
            None => None,
        };
        let minions_pre = match data.get("minions_pre") {
            Some(data) => match data.as_object() {
                Some(data) => Some(data),
                None => {
                    return Err(SaltError::MissingExpectedDataError(
                        "get_keys: return['minions_pre'] is not object".to_owned(),
                    ));
                }
            },
            None => None,
        };
        let minions = match data.get("minions") {
            Some(data) => match data.as_object() {
                Some(data) => Some(data),
                None => {
                    return Err(SaltError::MissingExpectedDataError(
                        "get_keys: return['minions'] is not object".to_owned(),
                    ));
                }
            },
            None => None,
        };
        let mut keys = Vec::new();
        if let Some(minions_rejected) = minions_rejected {
            for (host, finger) in minions_rejected.iter() {
                keys.push(SaltMinionKey {
                    id: host.clone(),
                    state: SaltKeyState::Rejected.to_string(),
                    finger: finger.as_str().unwrap().to_owned(),
                });
            }
        }
        if let Some(minions_denied) = minions_denied {
            for (host, finger) in minions_denied.iter() {
                keys.push(SaltMinionKey {
                    id: host.clone(),
                    state: SaltKeyState::Denied.to_string(),
                    finger: finger.as_str().unwrap().to_owned(),
                });
            }
        }
        if let Some(minions_pre) = minions_pre {
            for (host, finger) in minions_pre.iter() {
                keys.push(SaltMinionKey {
                    id: host.clone(),
                    state: SaltKeyState::Pending.to_string(),
                    finger: finger.as_str().unwrap().to_owned(),
                });
            }
        }
        if let Some(minions) = minions {
            for (host, finger) in minions.iter() {
                keys.push(SaltMinionKey {
                    id: host.clone(),
                    state: SaltKeyState::Accepted.to_string(),
                    finger: finger.as_str().unwrap().to_owned(),
                });
            }
        }
        Ok(keys)
    }

    pub async fn accept_key(
        &self,
        salt_token: &SaltToken,
        state: &SaltKeyState,
        id: &str,
    ) -> Result<(), SaltError> {
        let mut kwargs = HashMap::new();
        kwargs.insert("include_rejected".to_owned(), "True".to_owned());
        kwargs.insert("include_denied".to_owned(), "True".to_owned());
        let data = match self
            .run_job_wheel(
                salt_token,
                "key.accept_dict",
                Some(vec![SV::V(json!({
                    state.to_string(): vec![id.to_owned()],
                }))]),
                Some(kwargs),
            )
            .await
        {
            Ok(data) => data,
            Err(e) => return Err(e),
        };
        let data = match data.get("return") {
            Some(data) => data,
            None => {
                return Err(SaltError::MissingExpectedDataError(
                    "accept_key: missing return".to_owned(),
                ));
            }
        };
        let list = match data.get(SaltKeyState::Accepted.to_string()) {
            Some(data) => match data.as_array() {
                Some(data) => data,
                None => {
                    return Err(SaltError::MissingExpectedDataError(
                        "accept_key: return[SaltKeyState::Accepted] is not array".to_owned(),
                    ));
                }
            },
            None => {
                return Err(SaltError::MissingExpectedDataError(
                    "accept_key: missing return[SaltKeyState::Accepted]".to_owned(),
                ));
            }
        };
        if list.is_empty() {
            return Err(SaltError::MissingExpectedDataError(
                "accept_key: return[SaltKeyState::Accepted] is empty".to_owned(),
            ));
        }
        Ok(())
    }

    pub async fn reject_key(
        &self,
        salt_token: &SaltToken,
        state: &SaltKeyState,
        id: &str,
    ) -> Result<(), SaltError> {
        let mut kwargs = HashMap::new();
        kwargs.insert("include_accepted".to_owned(), "True".to_owned());
        kwargs.insert("include_denied".to_owned(), "True".to_owned());
        let data = match self
            .run_job_wheel(
                salt_token,
                "key.reject_dict",
                Some(vec![SV::V(json!({
                    state.to_string(): vec![id.to_owned()],
                }))]),
                Some(kwargs),
            )
            .await
        {
            Ok(data) => data,
            Err(e) => return Err(e),
        };
        let data = match data.get("return") {
            Some(data) => data,
            None => {
                return Err(SaltError::MissingExpectedDataError(
                    "reject_key: missing return".to_owned(),
                ));
            }
        };
        let list = match data.get(SaltKeyState::Rejected.to_string()) {
            Some(data) => match data.as_array() {
                Some(data) => data,
                None => {
                    return Err(SaltError::MissingExpectedDataError(
                        "reject_key: return[state] is not array".to_owned(),
                    ));
                }
            },
            None => {
                return Err(SaltError::MissingExpectedDataError(
                    "reject_key: missing return[state]".to_owned(),
                ));
            }
        };
        if list.is_empty() {
            return Err(SaltError::MissingExpectedDataError(
                "reject_key: return[state] is empty".to_owned(),
            ));
        }
        Ok(())
    }

    pub async fn delete_key(
        &self,
        salt_token: &SaltToken,
        state: &SaltKeyState,
        id: &str,
    ) -> Result<(), SaltError> {
        let data = match self
            .run_job_wheel(
                salt_token,
                "key.delete_dict",
                Some(vec![SV::V(json!({
                    state.to_string(): vec![id.to_owned()],
                }))]),
                None,
            )
            .await
        {
            Ok(data) => data,
            Err(e) => return Err(e),
        };
        let data = match data.get("success") {
            Some(data) => match data.as_bool() {
                Some(data) => data,
                None => {
                    return Err(SaltError::MissingExpectedDataError(
                        "delete_key: success is not boolean".to_owned(),
                    ));
                }
            },
            None => {
                return Err(SaltError::MissingExpectedDataError(
                    "delete_key: missing return".to_owned(),
                ));
            }
        };
        if !data {
            return Err(SaltError::MissingExpectedDataError(
                "delete_key: success is false".to_owned(),
            ));
        }
        Ok(())
    }

    pub async fn refresh_minion(&self, salt_token: &SaltToken, id: &str) -> Result<(), SaltError> {
        match self
            .run_job_local_async(salt_token, id, "grains.items", None, None, None)
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        match self
            .run_job_local_async(salt_token, id, "pillar.items", None, None, None)
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        match self
            .run_job_local_async(salt_token, id, "pkg.list_pkgs", None, None, None)
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        // Expect compliance to take the longest, so sync on that and show the UI as done whenever it returns
        let mut map_test_true = HashMap::new();
        map_test_true.insert("test".to_owned(), "True".to_owned());
        match self
            .run_job_local(
                salt_token,
                id,
                "state.highstate",
                None,
                None,
                Some(map_test_true),
            )
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
