joinable!(job_returns -> events (event_id));
allow_tables_to_appear_in_same_query!(job_returns, events);

table! {
    authtokens (id) {
        id -> Varchar,
        user_id -> Varchar,
        timestamp -> Timestamp,
        salt_token -> Nullable<Text>,
    }
}

table! {
    events (id) {
        id -> Varchar,
        timestamp -> Timestamp,
        tag -> Varchar,
        data -> Mediumtext,
    }
}

table! {
    jobs (id) {
        id -> Varchar,
        timestamp -> Timestamp,
        jid -> Varchar,
        user -> Varchar,
        minions -> Mediumtext,
        event_id -> Varchar,
    }
}

table! {
    job_returns (id) {
        id -> Varchar,
        timestamp -> Timestamp,
        jid -> Varchar,
        job_id -> Varchar,
        event_id -> Varchar,
    }
}

table! {
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
    }
}

table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        password -> Nullable<Varchar>,
    }
}
