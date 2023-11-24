use crate::schema::*;
use resalt_models::*;

/*
=========================
=    DATABASE MODELS    =
=========================
*/

#[derive(
    Clone, Debug, Identifiable, Insertable, PartialEq, Eq, Queryable, AsChangeset, Associations,
)]
#[diesel(belongs_to(SQLUser, foreign_key = user_id))]
#[diesel(table_name = authtokens)]
pub struct SQLAuthToken {
    pub id: String,
    pub user_id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub salt_token: Option<String>,
}

impl From<AuthToken> for SQLAuthToken {
    fn from(auth_token: AuthToken) -> Self {
        SQLAuthToken {
            id: auth_token.id,
            user_id: auth_token.user_id,
            timestamp: auth_token.timestamp.into(),
            salt_token: auth_token.salt_token,
        }
    }
}

impl From<SQLAuthToken> for AuthToken {
    fn from(sql_auth_token: SQLAuthToken) -> Self {
        AuthToken {
            id: sql_auth_token.id,
            user_id: sql_auth_token.user_id,
            timestamp: sql_auth_token.timestamp.into(),
            salt_token: sql_auth_token.salt_token,
        }
    }
}

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Eq, Queryable, AsChangeset)]
#[diesel(table_name = events)]
pub struct SQLEvent {
    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub tag: String,
    pub data: String,
}

impl From<Event> for SQLEvent {
    fn from(event: Event) -> Self {
        SQLEvent {
            id: event.id,
            timestamp: event.timestamp.into(),
            tag: event.tag,
            data: event.data,
        }
    }
}

impl From<SQLEvent> for Event {
    fn from(sql_event: SQLEvent) -> Self {
        Event {
            id: sql_event.id,
            timestamp: sql_event.timestamp.into(),
            tag: sql_event.tag,
            data: sql_event.data,
        }
    }
}

#[derive(
    Clone, Debug, Identifiable, Insertable, PartialEq, Eq, Queryable, AsChangeset, Associations,
)]
#[diesel(belongs_to(SQLEvent, foreign_key = event_id))]
#[diesel(table_name = jobs)]
pub struct SQLJob {
    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub jid: String,
    pub user: Option<String>,
    pub event_id: Option<String>,
}

impl From<Job> for SQLJob {
    fn from(job: Job) -> Self {
        SQLJob {
            id: job.id,
            timestamp: job.timestamp.into(),
            jid: job.jid,
            user: job.user,
            event_id: job.event_id,
        }
    }
}

impl From<SQLJob> for Job {
    fn from(sql_job: SQLJob) -> Self {
        Job {
            id: sql_job.id,
            timestamp: sql_job.timestamp.into(),
            jid: sql_job.jid,
            user: sql_job.user,
            event_id: sql_job.event_id,
        }
    }
}

#[derive(
    Clone, Debug, Identifiable, Insertable, PartialEq, Eq, Queryable, AsChangeset, Associations,
)]
#[diesel(belongs_to(SQLJob, foreign_key = job_id))]
#[diesel(belongs_to(SQLEvent, foreign_key = event_id))]
#[diesel(belongs_to(SQLMinion, foreign_key = minion_id))]
#[diesel(table_name = job_returns)]
pub struct SQLJobReturn {
    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub jid: String,
    pub job_id: String,
    pub event_id: String,
    pub minion_id: String,
}

impl From<JobReturn> for SQLJobReturn {
    fn from(job_return: JobReturn) -> Self {
        SQLJobReturn {
            id: job_return.id,
            timestamp: job_return.timestamp.into(),
            jid: job_return.jid,
            job_id: job_return.job_id,
            event_id: job_return.event_id,
            minion_id: job_return.minion_id,
        }
    }
}

