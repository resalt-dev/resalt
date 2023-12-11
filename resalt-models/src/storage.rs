#[derive(Clone, Debug, PartialEq, Eq)]
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