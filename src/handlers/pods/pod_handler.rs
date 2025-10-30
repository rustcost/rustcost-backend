// use axum::{
//     extract::Query,
//     routing::get,
//     Json, Router,
// };
// use serde::{Deserialize, Serialize};
// use chrono::{Duration, NaiveDateTime};
//
// use crate::infra::repositories::pod_repository::{get_pods, get_namespaces, get_avg_cpu_today_pod, get_avg_mem_today_pod, get_avg_cpu_today_namespace, get_avg_mem_today_namespace, get_pod_metrics_between, get_namespace_metrics_between, get_namespace_metrics_between_bucketed, get_pod_metrics_between_by_id_bucketed, PodMetricBucket, get_pods_by_namespace};
// use crate::domain::models::pod::{Pod, PodMetric, PodMetricDto};
// use crate::AppState;
//
// /// Standard API response wrapper
// #[derive(Serialize)]
// struct ApiResponse<T> {
//     success: bool,
//     data: Option<T>,
//     error: Option<String>,
// }
//
// /// GET /pod
// async fn list_pods() -> Json<ApiResponse<Vec<Pod>>> {
//     match get_pods() {
//         Ok(pods) => Json(ApiResponse { success: true, data: Some(pods), error: None }),
//         Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
//     }
// }
//
// /// GET /pod/namespaces
// async fn list_namespaces() -> Json<ApiResponse<Vec<String>>> {
//     match get_namespaces() {
//         Ok(namespaces) => Json(ApiResponse { success: true, data: Some(namespaces), error: None }),
//         Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
//     }
// }
//
// /// Query for pod averages
// #[derive(Deserialize)]
// struct PodQuery {
//     pod_id: i32,
// }
//
// /// GET /pod/avg?pod_id=123
// async fn avg_today_pod(Query(q): Query<PodQuery>) -> Json<ApiResponse<(Option<f64>, Option<f64>)>> {
//     let cpu = get_avg_cpu_today_pod(q.pod_id).unwrap_or(None);
//     let mem = get_avg_mem_today_pod(q.pod_id).unwrap_or(None);
//
//     Json(ApiResponse {
//         success: true,
//         data: Some((cpu, mem)),
//         error: None,
//     })
// }
//
// /// Query for namespace averages
// #[derive(Deserialize)]
// struct NamespaceQuery {
//     ns: String,
// }
//
// /// GET /pod/avg_ns?ns=default
// async fn avg_today_namespace(Query(q): Query<NamespaceQuery>) -> Json<ApiResponse<(Option<f64>, Option<f64>)>> {
//     let cpu = get_avg_cpu_today_namespace(&q.ns).unwrap_or(None);
//     let mem = get_avg_mem_today_namespace(&q.ns).unwrap_or(None);
//
//     Json(ApiResponse {
//         success: true,
//         data: Some((cpu, mem)),
//         error: None,
//     })
// }
//
// /// Query for metrics between times
// #[derive(Deserialize)]
// struct MetricsQuery {
//     start: NaiveDateTime,
//     end: NaiveDateTime,
// }
// /// Determine zoom level (bucket granularity) based on range
// pub fn zoom_bucket(start: NaiveDateTime, end: NaiveDateTime) -> Option<&'fixed str> {
//     let diff = end - start;
//     if diff <= chrono::Duration::days(1) {
//         Some("minute") // 1일 이하 → 분 단위
//     } else if diff <= chrono::Duration::days(7) {
//         Some("hour")   // 7일 이하 → 시 단위
//     } else if diff <= chrono::Duration::days(30) {
//         Some("day")    // 30일 이하 → 일 단위
//     } else {
//         None           // 30일 초과는 허용 안 함
//     }
// }
//
// pub async fn pod_metrics(
//     Query(q): Query<PodMetricsQuery>,
// ) -> Json<ApiResponse<Vec<PodMetricDto>>> {
//     match zoom_bucket(q.start, q.end) {
//         Some(bucket) => {
//             // pod_id 없는 경우: 무조건 raw metrics (단, max 1 day)
//             if q.pod_id == 0 {
//                 if bucket != "minute" {
//                     return Json(ApiResponse {
//                         success: false,
//                         data: None,
//                         error: Some("Range too large: max 1 day allowed when pod_id is not specified".into()),
//                     });
//                 }
//
//                 return match get_pod_metrics_between(q.start, q.end) {
//                     Ok(metrics) => Json(ApiResponse {
//                         success: true,
//                         data: Some(
//                             metrics
//                                 .into_iter()
//                                 .map(|m| PodMetricDto {
//                                     pod_id: m.pod_id.unwrap_or_default(),
//                                     bucket: m.timestamp,
//                                     avg_cpu: m.cpu_mcores as f64,
//                                     avg_mem: m.memory_bytes as f64,
//                                 })
//                                 .collect(),
//                         ),
//                         error: None,
//                     }),
//                     Err(e) => Json(ApiResponse {
//                         success: false,
//                         data: None,
//                         error: Some(e.to_string()),
//                     }),
//                 };
//             }
//
//             // pod_id 있는 경우
//             if bucket == "minute" {
//                 // 1일 이하 → raw metrics
//                 match get_pod_metrics_between(q.start, q.end) {
//                     Ok(metrics) => Json(ApiResponse {
//                         success: true,
//                         data: Some(
//                             metrics
//                                 .into_iter()
//                                 .filter(|m| m.pod_id == Some(q.pod_id))
//                                 .map(|m| PodMetricDto {
//                                     pod_id: q.pod_id,
//                                     bucket: m.timestamp,
//                                     avg_cpu: m.cpu_mcores as f64,
//                                     avg_mem: m.memory_bytes as f64,
//                                 })
//                                 .collect(),
//                         ),
//                         error: None,
//                     }),
//                     Err(e) => Json(ApiResponse {
//                         success: false,
//                         data: None,
//                         error: Some(e.to_string()),
//                     }),
//                 }
//             } else {
//                 // 1일 초과 → bucketed
//                 match get_pod_metrics_between_by_id_bucketed(q.pod_id, q.start, q.end, bucket) {
//                     Ok(metrics) => Json(ApiResponse {
//                         success: true,
//                         data: Some(
//                             metrics
//                                 .into_iter()
//                                 .map(|m| PodMetricDto {
//                                     pod_id: q.pod_id,
//                                     bucket: m.bucket,
//                                     avg_cpu: m.avg_cpu,
//                                     avg_mem: m.avg_mem,
//                                 })
//                                 .collect(),
//                         ),
//                         error: None,
//                     }),
//                     Err(e) => Json(ApiResponse {
//                         success: false,
//                         data: None,
//                         error: Some(e.to_string()),
//                     }),
//                 }
//             }
//         }
//         None => Json(ApiResponse {
//             success: false,
//             data: None,
//             error: Some("Range too large: queries above 30 days are not allowed".into()),
//         }),
//     }
// }
//
//
// /// GET /pod/metrics_ns?ns=default&start=...&end=...
// #[derive(Deserialize)]
// pub struct NsMetricsQuery {
//     pub ns: String,
//     pub start: NaiveDateTime,
//     pub end: NaiveDateTime,
// }
//
// /// Determine zoom level (bucket granularity)
//
// #[derive(Serialize)]
// pub struct NamespaceMetric {
//     pub namespace: String,
//     pub bucket: NaiveDateTime,
//     pub avg_cpu: f64,
//     pub avg_mem: f64,
// }
//
//
// pub async fn namespace_metrics_between(
//     Query(q): Query<NsMetricsQuery>,
// ) -> Json<ApiResponse<Vec<NamespaceMetric>>> {
//     match zoom_bucket(q.start, q.end) {
//         Some(bucket) => {
//             if bucket == "minute" {
//                 // 1일 이하 → raw metrics
//                 match get_namespace_metrics_between(&q.ns, q.start, q.end) {
//                     Ok(metrics) => {
//                         let result: Vec<NamespaceMetric> = metrics
//                             .into_iter()
//                             .map(|m| NamespaceMetric {
//                                 namespace: q.ns.clone(),
//                                 bucket: m.bucket,
//                                 avg_cpu: m.avg_cpu ,
//                                 avg_mem: m.avg_mem ,
//                             })
//                             .collect();
//
//                         Json(ApiResponse {
//                             success: true,
//                             data: Some(result),
//                             error: None,
//                         })
//                     }
//                     Err(e) => Json(ApiResponse {
//                         success: false,
//                         data: None,
//                         error: Some(e.to_string()),
//                     }),
//                 }
//             } else {
//                 // 1일 초과 → bucketed
//                 match get_namespace_metrics_between_bucketed(&q.ns, q.start, q.end, bucket) {
//                     Ok(metrics) => {
//                         let result: Vec<NamespaceMetric> = metrics
//                             .into_iter()
//                             .map(|m| NamespaceMetric {
//                                 namespace: q.ns.clone(),
//                                 bucket: m.bucket,
//                                 avg_cpu: m.avg_cpu,
//                                 avg_mem: m.avg_mem,
//                             })
//                             .collect();
//
//                         Json(ApiResponse {
//                             success: true,
//                             data: Some(result),
//                             error: None,
//                         })
//                     }
//                     Err(e) => Json(ApiResponse {
//                         success: false,
//                         data: None,
//                         error: Some(e.to_string()),
//                     }),
//                 }
//             }
//         }
//         None => Json(ApiResponse {
//             success: false,
//             data: None,
//             error: Some("Range too large: queries above 30 days are not allowed".into()),
//         }),
//     }
// }
//
// #[derive(Deserialize)]
// pub struct PodMetricsQuery {
//     pub pod_id: i32,
//     pub start: NaiveDateTime,
//     pub end: NaiveDateTime,
// }
//
// #[derive(Deserialize)]
// pub struct PodsByNamespaceQuery {
//     pub ns: String,
// }
//
// async fn list_pods_by_namespace(
//     Query(q): Query<PodsByNamespaceQuery>,
// ) -> Json<ApiResponse<Vec<Pod>>> {
//     match get_pods_by_namespace(&q.ns) {
//         Ok(pods) => Json(ApiResponse {
//             success: true,
//             data: Some(pods),
//             error: None,
//         }),
//         Err(e) => Json(ApiResponse {
//             success: false,
//             data: None,
//             error: Some(e.to_string()),
//         }),
//     }
// }
//
// /// Register pod routes
// pub fn pod_routes(state: AppState) -> Router<AppState> {
//     Router::new()
//         .route("/pod", get(list_pods))
//         .route("/pod/namespaces", get(list_namespaces))
//         .route("/pod/by_ns", get(list_pods_by_namespace)) // ✅ 추가
//         .route("/pod/avg", get(avg_today_pod))
//         .route("/pod/avg_ns", get(avg_today_namespace))
//         .route("/pod/metrics", get(pod_metrics))
//         .route("/pod/metrics_ns", get(namespace_metrics_between))
//         .with_state(state)
// }
//
