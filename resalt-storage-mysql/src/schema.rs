// @generated automatically by Diesel CLI.

diesel::table! {
    authtokens (id) {
        id -> Varchar,
        user_id -> Varchar,
        timestamp -> Timestamp,
        salt_token -> Nullable<Text>,
    }
}

diesel::table! {
    events (id) {
        id -> Varchar,
        timestamp -> Timestamp,
        tag -> Varchar,
        data -> Mediumtext,
    }
}

diesel::table! {
    job_returns (id) {
        id -> Varchar,
        timestamp -> Timestamp,
        jid -> Varchar,
        job_id -> Varchar,
        event_id -> Varchar,
        minion_id -> Varchar,
    }
}

diesel::table! {
    jobs (id) {
        id -> Varchar,
        timestamp -> Timestamp,
        jid -> Varchar,
        user -> Nullable<Varchar>,
        event_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    minion_presets (id) {
        id -> Varchar,
        name -> Varchar,
        filter -> Varchar,
    }
}

diesel::table! {
    minions (id) {
        id -> Varchar,
        last_seen -> Timestamp,
        grains -> Nullable<Mediumtext>,
        pillars -> Nullable<Mediumtext>,
        pkgs -> Nullable<Mediumtext>,
        last_updated_grains -> Nullable<Timestamp>,
        last_updated_pillars -> Nullable<Timestamp>,
        last_updated_pkgs -> Nullable<Timestamp>,
        conformity -> Nullable<Longtext>,
        conformity_success -> Nullable<Integer>,
        conformity_incorrect -> Nullable<Integer>,
        conformity_error -> Nullable<Integer>,
        last_updated_conformity -> Nullable<Timestamp>,
        os_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    permission_group_users (id) {
        id -> Varchar,
        user_id -> Varchar,
        group_id -> Varchar,
    }
}

diesel::table! {
    permission_groups (id) {
        id -> Varchar,
        name -> Varchar,
        perms -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        password -> Nullable<Varchar>,
        perms -> Text,
        last_login -> Nullable<Timestamp>,
        email -> Nullable<Varchar>,
    }
}

diesel::joinable!(permission_group_users -> permission_groups (group_id));
diesel::joinable!(permission_group_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    authtokens,
    events,
    job_returns,
    jobs,
    minion_presets,
    minions,
    permission_group_users,
    permission_groups,
    users,
);
