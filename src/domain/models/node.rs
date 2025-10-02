use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::infra::db::schema::{nodes, node_metrics};

/// =======================
/// Nodes
/// =======================
#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Node {
    pub node_id: i32,
    pub name: String,
    pub labels: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = nodes)]
pub struct NewNode {
    pub name: String,
    pub labels: Option<serde_json::Value>,
}

#[derive(AsChangeset)]
#[diesel(table_name = nodes)]
pub struct UpdateNode {
    pub labels: Option<serde_json::Value>,
}

/// =======================
/// Node Metrics
/// =======================
#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(belongs_to(Node))]
#[diesel(table_name = node_metrics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NodeMetric {
    pub id: i64,
    pub node_id: Option<i32>,
    pub cpu_mcores: i64,
    pub memory_bytes: i64,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = node_metrics)]
pub struct NewNodeMetric {
    pub node_id: Option<i32>,
    pub cpu_mcores: i64,
    pub memory_bytes: i64,
    pub timestamp: chrono::NaiveDateTime,
}
