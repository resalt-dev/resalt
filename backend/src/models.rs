use crate::schema::*;
use serde::{ser::SerializeStruct, *};

/*
=========================
=    DATABASE MODELS    =
=========================
*/

#[derive(Debug, Identifiable, Associations, Insertable, PartialEq, Queryable)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "authtokens"]
pub struct AuthToken {
    pub id: String,
    pub user_id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub salt_token: Option<String>,
}

#[derive(Debug, Identifiable, Insertable, PartialEq, Queryable)]
#[table_name = "events"]
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

#[derive(Debug, Identifiable, Associations, Insertable, PartialEq, Queryable)]
#[belongs_to(Event, foreign_key = "event_id")]
#[table_name = "jobs"]
pub struct Job {
    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub jid: String,
    pub user: String,
    pub minions: String, // JSON
    pub event_id: String,
}

impl Serialize for Job {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = self.timestamp.format("%Y-%m-%d %H:%M:%S%.6f").to_string();
        let minions: Vec<String> = serde_json::from_str(&self.minions).unwrap();
        let mut state = serializer.serialize_struct("Job", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("timestamp", &timestamp)?;
        state.serialize_field("jid", &self.jid)?;
        state.serialize_field("user", &self.user)?;
        state.serialize_field("minions", &minions)?;
        state.serialize_field("event_id", &self.event_id)?;
        state.end()
    }
}

#[derive(Debug, Identifiable, Associations, Insertable, PartialEq, Queryable)]
#[belongs_to(Job, foreign_key = "job_id")]
#[belongs_to(Event, foreign_key = "event_id")]
#[table_name = "job_returns"]
pub struct JobReturn {
    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub jid: String,
    pub job_id: String,
    pub event_id: String,
}

#[derive(Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset)]
#[table_name = "minions"]
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
        state.serialize_field("last_seen", &last_seen)?;
        state.serialize_field("grains", &self.grains)?;
        state.serialize_field("pillars", &self.pillars)?;
        state.serialize_field("pkgs", &self.pkgs)?;
        state.serialize_field("last_updated_grains", &last_updated_grains)?;
        state.serialize_field("last_updated_pillars", &last_updated_pillars)?;
        state.serialize_field("last_updated_pkgs", &last_updated_pkgs)?;
        state.serialize_field("conformity", &self.conformity)?;
        state.serialize_field("conformity_success", &self.conformity_success)?;
        state.serialize_field("conformity_incorrect", &self.conformity_incorrect)?;
        state.serialize_field("conformity_error", &self.conformity_error)?;
        state.serialize_field("last_updated_conformity", &last_updated_conformity)?;
        state.end()
    }
}

#[derive(Debug, Identifiable, Insertable, PartialEq, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
}

/*
===========================
=   NON-DATABASE MODELS   =
===========================
*/

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SaltToken {
    pub token: String,
    pub start: f64,
    pub expire: f64,
    pub user: String,
    pub eauth: String,
    pub perms: serde_json::Value,
}

#[derive(Debug, PartialEq)]
pub struct AuthStatus {
    pub user_id: String,
    pub salt_token: Option<SaltToken>,
}
