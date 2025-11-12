use serde_json::{json, Value};
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::api::dto::metrics_dto::RangeQuery;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::core::persistence::metrics::k8s::node::day::metric_node_day_api_repository_trait::MetricNodeDayApiRepository;
use crate::domain::metric::k8s::node::repository::metric_node_day_api_repository::MetricNodeDayApiRepositoryImpl;
use serde::{Deserialize, Serialize};
use crate::domain::metric::k8s::cluster::dto::cluster_response_dto::{ClusterMetricDto, NodeMetricDto};

pub async fn cluster_get(
    node_info_list: Vec<InfoNodeEntity>,
    q: RangeQuery,
) -> Result<Value> {
    let repo_metric = MetricNodeDayApiRepositoryImpl::default();

    let start = q.start
        .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        .unwrap_or_else(|| Utc::now() - chrono::Duration::hours(1));

    let end = q.end
        .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        .unwrap_or_else(Utc::now);

    let mut cluster = ClusterMetricDto {
        start,
        end,
        total_nodes: node_info_list.len(),
        ..Default::default()
    };

    for node_info in node_info_list {
        let node_name = match &node_info.node_name { Some(name) => name, None => continue };

        let metrics = match repo_metric.get_row_between(node_name, start, end) {
            Ok(data) => data,
            Err(e) => { tracing::warn!("Failed to get metrics for node {}: {}", node_name, e); vec![] }
        };

        if metrics.is_empty() { continue; }

        let avg_cpu = avg(metrics.iter().filter_map(|m| m.cpu_usage_nano_cores));
        let avg_mem = avg(metrics.iter().filter_map(|m| m.memory_usage_bytes));
        let avg_fs = avg(metrics.iter().filter_map(|m| m.fs_used_bytes));

        let total_rx: u64 = metrics.iter().filter_map(|m| m.network_physical_rx_bytes).sum();
        let total_tx: u64 = metrics.iter().filter_map(|m| m.network_physical_tx_bytes).sum();

        cluster.total_cpu_usage_nano_cores += avg_cpu.unwrap_or(0.0) as u64;
        cluster.total_memory_usage_bytes += avg_mem.unwrap_or(0.0) as u64;
        cluster.total_fs_used_bytes += avg_fs.unwrap_or(0.0) as u64;
        cluster.total_network_rx_bytes += total_rx;
        cluster.total_network_tx_bytes += total_tx;

        cluster.nodes.push(NodeMetricDto {
            node_name: node_name.clone(),
            info: node_info,
            avg_cpu_usage_nano_cores: avg_cpu,
            avg_memory_usage_bytes: avg_mem,
            avg_fs_used_bytes: avg_fs,
            total_network_rx_bytes: Some(total_rx),
            total_network_tx_bytes: Some(total_tx),
        });
    }

    Ok(serde_json::to_value(cluster)?)
}

fn avg<I>(values: I) -> Option<f64>
where I: Iterator<Item = u64> {
    let mut count = 0;
    let mut sum: f64 = 0.0;
    for v in values { sum += v as f64; count += 1; }
    if count == 0 { None } else { Some(sum / count as f64) }
}

pub async fn cluster_cost(q: RangeQuery) -> Result<Value> {
    Ok(json!({ "status": "ok", "message": "cluster_cost not implemented yet", "query": q }))
}

pub async fn cluster_summary(q: RangeQuery) -> Result<Value> {
    Ok(json!({ "status": "ok", "message": "cluster_summary not implemented yet", "query": q }))
}

pub async fn cluster_trends(q: RangeQuery) -> Result<Value> {
    Ok(json!({ "status": "ok", "message": "cluster_trends not implemented yet", "query": q }))
}

pub async fn cluster_efficiency(q: RangeQuery) -> Result<Value> {
    Ok(json!({ "status": "ok", "message": "cluster_efficiency not implemented yet", "query": q }))
}
