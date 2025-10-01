use axum::{
    extract::Query,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::infra::repositories::pod_repository::{
    get_pods,
    get_namespaces,
    get_avg_cpu_today_pod,
    get_avg_mem_today_pod,
    get_avg_cpu_today_namespace,
    get_avg_mem_today_namespace,
    get_pod_metrics_between,
    get_namespace_metrics_between,
};
use crate::domain::models::pod::{Pod, PodMetric};
use crate::AppState;

/// Standard API response wrapper
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// GET /pod
async fn list_pods() -> Json<ApiResponse<Vec<Pod>>> {
    match get_pods() {
        Ok(pods) => Json(ApiResponse { success: true, data: Some(pods), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

/// GET /pod/namespaces
async fn list_namespaces() -> Json<ApiResponse<Vec<String>>> {
    match get_namespaces() {
        Ok(namespaces) => Json(ApiResponse { success: true, data: Some(namespaces), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

/// Query for pod averages
#[derive(Deserialize)]
struct PodQuery {
    pod_id: i32,
}

/// GET /pod/avg?pod_id=123
async fn avg_today_pod(Query(q): Query<PodQuery>) -> Json<ApiResponse<(Option<f64>, Option<f64>)>> {
    let cpu = get_avg_cpu_today_pod(q.pod_id).unwrap_or(None);
    let mem = get_avg_mem_today_pod(q.pod_id).unwrap_or(None);

    Json(ApiResponse {
        success: true,
        data: Some((cpu, mem)),
        error: None,
    })
}

/// Query for namespace averages
#[derive(Deserialize)]
struct NamespaceQuery {
    ns: String,
}

/// GET /pod/avg_ns?ns=default
async fn avg_today_namespace(Query(q): Query<NamespaceQuery>) -> Json<ApiResponse<(Option<f64>, Option<f64>)>> {
    let cpu = get_avg_cpu_today_namespace(&q.ns).unwrap_or(None);
    let mem = get_avg_mem_today_namespace(&q.ns).unwrap_or(None);

    Json(ApiResponse {
        success: true,
        data: Some((cpu, mem)),
        error: None,
    })
}

/// Query for metrics between times
#[derive(Deserialize)]
struct MetricsQuery {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

/// GET /pod/metrics?start=...&end=...
async fn metrics_between(Query(q): Query<MetricsQuery>) -> Json<ApiResponse<Vec<PodMetric>>> {
    match get_pod_metrics_between(q.start, q.end) {
        Ok(metrics) => Json(ApiResponse { success: true, data: Some(metrics), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

/// GET /pod/metrics_ns?ns=default&start=...&end=...
#[derive(Deserialize)]
struct NsMetricsQuery {
    ns: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
}
async fn namespace_metrics_between(Query(q): Query<NsMetricsQuery>) -> Json<ApiResponse<Vec<PodMetric>>> {
    match get_namespace_metrics_between(&q.ns, q.start, q.end) {
        Ok(metrics) => Json(ApiResponse { success: true, data: Some(metrics), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

/// Register pod routes
pub fn pod_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/pod", get(list_pods))
        .route("/pod/namespaces", get(list_namespaces))
        .route("/pod/avg", get(avg_today_pod))
        .route("/pod/avg_ns", get(avg_today_namespace))
        .route("/pod/metrics", get(metrics_between))
        .route("/pod/metrics_ns", get(namespace_metrics_between))
        .with_state(state)
}
