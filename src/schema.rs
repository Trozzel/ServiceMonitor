// @generated automatically by Diesel CLI.

diesel::table! {
    group_accts (id) {
        id -> Bigint,
        parent_id -> Nullable<Bigint>,
        name -> Text,
        created -> Datetime,
        modified -> Datetime,
    }
}

diesel::table! {
    service_status (id) {
        id -> Bigint,
        hostname -> Text,
        name -> Text,
        description -> Nullable<Text>,
        enabled -> Nullable<Bool>,
        active_status -> Nullable<Text>,
        last_check -> Nullable<Datetime>,
    }
}

diesel::table! {
    users (id) {
        id -> Bigint,
        name -> Text,
        password -> Text,
        group_accts_id -> Nullable<Bigint>,
        active -> Bool,
        created -> Datetime,
        modified -> Datetime,
    }
}

diesel::joinable!(users -> group_accts (group_accts_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_accts,
    service_status,
    users,
);