impl From<SQLJobReturn> for JobReturn {
    fn from(sql_job_return: SQLJobReturn) -> Self {
        JobReturn {
            id: sql_job_return.id,
            timestamp: sql_job_return.timestamp.into(),
            jid: sql_job_return.jid,
            job_id: sql_job_return.job_id,
            event_id: sql_job_return.event_id,
            minion_id: sql_job_return.minion_id,
        }
    }
}

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Eq, Queryable, AsChangeset)]
#[diesel(table_name = minions)]
pub struct SQLMinion {
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

impl From<Minion> for SQLMinion {
    fn from(minion: Minion) -> Self {
        SQLMinion {
            id: minion.id,
            last_seen: minion.last_seen.into(),
            grains: minion.grains,
            pillars: minion.pillars,
            pkgs: minion.pkgs,
            last_updated_grains: minion.last_updated_grains.map(|rt| rt.into()),
            last_updated_pillars: minion.last_updated_pillars.map(|rt| rt.into()),
            last_updated_pkgs: minion.last_updated_pkgs.map(|rt| rt.into()),
            conformity: minion.conformity,
            conformity_success: minion.conformity_success,
            conformity_incorrect: minion.conformity_incorrect,
            conformity_error: minion.conformity_error,
            last_updated_conformity: minion.last_updated_conformity.map(|rt| rt.into()),
            os_type: minion.os_type,
        }
    }
}

impl From<SQLMinion> for Minion {
    fn from(sql_minion: SQLMinion) -> Self {
        Minion {
            id: sql_minion.id,
            last_seen: sql_minion.last_seen.into(),
            grains: sql_minion.grains,
            pillars: sql_minion.pillars,
            pkgs: sql_minion.pkgs,
            last_updated_grains: sql_minion.last_updated_grains.map(|rt| rt.into()),
            last_updated_pillars: sql_minion.last_updated_pillars.map(|rt| rt.into()),
            last_updated_pkgs: sql_minion.last_updated_pkgs.map(|rt| rt.into()),
            conformity: sql_minion.conformity,
            conformity_success: sql_minion.conformity_success,
            conformity_incorrect: sql_minion.conformity_incorrect,
            conformity_error: sql_minion.conformity_error,
            last_updated_conformity: sql_minion.last_updated_conformity.map(|rt| rt.into()),
            os_type: sql_minion.os_type,
        }
    }
}

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Eq, Queryable, AsChangeset)]
#[diesel(table_name = users)]
pub struct SQLUser {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub perms: String,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
}

impl From<User> for SQLUser {
    fn from(user: User) -> Self {
        SQLUser {
            id: user.id,
            username: user.username,
            password: user.password,
            perms: user.perms,
            last_login: user.last_login.map(|rt| rt.into()),
            email: user.email,
        }
    }
}

impl From<SQLUser> for User {
    fn from(sql_user: SQLUser) -> Self {
        User {
            id: sql_user.id,
            username: sql_user.username,
            password: sql_user.password,
            perms: sql_user.perms,
            last_login: sql_user.last_login.map(|rt| rt.into()),
            email: sql_user.email,
        }
    }
}

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Eq, Queryable, AsChangeset)]
#[diesel(table_name = permission_groups)]
pub struct SQLPermissionGroup {
    pub id: String,
    pub name: String,
    pub perms: String,
}

impl From<PermissionGroup> for SQLPermissionGroup {
    fn from(permission_group: PermissionGroup) -> Self {
        SQLPermissionGroup {
            id: permission_group.id,
            name: permission_group.name,
            perms: permission_group.perms,
        }
    }
}

impl From<SQLPermissionGroup> for PermissionGroup {
    fn from(sql_permission_group: SQLPermissionGroup) -> Self {
        PermissionGroup {
            id: sql_permission_group.id,
            name: sql_permission_group.name,
            perms: sql_permission_group.perms,
        }
    }
}

#[derive(
    Clone, Debug, Identifiable, Insertable, PartialEq, Eq, Queryable, AsChangeset, Associations,
)]
#[diesel(belongs_to(SQLUser, foreign_key = user_id))]
#[diesel(belongs_to(SQLPermissionGroup, foreign_key = group_id))]
#[diesel(table_name = permission_group_users)]
pub struct SQLPermissionGroupUser {
    pub id: String,
    pub group_id: String,
    pub user_id: String,
}

impl From<PermissionGroupUser> for SQLPermissionGroupUser {
    fn from(permission_group_user: PermissionGroupUser) -> Self {
        SQLPermissionGroupUser {
            id: permission_group_user.id,
            group_id: permission_group_user.group_id,
            user_id: permission_group_user.user_id,
        }
    }
}

impl From<SQLPermissionGroupUser> for PermissionGroupUser {
    fn from(sql_permission_group_user: SQLPermissionGroupUser) -> Self {
        PermissionGroupUser {
            id: sql_permission_group_user.id,
            group_id: sql_permission_group_user.group_id,
            user_id: sql_permission_group_user.user_id,
        }
    }
}

#[derive(Clone, Debug, Identifiable, Insertable, PartialEq, Eq, Queryable, AsChangeset)]
#[diesel(table_name = minion_presets)]
pub struct SQLMinionPreset {
    pub id: String,
    pub name: String,
    pub filter: String,
}

impl From<MinionPreset> for SQLMinionPreset {
    fn from(minion_preset: MinionPreset) -> Self {
        SQLMinionPreset {
            id: minion_preset.id,
            name: minion_preset.name,
            filter: minion_preset.filter,
        }
    }
}

impl From<SQLMinionPreset> for MinionPreset {
    fn from(sql_minion_preset: SQLMinionPreset) -> Self {
        MinionPreset {
            id: sql_minion_preset.id,
            name: sql_minion_preset.name,
            filter: sql_minion_preset.filter,
        }
    }
}
