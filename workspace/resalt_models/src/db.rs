use crate::ResaltTime;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/*
=========================
=    DATABASE MODELS    =
=========================
*/

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthToken {
    pub id: String,
    pub user_id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub salt_token: Option<String>,
}

impl AuthToken {
    pub fn hash(&self) -> Vec<(&str, String)> {
        let mut values = Vec::from([
            ("id", self.id.clone()),
            ("user_id", self.user_id.clone()),
            (
                "timestamp",
                self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            ),
        ]);
        if let Some(salt_token) = &self.salt_token {
            values.push(("salt_token", salt_token.clone()));
        }
        values
    }

    pub fn dehash(values: Vec<(String, String)>) -> Self {
        let mut auth_token = AuthToken {
            id: "".to_string(),
            user_id: "".to_string(),
            timestamp: chrono::NaiveDateTime::default(),
            salt_token: None,
        };
        for (key, value) in values {
            match key.as_str() {
                "id" => auth_token.id = value,
                "user_id" => auth_token.user_id = value,
                "timestamp" => {
                    auth_token.timestamp =
                        chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap()
                }
                "salt_token" => auth_token.salt_token = Some(value),
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
    pub fn hash(&self) -> Vec<(&str, String)> {
        let mut values = Vec::from([
            ("id", self.id.clone()),
            (
                "last_seen",
                self.last_seen.format("%Y-%m-%d %H:%M:%S").to_string(),
            ),
        ]);
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

    pub fn dehash(values: Vec<(String, String)>) -> Self {
        let mut minion = Minion {
            id: "".to_string(),
            last_seen: chrono::NaiveDateTime::default(),
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
                "id" => minion.id = value,
                "last_seen" => {
                    minion.last_seen =
                        chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap()
                }
                "grains" => minion.grains = Some(value),
                "pillars" => minion.pillars = Some(value),
                "pkgs" => minion.pkgs = Some(value),
                "last_updated_grains" => {
                    minion.last_updated_grains = Some(
                        chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap(),
                    )
                }
                "last_updated_pillars" => {
                    minion.last_updated_pillars = Some(
                        chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap(),
                    )
                }
                "last_updated_pkgs" => {
                    minion.last_updated_pkgs = Some(
                        chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap(),
                    )
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
                    minion.last_updated_conformity = Some(
                        chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap(),
                    )
                }
                "os_type" => minion.os_type = Some(value),
                _ => (),
            }
        }
        minion
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
    #[serde(rename = "ldapSync")]
    pub ldap_sync: Option<String>,
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

        return result;
    }

    pub fn hash(&self) -> Vec<(&str, String)> {
        let mut values = Vec::from([
            ("id", self.id.clone()),
            ("username", self.username.clone()),
            ("perms", "[]".to_owned()),
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
        if let Some(ldap_sync) = &self.ldap_sync {
            values.push(("ldap_sync", ldap_sync.clone()));
        }
        values
    }

    pub fn dehash(values: Vec<(String, String)>) -> Self {
        let mut user = User {
            id: "".to_string(),
            username: "".to_string(),
            password: None,
            perms: "[]".to_string(),
            last_login: None,
            email: None,
            ldap_sync: None,
        };
        for (key, value) in values {
            match key.as_str() {
                "id" => user.id = value,
                "username" => user.username = value,
                "password" => user.password = Some(value),
                "perms" => user.perms = value,
                "last_login" => {
                    user.last_login = Some(
                        chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").unwrap(),
                    )
                }
                "email" => user.email = Some(value),
                "ldap_sync" => user.ldap_sync = Some(value),
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
    #[serde(rename = "ldapSync")]
    pub ldap_sync: Option<String>,
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
            "ldapSync": self.ldap_sync,
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
