use crate::{ResaltTime, SaltToken};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/*
=========================
=    DATABASE MODELS    =
=========================
*/

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthToken {
    pub id: String,
    pub user_id: String,
    pub timestamp: ResaltTime,
    pub salt_token: Option<SaltToken>,
}

impl AuthToken {
    pub fn hash(&self) -> Vec<(&str, String)> {
        let mut values = Vec::from([
            ("user_id", self.user_id.clone()),
            (
                "timestamp",
                self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            ),
        ]);
        if let Some(salt_token) = &self.salt_token {
            values.push(("salt_token", serde_json::to_string(salt_token).unwrap()));
        }
        values
    }

    pub fn dehash(id: String, values: Vec<(String, String)>) -> Self {
        let mut auth_token = AuthToken {
            id,
            user_id: "".to_string(),
            timestamp: ResaltTime::default(),
            salt_token: None,
        };
        for (key, value) in values {
            match key.as_str() {
                "user_id" => auth_token.user_id = value,
                "timestamp" => {
                    auth_token.timestamp =
                        ResaltTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap()
                }
                "salt_token" => auth_token.salt_token = Some(serde_json::from_str(&value).unwrap()),
                _ => (),
            }
        }
        auth_token
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub timestamp: ResaltTime,
    pub tag: String,
    pub data: String,
}

impl Event {
    pub fn hash(&self) -> Vec<(&str, String)> {
        let values = Vec::from([
            (
                "timestamp",
                self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            ),
            ("tag", self.tag.clone()),
            ("data", self.data.clone()),
        ]);
        values
    }

    pub fn dehash(id: String, values: Vec<(String, String)>) -> Self {
        let mut event = Event {
            id,
            timestamp: ResaltTime::default(),
            tag: "".to_string(),
            data: "".to_string(),
        };
        for (key, value) in values {
            match key.as_str() {
                "timestamp" => {
                    event.timestamp =
                        ResaltTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap()
                }
                "tag" => event.tag = value,
                "data" => event.data = value,
                _ => (),
            }
        }
        event
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub timestamp: ResaltTime,
    pub jid: String,
    pub user: Option<String>,
    #[serde(rename = "eventId")]
    pub event_id: Option<String>,
}

impl Job {
    pub fn hash(&self) -> Vec<(&str, String)> {
        let mut values = Vec::from([(
            "timestamp",
            self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
        )]);
        if let Some(user) = &self.user {
            values.push(("user", user.clone()));
        }
        if let Some(event_id) = &self.event_id {
            values.push(("event_id", event_id.clone()));
        }
        values
    }

    pub fn dehash(id: String, values: Vec<(String, String)>) -> Self {
        let mut job = Job {
            id: id.clone(),
            timestamp: ResaltTime::default(),
            jid: id,
            user: None,
            event_id: None,
        };
        for (key, value) in values {
            match key.as_str() {
                "timestamp" => {
                    job.timestamp = ResaltTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap()
                }
                "user" => job.user = Some(value),
                "event_id" => job.event_id = Some(value),
                _ => (),
            }
        }
        job
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobReturn {
    pub id: String,
    pub timestamp: ResaltTime,
    pub jid: String,
    #[serde(rename = "jobId")]
    pub job_id: String,
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "minionId")]
    pub minion_id: String,
}

impl JobReturn {
    pub fn hash(&self) -> Vec<(&str, String)> {
        let values = Vec::from([
            (
                "timestamp",
                self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            ),
            ("jid", self.jid.clone()),
            ("job_id", self.job_id.clone()),
            ("event_id", self.event_id.clone()),
            ("minion_id", self.minion_id.clone()),
        ]);
        values
    }

