use crate::prelude::*;
use actix_web::http::StatusCode;
use async_stream::stream;
use awc::{
    error::{JsonPayloadError, SendRequestError},
    *,
};
use futures::StreamExt;
use log::*;
use rustls::ClientConfig;
use rustls_native_certs::load_native_certs;
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

const X_AUTH_TOKEN: &str = "X-Auth-Token";

#[derive(Clone, Debug, Default)]
pub struct SaltEvent {
    pub tag: String,
    pub data: String,
}

#[derive(Debug)]
pub enum SaltError {
    Forbidden,
    RequestError(SendRequestError),
    ResponseParseError(Option<JsonPayloadError>),
    FailedRequest(
        ClientResponse<
            actix_web::dev::Decompress<
                actix_web::dev::Payload<
                    std::pin::Pin<
                        Box<
                            dyn futures_core::Stream<
                                Item = Result<actix_web::web::Bytes, error::PayloadError>,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    ),
}

#[derive(Default)]
pub enum SaltTgtType {
    #[default]
    Glob,
    PCRE,
    List,
    Grain,
    GrainPCRE,
    Pillar,
    PillarPCRE,
    NodeGroup,
    NodeGroupPCRE,
    Range,
    RangePCRE,
    Compound,
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
            SaltTgtType::NodeGroupPCRE => "nodegroup_pcre".to_string(),
            SaltTgtType::Range => "range".to_string(),
            SaltTgtType::RangePCRE => "range_pcre".to_string(),
            SaltTgtType::Compound => "compound".to_string(),
            SaltTgtType::IPCIDR => "ipcidr".to_string(),
        }
    }
}

type Dictionary = HashMap<String, String>;

lazy_static::lazy_static! {
    static ref AWC_CONFIG: ClientConfig = {
        let certs = load_native_certs().unwrap();

        // Convert Vec<rustls_native_certs::Certificate> to RootCertStore
        let mut root_store = rustls::RootCertStore::empty();
        for cert in certs {
            root_store.add(&rustls::Certificate(cert.0)).unwrap();
        }

        let mut config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        if !SConfig::salt_api_tls_verify() {
            config
                .dangerous()
                .set_certificate_verifier(Arc::new(danger::NoCertificateVerification));
        }

        config
    };
}

#[derive(Clone)]
pub struct SaltAPI {
    client: Arc<Mutex<awc::Client>>,
}

impl SaltAPI {
    pub(crate) fn create_awc_client() -> awc::Client {
        awc::Client::builder()
            .connector(Connector::new().rustls(Arc::new(AWC_CONFIG.to_owned())))
            .finish()
    }

    pub fn new() -> Self {
        Self {
            //client: Client::default(),
            client: Arc::new(Mutex::new(SaltAPI::create_awc_client())),
        }
    }

    pub async fn login(&self, username: &str, authtoken: &str) -> Result<SaltToken, SaltError> {
        let url = format!("{}/login", &SConfig::salt_api_url());
        // Send POST request to Salt API for auth token
        // This will contact us back on the /token endpoint to validate auth token
        let mut res = match self
            .client
            .lock()
            .unwrap()
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
        let salt_token: SaltToken = match serde_json::from_value(salt_token.clone()) {
            Ok(salt_token) => salt_token,
            Err(e) => {
                error!("{:?}", e);
                return Err(SaltError::ResponseParseError(None));
            }
        };

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

        let client = self.client.lock().unwrap().clone();
        stream! {
            debug!("Connecting to SSE stream: {}", &url);
            let mut stream = client
                .get(url)
                .insert_header(("Accept", "text/event-stream"))
                .send()
                .await
                .unwrap();

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

            debug!("SSE stream closed");
        }
    }

    async fn run_job(
        &self,
        salt_token: &SaltToken,
        data: serde_json::Value,
    ) -> Result<(), SaltError> {
        let url = &SConfig::salt_api_url();

        let mut res = match self
            .client
            .lock()
            .unwrap()
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

        Ok(())
    }

