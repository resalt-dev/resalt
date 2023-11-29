use super::model::*;
use async_stream::stream;
use futures::StreamExt;
use futures_core::stream;
use log::*;
use reqwest::StatusCode;
use resalt_config::SConfig;
use resalt_models::*;
use serde_json::{json, Value};
use std::{collections::HashMap, time::Duration};

const X_AUTH_TOKEN: &str = "X-Auth-Token";

pub fn create_reqwest_client() -> reqwest::Client {
    let mut builder = reqwest::ClientBuilder::new();
    if SConfig::salt_api_tls_skipverify() {
        builder = builder.danger_accept_invalid_certs(true);
    }
    builder = builder.connect_timeout(Duration::from_secs(5));
    // builder = builder.timeout(Duration::from_secs(5)); // No timeout, as it breaks SSE
    builder.build().unwrap()
}

#[derive(Clone)]
pub struct SaltAPI {
    client: reqwest::Client,
}

impl Default for SaltAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl SaltAPI {
    pub fn new() -> Self {
        Self {
            client: create_reqwest_client(),
        }
    }
    pub async fn login(&self, username: &str, authtoken: &str) -> Result<SaltToken, SaltError> {
        let url = format!("{}/login", &SConfig::salt_api_url());
        // Send POST request to Salt API for auth token
        // This will contact us back on the /token endpoint to validate auth token
        let res = match self
            .client
            .post(&url)
            .json(&json!({
                "eauth": "rest",
                "username": username,
                "password": authtoken,
            }))
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                error!("login_err {:?}", e);
                return Err(SaltError::RequestError(e.to_string()));
            }
        };

        // If access denied (e.g. missing permissions)
        if res.status() == StatusCode::FORBIDDEN {
            return Err(SaltError::Forbidden);
        }
        // If status != 200, something went wrong
        if res.status() != StatusCode::OK {
            return Err(SaltError::FailedRequest);
        }

        // Parse response from JSON stored as {return: [SaltToken]}
        let body = match res.json::<serde_json::Value>().await {
            Ok(body) => body,
            Err(e) => {
                error!("res.json: {:?}", e);
                return Err(SaltError::ResponseParseError(Some(e.to_string())));
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
                error!("Convert to SaltToken: {:?}", e);
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

    pub fn listen_events(&self, salt_token: &SaltToken) -> impl stream::Stream<Item = SaltEvent> {
        let url = format!(
            "{}/events?salt_token={}",
            SConfig::salt_api_url(),
            salt_token.token.clone()
        );

        let client = self.client.clone();
        stream! {
            debug!("Connecting to SSE stream: {}", &url);
            let res = match client
                .get(&url)
                .header("Accept", "text/event-stream")
                .send()
                .await
            {
                Ok(res) => res,
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

            // let mut stream = res.
            let mut stream = res.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = match chunk {
                    Ok(d) => d,
                    Err(e) => {
                        error!("stream.next: {:?}", e);
                        break;
                    }
                };
                trace!("SSE line {:?}", chunk);

                // Loop through every byte in line
                for byte in chunk {
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

        let res = match self
            .client
            .post(url)
            .header(X_AUTH_TOKEN, salt_token.token.clone())
            .json(&data)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                return Err(SaltError::RequestError(e.to_string()));
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
            return Err(SaltError::FailedRequest);
        }

        let body = match res.json::<serde_json::Value>().await {
            Ok(body) => body,
            Err(e) => {
                return Err(SaltError::ResponseParseError(Some(e.to_string())));
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
        kwarg: Option<HashMap<String, String>>,
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
        kwarg: Option<HashMap<String, String>>,
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
        kwarg: Option<HashMap<String, String>>,
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
        kwarg: Option<HashMap<String, String>>,
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
        kwarg: Option<HashMap<String, String>>,
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
        kwarg: Option<HashMap<String, String>>,
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
        kwarg: Option<HashMap<String, String>>,
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