    pub fn dehash(id: String, values: Vec<(String, String)>) -> Self {
        let mut job_return = JobReturn {
            id,
            timestamp: ResaltTime::default(),
            jid: "".to_string(),
            job_id: "".to_string(),
            event_id: "".to_string(),
            minion_id: "".to_string(),
        };
        for (key, value) in values {
            match key.as_str() {
                "timestamp" => {
                    job_return.timestamp =
                        ResaltTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap()
                }
                "jid" => job_return.jid = value,
                "job_id" => job_return.job_id = value,
                "event_id" => job_return.event_id = value,
                "minion_id" => job_return.minion_id = value,
                _ => (),
            }
        }
        job_return
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Minion {
    pub id: String,
    #[serde(rename = "lastSeen")]
    pub last_seen: ResaltTime,
    pub grains: Option<String>,
    pub pillars: Option<String>,
    pub pkgs: Option<String>,
    #[serde(rename = "lastUpdatedGrains")]
    pub last_updated_grains: Option<ResaltTime>,
    #[serde(rename = "lastUpdatedPillars")]
    pub last_updated_pillars: Option<ResaltTime>,
    #[serde(rename = "lastUpdatedPkgs")]
    pub last_updated_pkgs: Option<ResaltTime>,
    pub conformity: Option<String>,
    #[serde(rename = "conformitySuccess")]
    pub conformity_success: Option<i32>,
    #[serde(rename = "conformityIncorrect")]
    pub conformity_incorrect: Option<i32>,
    #[serde(rename = "conformityError")]
    pub conformity_error: Option<i32>,
    #[serde(rename = "lastUpdatedConformity")]
    pub last_updated_conformity: Option<ResaltTime>,
    #[serde(rename = "osType")]
    pub os_type: Option<String>,
}

impl Minion {
    pub fn hash(&self) -> Vec<(&str, String)> {
        let mut values = Vec::from([(
            "last_seen",
            self.last_seen.format("%Y-%m-%d %H:%M:%S").to_string(),
        )]);
        if let Some(grains) = &self.grains {
            values.push(("grains", grains.clone()));
        }
        if let Some(pillars) = &self.pillars {
            values.push(("pillars", pillars.clone()));
        }
        if let Some(pkgs) = &self.pkgs {
            values.push(("pkgs", pkgs.clone()));
        }
        if let Some(last_updated_grains) = self.last_updated_grains {
            values.push((
                "last_updated_grains",
                last_updated_grains.format("%Y-%m-%d %H:%M:%S").to_string(),
            ));
        }
        if let Some(last_updated_pillars) = self.last_updated_pillars {
            values.push((
                "last_updated_pillars",
                last_updated_pillars.format("%Y-%m-%d %H:%M:%S").to_string(),
            ));
        }
        if let Some(last_updated_pkgs) = self.last_updated_pkgs {
            values.push((
                "last_updated_pkgs",
                last_updated_pkgs.format("%Y-%m-%d %H:%M:%S").to_string(),
            ));
        }
        if let Some(conformity) = &self.conformity {
            values.push(("conformity", conformity.clone()));
        }
        if let Some(conformity_success) = self.conformity_success {
            values.push(("conformity_success", conformity_success.to_string()));
        }
        if let Some(conformity_incorrect) = self.conformity_incorrect {
            values.push(("conformity_incorrect", conformity_incorrect.to_string()));
        }
        if let Some(conformity_error) = self.conformity_error {
            values.push(("conformity_error", conformity_error.to_string()));
        }
        if let Some(last_updated_conformity) = self.last_updated_conformity {
            values.push((
                "last_updated_conformity",
                last_updated_conformity
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            ));
        }
        if let Some(os_type) = &self.os_type {
            values.push(("os_type", os_type.clone()));
        }
        values
    }

    pub fn dehash(id: String, values: Vec<(String, String)>) -> Self {
        let mut minion = Minion {
            id,
            last_seen: ResaltTime::default(),
            grains: None,
            pillars: None,
            pkgs: None,
            last_updated_grains: None,
            last_updated_pillars: None,
            last_updated_pkgs: None,
            conformity: None,
            conformity_success: None,
            conformity_incorrect: None,
            conformity_error: None,
            last_updated_conformity: None,
            os_type: None,
        };
        for (key, value) in values {
            match key.as_str() {
                "last_seen" => {
                    minion.last_seen =
                        ResaltTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap()
                }
                "grains" => minion.grains = Some(value),
                "pillars" => minion.pillars = Some(value),
                "pkgs" => minion.pkgs = Some(value),
                "last_updated_grains" => {
                    minion.last_updated_grains =
                        Some(ResaltTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap())
                }
                "last_updated_pillars" => {
                    minion.last_updated_pillars =
                        Some(ResaltTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap())
                }
                "last_updated_pkgs" => {
                    minion.last_updated_pkgs =
                        Some(ResaltTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap())
                }
                "conformity" => minion.conformity = Some(value),
                "conformity_success" => {
                    minion.conformity_success = Some(value.parse::<i32>().unwrap())
                }
                "conformity_incorrect" => {
                    minion.conformity_incorrect = Some(value.parse::<i32>().unwrap())
                }
                "conformity_error" => minion.conformity_error = Some(value.parse::<i32>().unwrap()),
                "last_updated_conformity" => {
                    minion.last_updated_conformity =
                        Some(ResaltTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap())
                }
                "os_type" => minion.os_type = Some(value),
                _ => (),
            }
        }
        minion
    }

    pub fn default_with_id(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl Default for Minion {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            last_seen: ResaltTime::default(),
            grains: None,
            pillars: None,
            pkgs: None,
            last_updated_grains: None,
            last_updated_pillars: None,
            last_updated_pkgs: None,
            conformity: None,
            conformity_success: None,
            conformity_incorrect: None,
            conformity_error: None,
            last_updated_conformity: None,
            os_type: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub perms: String,
    #[serde(rename = "lastLogin")]
    pub last_login: Option<ResaltTime>,
    pub email: Option<String>,
}

impl User {
    pub fn public(&self, permission_groups: Vec<PermissionGroup>) -> Value {
        let mut result: Value = serde_json::value::to_value(self).unwrap();
        // Remove password
        result.as_object_mut().unwrap().remove("password");

        // Add groups
        let permission_groups_json = permission_groups
            .iter()
            .map(|g| {
                json!({
                    "id": g.id,
                    "name": g.name,
                })
            })
            .collect::<Vec<Value>>();
        result.as_object_mut().unwrap().insert(
            "permissionGroups".to_owned(),
            serde_json::Value::Array(permission_groups_json),
        );

        // Convert "perms" to array
        let perms: Value = match serde_json::from_str(&self.perms) {
            Ok(perms) => perms,
            Err(_) => json!(Vec::<String>::new()),
        };
        result
            .as_object_mut()
            .unwrap()
            .insert("perms".to_owned(), perms);

        result
    }

    pub fn hash(&self) -> Vec<(&str, String)> {
        let mut values = Vec::from([
            ("username", self.username.clone()),
            ("perms", self.perms.clone()),
        ]);
        if let Some(password) = &self.password {
            values.push(("password", password.clone()));
        }
        if let Some(last_login) = self.last_login {
            values.push((
                "last_login",
                last_login.format("%Y-%m-%d %H:%M:%S").to_string(),
            ));
        }
        if let Some(email) = &self.email {
            values.push(("email", email.clone()));
        }
        values
    }

    pub fn dehash(id: String, values: Vec<(String, String)>) -> Self {
        let mut user = User {
            id,
            username: "".to_string(),
            password: None,
            perms: "[]".to_string(),
            last_login: None,
            email: None,
        };
        for (key, value) in values {
            match key.as_str() {
                "username" => user.username = value,
                "password" => user.password = Some(value),
                "perms" => user.perms = value,
                "last_login" => {
                    user.last_login =
                        Some(ResaltTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap())
                }
                "email" => user.email = Some(value),
                _ => (),
            }
        }
        user
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionGroup {
    pub id: String,
    pub name: String,
    pub perms: String,
}

impl PermissionGroup {
    pub fn public(&self, users: Vec<User>) -> serde_json::Value {
        let perms: Value = match serde_json::from_str(&self.perms) {
            Ok(perms) => perms,
            Err(_) => json!(Vec::<String>::new()),
        };
        serde_json::json!({
            "id": self.id,
            "name": self.name,
            "perms": perms,
            "users": users.iter().map(|u| json!({
                "id": u.id,
                "username": u.username,
            })).collect::<Vec<Value>>(),
        })
    }

    pub fn hash(&self) -> Vec<(&str, String)> {
        Vec::from([("name", self.name.clone()), ("perms", self.perms.clone())])
    }

    pub fn dehash(id: String, values: Vec<(String, String)>) -> Self {
        let mut permission_group = PermissionGroup {
            id,
            name: "".to_string(),
            perms: "[]".to_string(),
        };
        for (key, value) in values {
            match key.as_str() {
                "name" => permission_group.name = value,
                "perms" => permission_group.perms = value,
                _ => (),
            }
        }
        permission_group
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PermissionGroupUser {
    pub id: String,
    pub group_id: String,
    pub user_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinionPreset {
    pub id: String,
    pub name: String,
    pub filter: String,
}

impl MinionPreset {
    pub fn hash(&self) -> Vec<(&str, String)> {
        Vec::from([("name", self.name.clone()), ("filter", self.filter.clone())])
    }

    pub fn dehash(id: String, values: Vec<(String, String)>) -> Self {
        let mut minion_preset = MinionPreset {
            id,
            name: "".to_string(),
            filter: "".to_string(),
        };
        for (key, value) in values {
            match key.as_str() {
                "name" => minion_preset.name = value,
                "filter" => minion_preset.filter = value,
                _ => (),
            }
        }
        minion_preset
    }
}
