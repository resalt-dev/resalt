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
                "timestamp" => event.timestamp = ResaltTime::parse_from_rfc3339(&value).unwrap(),
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
    pub fn default_with_id(id: &str) -> Self {
        Self {
            id: id.to_owned(),
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
pub struct Preferences {
    pub theme: String,
}
impl Preferences {
    pub fn validate(&self) -> bool {
        self.theme.len() <= 32
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
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

impl Default for User {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            username: "".to_string(),
            password: None,
            perms: "".to_string(),
            last_login: None,
            email: None,
        }
    }
}

impl User {
    pub fn public(
        &self,
        permission_groups: Vec<PermissionGroup>,
        preferences: Preferences,
    ) -> Value {
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

        // Add preferences
        result.as_object_mut().unwrap().insert(
            "preferences".to_owned(),
            serde_json::to_value(&preferences).unwrap(),
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageStatus {
    pub auth_tokens_total: i64,
    pub auth_tokens_active: i64,
    pub events_total: i64,
    pub job_returns_total: i64,
    pub jobs_total: i64,
    pub minions_total: i64,
    pub minions_success: i64,
    pub minions_incorrect: i64,
    pub minions_error: i64,
    pub minions_unknown: i64,
    pub permission_group_users_total: i64,
    pub permission_groups_total: i64,
    pub users_total: i64,
}
