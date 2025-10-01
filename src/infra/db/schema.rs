// @generated automatically by Diesel CLI.

diesel::table! {
    node_metrics (id) {
        id -> Int8,
        node_id -> Nullable<Int4>,
        cpu_mcores -> Int8,
        memory_bytes -> Int8,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    nodes (node_id) {
        node_id -> Int4,
        name -> Text,
        labels -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    pod_metrics (id) {
        id -> Int8,
        pod_id -> Nullable<Int4>,
        namespace -> Text,
        cpu_mcores -> Int8,
        memory_bytes -> Int8,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    pods (pod_id) {
        pod_id -> Int4,
        name -> Text,
        namespace -> Text,
        node_id -> Nullable<Int4>,
        labels -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(node_metrics -> nodes (node_id));
diesel::joinable!(pod_metrics -> pods (pod_id));
diesel::joinable!(pods -> nodes (node_id));

diesel::allow_tables_to_appear_in_same_query!(
    node_metrics,
    nodes,
    pod_metrics,
    pods,
);
