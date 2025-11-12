use serde_json::{json, Value};
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::api::dto::metrics_dto::RangeQuery;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::core::persistence::metrics::k8s::node::day::metric_node_day_api_repository_trait::MetricNodeDayApiRepository;
use crate::core::persistence::metrics::k8s::node::hour::metric_node_hour_api_repository_trait::MetricNodeHourApiRepository;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_api_repository_trait::MetricNodeMinuteApiRepository;
use crate::domain::metric::k8s::common::dto::{CommonMetricValuesDto, FilesystemMetricDto, MetricGetResponseDto, MetricScope, MetricSeriesDto, NetworkMetricDto, UniversalMetricPointDto};
use crate::domain::metric::k8s::common::util::k8s_metric_determine_granularity::determine_granularity;
use crate::domain::metric::k8s::common::util::k8s_metric_repository_resolve::resolve_k8s_metric_repository;
use crate::domain::metric::k8s::common::util::k8s_metric_repository_variant::K8sMetricRepositoryVariant;

pub async fn get_metric_k8s_cluster_raw(
    node_info_list: Vec<InfoNodeEntity>,
    q: RangeQuery,
) -> Result<Value, anyhow::Error> {
    let start = q.start
        .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        .unwrap_or_else(|| Utc::now() - chrono::Duration::hours(1));

    let end = q.end
        .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        .unwrap_or_else(Utc::now);

    let granularity = determine_granularity(start, end);
    let repo = resolve_k8s_metric_repository(&MetricScope::Node, &granularity);

    let mut aggregated_points: Vec<UniversalMetricPointDto> = vec![];

    for node_info in node_info_list.iter() {
        let node_name = match &node_info.node_name {
            Some(name) => name.clone(),
            None => continue,
        };

        let metrics = match &repo {
            K8sMetricRepositoryVariant::NodeMinute(r) => r.get_row_between(&node_name, start, end),
            K8sMetricRepositoryVariant::NodeHour(r) => r.get_row_between(&node_name, start, end),
            K8sMetricRepositoryVariant::NodeDay(r) => r.get_row_between(&node_name, start, end),
            _ => Ok(vec![]), // ✅ make sure all branches return the same type
        }
            .unwrap_or_else(|_| vec![]);

        for m in metrics {
            let point = UniversalMetricPointDto {
                time: m.time,
                cpu_memory: CommonMetricValuesDto {
                    cpu_usage_nano_cores: m.cpu_usage_nano_cores.map(|v| v as f64),
                    memory_usage_bytes: m.memory_usage_bytes.map(|v| v as f64),
                    ..Default::default()
                },
                filesystem: Some(FilesystemMetricDto {
                    used_bytes: m.fs_used_bytes.map(|v| v as f64),
                    capacity_bytes: m.fs_capacity_bytes.map(|v| v as f64),
                    inodes_used: m.fs_inodes_used.map(|v| v as f64),
                    inodes: m.fs_inodes.map(|v| v as f64),
                    ..Default::default()
                }),
                network: Some(NetworkMetricDto {
                    rx_bytes: m.network_physical_rx_bytes.map(|v| v as f64),
                    tx_bytes: m.network_physical_tx_bytes.map(|v| v as f64),
                    rx_errors: m.network_physical_rx_errors.map(|v| v as f64),
                    tx_errors: m.network_physical_tx_errors.map(|v| v as f64),
                    ..Default::default()
                }),
                ..Default::default()
            };

            aggregated_points.push(point);
        }
    }

    // Optional: group or average by timestamp to aggregate across nodes
    let cluster_series = MetricSeriesDto {
        name: "cluster".to_string(),
        scope: MetricScope::Cluster,
        points: aggregate_cluster_points(aggregated_points),
    };

    let response = MetricGetResponseDto {
        start,
        end,
        scope: "cluster".to_string(),
        target: None,
        granularity,
        series: vec![cluster_series],
    };

    Ok(serde_json::to_value(response)?)
}

pub async fn get_metric_k8s_cluster_summary(q: RangeQuery) -> Result<Value> {
    Ok(json!({ "status": "ok", "message": "cluster_cost not implemented yet", "query": q }))
}

pub async fn get_metric_k8s_cluster_cost(q: RangeQuery) -> Result<Value> {
    Ok(json!({ "status": "ok", "message": "cluster_summary not implemented yet", "query": q }))
}

pub async fn get_metric_k8s_cluster_trend(q: RangeQuery) -> Result<Value> {
    Ok(json!({ "status": "ok", "message": "cluster_trends not implemented yet", "query": q }))
}

pub async fn get_metric_k8s_cluster_efficiency(q: RangeQuery) -> Result<Value> {
    Ok(json!({ "status": "ok", "message": "cluster_efficiency not implemented yet", "query": q }))
}

use std::collections::HashMap;

fn aggregate_cluster_points(points: Vec<UniversalMetricPointDto>) -> Vec<UniversalMetricPointDto> {
    let mut map: HashMap<i64, Vec<UniversalMetricPointDto>> = HashMap::new(); for p in points { let ts = p.time.timestamp(); map.entry(ts).or_default().push(p); } let mut aggregated: Vec<UniversalMetricPointDto> = Vec::new(); for (ts, pts) in map { let len = pts.len() as f64; if len == 0.0 { continue; } let mut cpu_usage = 0.0; let mut mem_usage = 0.0; for p in &pts { cpu_usage += p.cpu_memory.cpu_usage_nano_cores.unwrap_or(0.0); mem_usage += p.cpu_memory.memory_usage_bytes.unwrap_or(0.0); } aggregated.push(UniversalMetricPointDto { time: chrono::DateTime::<Utc>::from_timestamp(ts, 0).unwrap(), cpu_memory: CommonMetricValuesDto { cpu_usage_nano_cores: Some(cpu_usage / len), memory_usage_bytes: Some(mem_usage / len), ..Default::default() }, ..Default::default() }); } aggregated.sort_by_key(|p| p.time); aggregated }


