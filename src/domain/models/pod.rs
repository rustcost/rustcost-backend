use chrono::NaiveDateTime;
/// =======================
/// Pods
/// =======================

pub struct Pod {
    pub pod_id: i32,
    pub name: String,
    pub namespace: String,
    pub node_id: Option<i32>,
    pub labels: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
}


pub struct NewPod {
    pub name: String,
    pub namespace: String,
    pub node_id: Option<i32>,
    pub labels: Option<serde_json::Value>,
}


pub struct UpdatePod {
    pub node_id: Option<i32>,
    pub labels: Option<serde_json::Value>,
}


pub struct PodMetric {
    pub id: i64,
    pub pod_id: Option<i32>,
    pub namespace: String,
    pub cpu_mcores: i64,
    pub memory_bytes: i64,
    pub timestamp: chrono::NaiveDateTime,
}

pub struct NewPodMetric {
    pub pod_id: Option<i32>,
    pub namespace: String,
    pub cpu_mcores: i64,
    pub memory_bytes: i64,
    pub timestamp: chrono::NaiveDateTime,
}
pub struct PodMetricDto {
    pub pod_id: i32,
    pub bucket: NaiveDateTime,
    pub avg_cpu: f64,
    pub avg_mem: f64,
}