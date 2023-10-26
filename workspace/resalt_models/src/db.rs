use serde::{ser::SerializeStruct, *};
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub tag: String,
    pub data: String,
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = self.timestamp.format("%Y-%m-%d %H:%M:%S%.6f").to_string();
        let mut state = serializer.serialize_struct("Event", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("timestamp", &timestamp)?;
        state.serialize_field("tag", &self.tag)?;
        state.serialize_field("data", &self.data)?;
        state.end()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Job {
    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub jid: String,
    pub user: Option<String>,
    pub event_id: Option<String>,
}

impl Serialize for Job {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = self.timestamp.format("%Y-%m-%d %H:%M:%S%.6f").to_string();
        let mut state = serializer.serialize_struct("Job", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("timestamp", &timestamp)?;
        state.serialize_field("jid", &self.jid)?;
        state.serialize_field("user", &self.user)?;
        state.serialize_field("event_id", &self.event_id)?;
        state.end()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JobReturn {
    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub jid: String,
    pub job_id: String,
    pub event_id: String,
    pub minion_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Minion {
    pub id: String,
    pub last_seen: chrono::NaiveDateTime,
    pub grains: Option<String>,
    pub pillars: Option<String>,
    pub pkgs: Option<String>,
    pub last_updated_grains: Option<chrono::NaiveDateTime>,
    pub last_updated_pillars: Option<chrono::NaiveDateTime>,
    pub last_updated_pkgs: Option<chrono::NaiveDateTime>,
    pub conformity: Option<String>,
    pub conformity_success: Option<i32>,
    pub conformity_incorrect: Option<i32>,
    pub conformity_error: Option<i32>,
    pub last_updated_conformity: Option<chrono::NaiveDateTime>,
    pub os_type: Option<String>,
}

impl Default for Minion {
    fn default() -> Self {
        Self {
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
        }
    }
}

impl Serialize for Minion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let last_seen = self.last_seen.format("%Y-%m-%d %H:%M:%S").to_string();
        let last_updated_grains = self
            .last_updated_grains
            .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string());
        let last_updated_pillars = self
            .last_updated_pillars
            .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string());
        let last_updated_pkgs = self
            .last_updated_pkgs
            .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string());
        let last_updated_conformity = self
            .last_updated_conformity
            .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string());

        let mut state = serializer.serialize_struct("Minion", 13)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("lastSeen", &last_seen)?;
        state.serialize_field("grains", &self.grains)?;
        state.serialize_field("pillars", &self.pillars)?;
        state.serialize_field("pkgs", &self.pkgs)?;
        state.serialize_field("lastUpdatedGrains", &last_updated_grains)?;
        state.serialize_field("lastUpdatedPillars", &last_updated_pillars)?;
        state.serialize_field("lastUpdatedPkgs", &last_updated_pkgs)?;
        state.serialize_field("conformity", &self.conformity)?;
        state.serialize_field("conformitySuccess", &self.conformity_success)?;
        state.serialize_field("conformityIncorrect", &self.conformity_incorrect)?;
        state.serialize_field("conformityError", &self.conformity_error)?;
        state.serialize_field("lastUpdatedConformity", &last_updated_conformity)?;
        state.serialize_field("osType", &self.os_type)?;
        state.end()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub perms: String,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
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
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let last_login = self
            .last_login
            .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string());
        let mut state = serializer.serialize_struct("User", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("username", &self.username)?;
        state.serialize_field("password", &self.password)?;
        state.serialize_field("perms", &self.perms)?;
        state.serialize_field("lastLogin", &last_login)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("ldapSync", &self.ldap_sync)?;
        state.end()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PermissionGroup {
    pub id: String,
    pub name: String,
    pub perms: String,
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

impl Serialize for PermissionGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("PermissionGroup", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("perms", &self.perms)?;
        state.serialize_field("ldapSync", &self.ldap_sync)?;
        state.end()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PermissionGroupUser {
    pub id: String,
    pub group_id: String,
    pub user_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinionPreset {
    pub id: String,
    pub name: String,
    pub filter: String,
}

impl Serialize for MinionPreset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MinionPreset", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("filter", &self.filter)?;
        state.end()
    }
}
