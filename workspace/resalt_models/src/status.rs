use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemStatus {
    pub salt: bool,
    pub db: bool,
    #[serde(rename = "dbAuthTokensTotal")]
    pub db_auth_tokens_total: Option<i64>,
    #[serde(rename = "dbAuthTokensActive")]
    pub db_auth_tokens_active: Option<i64>,
    #[serde(rename = "dbEventsTotal")]
    pub db_events_total: Option<i64>,
    #[serde(rename = "dbJobReturnsTotal")]
    pub db_job_returns_total: Option<i64>,
    #[serde(rename = "dbJobsTotal")]
    pub db_jobs_total: Option<i64>,
    #[serde(rename = "dbMinionsTotal")]
    pub db_minions_total: Option<i64>,
    #[serde(rename = "dbPermissionGroupUsersTotal")]
    pub db_permission_group_users_total: Option<i64>,
    #[serde(rename = "dbPermissionGroupsTotal")]
    pub db_permission_groups_total: Option<i64>,
    #[serde(rename = "dbUsersTotal")]
    pub db_users_total: Option<i64>,
}
