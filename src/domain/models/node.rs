
/// =======================
/// Nodes
/// =======================

pub struct Node {
    pub node_id: i32,
    pub name: String,
    pub labels: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
}


pub struct NewNode {
    pub name: String,
    pub labels: Option<serde_json::Value>,
}

pub struct UpdateNode {
    pub labels: Option<serde_json::Value>,
}

pub struct NodeMetric {
    pub id: i64,
    pub node_id: Option<i32>,
    pub cpu_mcores: i64,
    pub memory_bytes: i64,
    pub timestamp: chrono::NaiveDateTime,
}

pub struct NewNodeMetric {
    pub node_id: Option<i32>,
    pub cpu_mcores: i64,
    pub memory_bytes: i64,
    pub timestamp: chrono::NaiveDateTime,
}
