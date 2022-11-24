use crate::schema::*;

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

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub perms: String,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub ldap_sync: Option<String>,
}

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Queryable, AsChangeset)]
#[diesel(table_name = permission_groups)]
pub struct PermissionGroup {
    pub id: String,
    pub name: String,
    pub perms: String,
    pub ldap_sync: Option<String>,
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
