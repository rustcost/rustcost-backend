// @generated automatically by Diesel CLI.

diesel::table! {
    nodes (id) {
        id -> Int4,
        name -> Varchar,
        cpu_capacity -> Nullable<Varchar>,
        memory_capacity -> Nullable<Varchar>,
        kubelet_version -> Nullable<Varchar>,
        os_image -> Nullable<Varchar>,
        architecture -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    nodes,
    posts,
);
