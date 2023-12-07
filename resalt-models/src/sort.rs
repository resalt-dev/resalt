use serde::{Deserialize, Serialize};

use crate::{Job, Minion};

/*
minions.sort_by(|a, b| match sort {
        "id.asc" => a.id.cmp(&b.id),
        "id.desc" => b.id.cmp(&a.id),
        "lastSeen.asc" => a.last_seen.cmp(&b.last_seen),
        "lastSeen.desc" => b.last_seen.cmp(&a.last_seen),
        "conformitySuccess.asc" => a
            .conformity_success
            .unwrap_or_default()
            .cmp(&b.conformity_success.unwrap_or_default()),
        "conformitySuccess.desc" => b
            .conformity_success
            .unwrap_or_default()
            .cmp(&a.conformity_success.unwrap_or_default()),
        "conformityIncorrect.asc" => a
            .conformity_incorrect
            .unwrap_or_default()
            .cmp(&b.conformity_incorrect.unwrap_or_default()),
        "conformityIncorrect.desc" => b
            .conformity_incorrect
            .unwrap_or_default()
            .cmp(&a.conformity_incorrect.unwrap_or_default()),
        "conformityError.asc" => a
            .conformity_error
            .unwrap_or_default()
            .cmp(&b.conformity_error.unwrap_or_default()),
        "conformityError.desc" => b
            .conformity_error
            .unwrap_or_default()
            .cmp(&a.conformity_error.unwrap_or_default()),
        "osType.asc" => a
            .os_type
            .as_ref()
            .unwrap_or(&String::from(""))
            .cmp(b.os_type.as_ref().unwrap_or(&String::from(""))),
        "osType.desc" => b
            .os_type
            .as_ref()
            .unwrap_or(&String::from(""))
            .cmp(a.os_type.as_ref().unwrap_or(&String::from(""))),
        _ => std::cmp::Ordering::Equal,
    }) */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MinionSort {
    #[default]
    #[serde(rename = "id.asc")]
    IdAsc,
    #[serde(rename = "id.desc")]
    IdDesc,
    #[serde(rename = "lastSeen.asc")]
    LastSeenAsc,
    #[serde(rename = "lastSeen.desc")]
    LastSeenDesc,
    #[serde(rename = "conformitySuccess.asc")]
    ConformitySuccessAsc,
    #[serde(rename = "conformitySuccess.desc")]
    ConformitySuccessDesc,
    #[serde(rename = "conformityIncorrect.asc")]
    ConformityIncorrectAsc,
    #[serde(rename = "conformityIncorrect.desc")]
    ConformityIncorrectDesc,
    #[serde(rename = "conformityError.asc")]
    ConformityErrorAsc,
    #[serde(rename = "conformityError.desc")]
    ConformityErrorDesc,
    #[serde(rename = "osType.asc")]
    OsTypeAsc,
    #[serde(rename = "osType.desc")]
    OsTypeDesc,
}

pub fn sort_minions(minions: &mut [Minion], sort: &MinionSort) {
    minions.sort_by(|a, b| match sort {
        MinionSort::IdAsc => a.id.cmp(&b.id),
        MinionSort::IdDesc => b.id.cmp(&a.id),
        MinionSort::LastSeenAsc => a.last_seen.cmp(&b.last_seen),
        MinionSort::LastSeenDesc => b.last_seen.cmp(&a.last_seen),
        MinionSort::ConformitySuccessAsc => a
            .conformity_success
            .unwrap_or_default()
            .cmp(&b.conformity_success.unwrap_or_default()),
        MinionSort::ConformitySuccessDesc => b
            .conformity_success
            .unwrap_or_default()
            .cmp(&a.conformity_success.unwrap_or_default()),
        MinionSort::ConformityIncorrectAsc => a
            .conformity_incorrect
            .unwrap_or_default()
            .cmp(&b.conformity_incorrect.unwrap_or_default()),
        MinionSort::ConformityIncorrectDesc => b
            .conformity_incorrect
            .unwrap_or_default()
            .cmp(&a.conformity_incorrect.unwrap_or_default()),
        MinionSort::ConformityErrorAsc => a
            .conformity_error
            .unwrap_or_default()
            .cmp(&b.conformity_error.unwrap_or_default()),
        MinionSort::ConformityErrorDesc => b
            .conformity_error
            .unwrap_or_default()
            .cmp(&a.conformity_error.unwrap_or_default()),
        MinionSort::OsTypeAsc => a
            .os_type
            .as_ref()
            .unwrap_or(&String::from(""))
            .cmp(b.os_type.as_ref().unwrap_or(&String::from(""))),
        MinionSort::OsTypeDesc => b
            .os_type
            .as_ref()
            .unwrap_or(&String::from(""))
            .cmp(a.os_type.as_ref().unwrap_or(&String::from(""))),
    })
}

/*
"id.asc" => a.id.cmp(&b.id),
"id.desc" => b.id.cmp(&a.id),
"timestamp.asc" => a.timestamp.cmp(&b.timestamp),
"timestamp.desc" => b.timestamp.cmp(&a.timestamp),
"jid.asc" => a.jid.cmp(&b.jid),
"jid.desc" => b.jid.cmp(&a.jid),
"user.asc" => a.user.cmp(&b.user),
"user.desc" => b.user.cmp(&a.user),
_ => std::cmp::Ordering::Equal, */
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum JobSort {
    #[default]
    #[serde(rename = "id.asc")]
    IdAsc,
    #[serde(rename = "id.desc")]
    IdDesc,
    #[serde(rename = "timestamp.asc")]
    TimestampAsc,
    #[serde(rename = "timestamp.desc")]
    TimestampDesc,
    #[serde(rename = "jid.asc")]
    JidAsc,
    #[serde(rename = "jid.desc")]
    JidDesc,
    #[serde(rename = "user.asc")]
    UserAsc,
    #[serde(rename = "user.desc")]
    UserDesc,
}

pub fn sort_jobs(jobs: &mut [Job], sort: &JobSort) {
    jobs.sort_by(|a, b| match sort {
        JobSort::IdAsc => a.id.cmp(&b.id),
        JobSort::IdDesc => b.id.cmp(&a.id),
        JobSort::TimestampAsc => a.timestamp.cmp(&b.timestamp),
        JobSort::TimestampDesc => b.timestamp.cmp(&a.timestamp),
        JobSort::JidAsc => a.jid.cmp(&b.jid),
        JobSort::JidDesc => b.jid.cmp(&a.jid),
        JobSort::UserAsc => a.user.cmp(&b.user),
        JobSort::UserDesc => b.user.cmp(&a.user),
    })
}
