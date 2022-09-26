use crate::{prelude::evalute_resalt_permission, schema::*};
use serde::{ser::SerializeStruct, *};
use serde_json::{json, Value};

/*
=========================
=    DATABASE MODELS    =
=========================
*/

#[derive(
    Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset, Associations,
)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = authtokens)]
pub struct AuthToken {
    pub id: String,
    pub user_id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub salt_token: Option<String>,
}

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset)]
#[diesel(table_name = events)]
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

#[derive(
    Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset, Associations,
)]
#[diesel(belongs_to(Event, foreign_key = event_id))]
#[diesel(table_name = jobs)]
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

#[derive(
    Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset, Associations,
)]
#[diesel(belongs_to(Job, foreign_key = job_id))]
#[diesel(belongs_to(Event, foreign_key = event_id))]
#[diesel(belongs_to(Minion, foreign_key = minion_id))]
#[diesel(table_name = job_returns)]
pub struct JobReturn {
    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub jid: String,
    pub job_id: String,
    pub event_id: String,
    pub minion_id: String,
}

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset)]
#[diesel(table_name = minions)]
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
            last_seen: chrono::NaiveDateTime::from_timestamp(0, 0),
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

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub perms: String,
    pub last_login: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn public(&self, permission_groups: Vec<PermissionGroup>) -> serde_json::Value {
        let perms: Value = match serde_json::from_str(&self.perms) {
            Ok(perms) => perms,
            Err(_) => json!(Vec::<String>::new()),
        };
        serde_json::json!({
            "id": self.id,
            "username": self.username,
            "isLocal": self.password.is_some(),
            "perms": perms,
            "lastLogin": self.last_login.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string()),
            "permissionGroups": permission_groups.iter().map(|g| json!({
                "id": g.id,
                "name": g.name,
            })).collect::<Vec<Value>>(),

        })
    }

    pub fn has_permission(&self, perm: &str) -> bool {
        let perms: Value = match serde_json::from_str(&self.perms) {
            Ok(perms) => perms,
            Err(_) => return false,
        };
        evalute_resalt_permission(&perms, perm)
            || evalute_resalt_permission(&perms, "admin.superadmin")
    }
}

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset)]
#[diesel(table_name = permission_groups)]
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

#[derive(
    Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset, Associations,
)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(PermissionGroup, foreign_key = group_id))]
#[diesel(table_name = permission_group_users)]
pub struct PermissionGroupUser {
    pub id: String,
    pub group_id: String,
    pub user_id: String,
}

/*
===========================
=   NON-DATABASE MODELS   =
===========================
*/
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
        self.expire < chrono::Utc::now().timestamp() as f64
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuthStatus {
    pub user_id: String,
    pub salt_token: Option<SaltToken>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaltMinionKey {
    pub id: String,
    pub state: String,
    pub finger: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricResult {
    pub title: String,
    pub chart: String,
    pub labels: Vec<String>,
    pub data: Vec<MetricResultData>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricResultData {
    pub label: String,
    pub data: Vec<i32>,
}
