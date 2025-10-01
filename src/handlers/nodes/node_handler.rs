use axum::{
    extract::Query,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::infra::repositories::node_repository::{
    get_nodes, get_avg_cpu_today, get_avg_memory_today, get_metrics_between,
};
use crate::domain::models::node::{Node, NodeMetric};
use crate::AppState;

/// Standard API response
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// GET /node
async fn list_nodes() -> Json<ApiResponse<Vec<Node>>> {
    match get_nodes() {
        Ok(nodes) => Json(ApiResponse { success: true, data: Some(nodes), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

/// GET /node/avg
async fn avg_today() -> Json<ApiResponse<(Option<f64>, Option<f64>)>> {
    let avg_cpu = get_avg_cpu_today().unwrap_or(None);
    let avg_mem = get_avg_memory_today().unwrap_or(None);

    Json(ApiResponse {
        success: true,
        data: Some((avg_cpu, avg_mem)),
        error: None,
    })
}

/// Query params for metrics
#[derive(Deserialize)]
struct MetricsQuery {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

/// GET /node/metrics?start=...&end=...
async fn metrics_between(Query(q): Query<MetricsQuery>) -> Json<ApiResponse<Vec<NodeMetric>>> {
    match get_metrics_between(q.start, q.end) {
        Ok(metrics) => Json(ApiResponse { success: true, data: Some(metrics), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

/// Register node routes
pub fn node_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/node", get(list_nodes))
        .route("/node/avg", get(avg_today))
        .route("/node/metrics", get(metrics_between))
        .with_state(state)
}
