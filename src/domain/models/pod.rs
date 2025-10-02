use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::infra::db::schema::{pods, pod_metrics};

/// =======================
/// Pods
/// =======================
#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(belongs_to(Node))]
#[diesel(table_name = pods)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Pod {
    pub pod_id: i32,
    pub name: String,
    pub namespace: String,
    pub node_id: Option<i32>,
    pub labels: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = pods)]
pub struct NewPod {
    pub name: String,
    pub namespace: String,
    pub node_id: Option<i32>,
    pub labels: Option<serde_json::Value>,
}

#[derive(AsChangeset)]
#[diesel(table_name = pods)]
pub struct UpdatePod {
    pub node_id: Option<i32>,
    pub labels: Option<serde_json::Value>,
}

/// =======================
/// Pod Metrics
/// =======================
#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(belongs_to(Pod))]
#[diesel(table_name = pod_metrics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PodMetric {
    pub id: i64,
    pub pod_id: Option<i32>,
    pub namespace: String,
    pub cpu_mcores: i64,
    pub memory_bytes: i64,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = pod_metrics)]
pub struct NewPodMetric {
    pub pod_id: Option<i32>,
    pub namespace: String,
    pub cpu_mcores: i64,
    pub memory_bytes: i64,
    pub timestamp: chrono::NaiveDateTime,
}
