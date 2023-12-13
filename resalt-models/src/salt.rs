use std::collections::HashMap;

use serde::{ser::SerializeMap, Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
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
        strip_quotes(serde_json::to_string(self).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SaltRunJob {
    Local {
        tgt: String,
        fun: String,
        arg: Option<Vec<Value>>,
        tgt_type: Option<SaltTgtType>,
        kwarg: Option<HashMap<String, String>>,
    },
    LocalAsync {
        tgt: String,
        fun: String,
        arg: Option<Vec<Value>>,
        tgt_type: Option<SaltTgtType>,
        kwarg: Option<HashMap<String, String>>,
    },
    LocalBatch {
        tgt: String,
        fun: String,
        arg: Option<Vec<Value>>,
        tgt_type: Option<SaltTgtType>,
        kwarg: Option<HashMap<String, String>>,
        batch_size: String,
    },
    Runner {
        fun: String,
        arg: Option<Vec<Value>>,
        kwarg: Option<HashMap<String, String>>,
    },
    RunnerAsync {
        fun: String,
        arg: Option<Vec<Value>>,
        kwarg: Option<HashMap<String, String>>,
    },
    Wheel {
        fun: String,
        arg: Option<Vec<Value>>,
        kwarg: Option<HashMap<String, String>>,
    },
    WheelAsync {
        fun: String,
        arg: Option<Vec<Value>>,
        kwarg: Option<HashMap<String, String>>,
    },
}

impl Serialize for SaltRunJob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(7))?;
        match self {
            SaltRunJob::Local {
                tgt,
                fun,
                arg,
                tgt_type,
                kwarg,
            } => {
                map.serialize_entry("client", "local")?;
                map.serialize_entry("tgt", tgt)?;
                map.serialize_entry("fun", fun)?;
                map.serialize_entry("arg", &arg.clone().unwrap_or_default())?;
                map.serialize_entry("tgt_type", &tgt_type.unwrap_or_default())?;
                map.serialize_entry("kwarg", &kwarg.clone().unwrap_or_default())?;
            }
            SaltRunJob::LocalAsync {
                tgt,
                fun,
                arg,
                tgt_type,
                kwarg,
            } => {
                map.serialize_entry("client", "local_async")?;
                map.serialize_entry("tgt", tgt)?;
                map.serialize_entry("fun", fun)?;
                map.serialize_entry("arg", &arg.clone().unwrap_or_default())?;
                map.serialize_entry("tgt_type", &tgt_type.unwrap_or_default())?;
                map.serialize_entry("kwarg", &kwarg.clone().unwrap_or_default())?;
            }
            SaltRunJob::LocalBatch {
                tgt,
                fun,
                arg,
                tgt_type,
                kwarg,
                batch_size,
            } => {
                map.serialize_entry("client", "local_batch")?;
                map.serialize_entry("tgt", tgt)?;
                map.serialize_entry("fun", fun)?;
                map.serialize_entry("arg", &arg.clone().unwrap_or_default())?;
                map.serialize_entry("tgt_type", &tgt_type.unwrap_or_default())?;
                map.serialize_entry("kwarg", &kwarg.clone().unwrap_or_default())?;
                map.serialize_entry("batch_size", batch_size)?;
            }
            SaltRunJob::Runner { fun, arg, kwarg } => {
                map.serialize_entry("client", "runner")?;
                map.serialize_entry("fun", fun)?;
                map.serialize_entry("arg", &arg.clone().unwrap_or_default())?;
                map.serialize_entry("kwarg", &kwarg.clone().unwrap_or_default())?;
            }
            SaltRunJob::RunnerAsync { fun, arg, kwarg } => {
                map.serialize_entry("client", "runner_async")?;
                map.serialize_entry("fun", fun)?;
                map.serialize_entry("arg", &arg.clone().unwrap_or_default())?;
                map.serialize_entry("kwarg", &kwarg.clone().unwrap_or_default())?;
            }
            SaltRunJob::Wheel { fun, arg, kwarg } => {
                map.serialize_entry("client", "wheel")?;
                map.serialize_entry("fun", fun)?;
                map.serialize_entry("arg", &arg.clone().unwrap_or_default())?;
                map.serialize_entry("kwarg", &kwarg.clone().unwrap_or_default())?;
            }
            SaltRunJob::WheelAsync { fun, arg, kwarg } => {
                map.serialize_entry("client", "wheel_async")?;
                map.serialize_entry("fun", fun)?;
                map.serialize_entry("arg", &arg.clone().unwrap_or_default())?;
                map.serialize_entry("kwarg", &kwarg.clone().unwrap_or_default())?;
            }
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::{SaltRunJob, SaltTgtType};
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_saltrunjob() {
        let salt_run_job = SaltRunJob::Local {
            tgt: "test".to_string(),
            fun: "test".to_string(),
            arg: Some(vec![json!("test")]),
            tgt_type: Some(SaltTgtType::Glob),
            kwarg: Some(HashMap::new()),
        };
        assert_eq!(
            json!(salt_run_job),
            json!({
                "client": "local",
                "tgt": "test",
                "fun": "test",
                "arg": ["test"],
                "tgt_type": "glob",
                "kwarg": {}
            })
        );

        let salt_run_job = SaltRunJob::LocalAsync {
            tgt: "test".to_string(),
            fun: "test".to_string(),
            arg: Some(vec![json!("test")]),
            tgt_type: Some(SaltTgtType::Glob),
            kwarg: Some(HashMap::new()),
        };
        assert_eq!(
            json!(salt_run_job),
            json!({
                "client": "local_async",
                "tgt": "test",
                "fun": "test",
                "arg": ["test"],
                "tgt_type": "glob",
                "kwarg": {}
            })
        );

        let salt_run_job = SaltRunJob::LocalBatch {
            tgt: "test".to_string(),
            fun: "test".to_string(),
            arg: Some(vec![json!("test")]),
            tgt_type: Some(SaltTgtType::Glob),
            kwarg: Some(HashMap::new()),
            batch_size: "test".to_string(),
        };
        assert_eq!(
            json!(salt_run_job),
            json!({
                "client": "local_batch",
                "tgt": "test",
                "fun": "test",
                "arg": ["test"],
                "tgt_type": "glob",
                "kwarg": {},
                "batch_size": "test"
            })
        );

        let salt_run_job = SaltRunJob::Runner {
            fun: "test".to_string(),
            arg: Some(vec![json!("test")]),
            kwarg: Some(HashMap::new()),
        };
        assert_eq!(
            json!(salt_run_job),
            json!({
                "client": "runner",
                "fun": "test",
                "arg": ["test"],
                "kwarg": {}
            })
        );

        let salt_run_job = SaltRunJob::RunnerAsync {
            fun: "test".to_string(),
            arg: Some(vec![json!("test")]),
            kwarg: Some(HashMap::new()),
        };
        assert_eq!(
            json!(salt_run_job),
            json!({
                "client": "runner_async",
                "fun": "test",
                "arg": ["test"],
                "kwarg": {}
            })
        );

        let salt_run_job = SaltRunJob::Wheel {
            fun: "test".to_string(),
            arg: Some(vec![json!("test")]),
            kwarg: Some(HashMap::new()),
        };
        assert_eq!(
            json!(salt_run_job),
            json!({
                "client": "wheel",
                "fun": "test",
                "arg": ["test"],
                "kwarg": {}
            })
        );

        let salt_run_job = SaltRunJob::WheelAsync {
            fun: "test".to_string(),
            arg: Some(vec![json!("test")]),
            kwarg: Some(HashMap::new()),
        };
        assert_eq!(
            json!(salt_run_job),
            json!({
                "client": "wheel_async",
                "fun": "test",
                "arg": ["test"],
                "kwarg": {}
            })
        );
    }
}