    async fn run_job_local<S: AsRef<str>>(
        &self,
        salt_token: &SaltToken,
        tgt: S,
        fun: S,
        arg: Option<Vec<S>>,
        timeout: Option<u64>,
        tgt_type: Option<SaltTgtType>,
        kwarg: Option<Dictionary>,
    ) -> Result<(), SaltError> {
        let data = json!({
            "client": "local",
            "tgt": tgt.as_ref(),
            "fun": fun.as_ref(),
            "arg": arg.map(|v| v.iter().map(|s| s.as_ref()).collect::<String>()).unwrap_or_default(),
            "timeout": timeout,
            "tgt_type": (tgt_type.unwrap_or_default()).to_string(),
            "kwarg": kwarg.unwrap_or_default(),
        });
        self.run_job(salt_token, data).await
    }

    async fn run_job_local_async<S: AsRef<str>>(
        &self,
        salt_token: &SaltToken,
        tgt: S,
        fun: S,
        arg: Option<Vec<S>>,
        tgt_type: Option<SaltTgtType>,
        kwarg: Option<Dictionary>,
    ) -> Result<(), SaltError> {
        let data = json!({
            "client": "local_async",
            "tgt": tgt.as_ref(),
            "fun": fun.as_ref(),
            "arg": arg.map(|v| v.iter().map(|s| s.as_ref()).collect::<String>()).unwrap_or_default(),
            "tgt_type": (tgt_type.unwrap_or_default()).to_string(),
            "kwarg": kwarg.unwrap_or_default(),
        });
        self.run_job(salt_token, data).await
    }

    async fn run_job_local_batch<S: AsRef<str>>(
        &self,
        salt_token: &SaltToken,
        tgt: S,
        fun: S,
        arg: Option<Vec<S>>,
        tgt_type: Option<SaltTgtType>,
        kwarg: Option<Dictionary>,
        batch: S,
    ) -> Result<(), SaltError> {
        let data = json!({
            "client": "local_batch",
            "tgt": tgt.as_ref(),
            "fun": fun.as_ref(),
            "arg": arg.map(|v| v.iter().map(|s| s.as_ref()).collect::<String>()).unwrap_or_default(),
            "tgt_type": (tgt_type.unwrap_or_default()).to_string(),
            "kwarg": kwarg.unwrap_or_default(),
            "batch": batch.as_ref(),
        });
        self.run_job(salt_token, data).await
    }

    pub async fn refresh_minions(&self, salt_token: &SaltToken) -> Result<(), SaltError> {
        let state = self
            .run_job_local_async(
                salt_token,
                "*",
                "state.highstate",
                Some(vec!["test=True"]),
                None,
                None,
            )
            .await;
        let grains = self
            .run_job_local_async(salt_token, "*", "grains.items", None, None, None)
            .await;
        let pillar = self
            .run_job_local_async(salt_token, "*", "pillar.items", None, None, None)
            .await;
        let pkg = self
            .run_job_local_async(salt_token, "*", "pkg.list_pkgs", None, None, None)
            .await;

        // TODO: sync with key-management, add non-responsive minions, and remove deleted ones

        if let Err(e) = state {
            return Err(e);
        }
        if let Err(e) = grains {
            return Err(e);
        }
        if let Err(e) = pillar {
            return Err(e);
        }
        if let Err(e) = pkg {
            return Err(e);
        }
        Ok(())
    }
}

mod danger {
    use rustls::client::*;
    use std::time::SystemTime;

    pub struct NoCertificateVerification;

    impl ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &rustls::Certificate,
            _intermediates: &[rustls::Certificate],
            _server_name: &ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp_response: &[u8],
            _now: SystemTime,
        ) -> Result<ServerCertVerified, rustls::Error> {
            Ok(ServerCertVerified::assertion())
        }
    }
}
