use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::api::dto::metrics_dto::RangeQuery;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::core::persistence::metrics::k8s::node::day::metric_node_day_api_repository_trait::MetricNodeDayApiRepository;
use crate::core::persistence::metrics::k8s::node::hour::metric_node_hour_api_repository_trait::MetricNodeHourApiRepository;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_api_repository_trait::MetricNodeMinuteApiRepository;
use crate::domain::info::service::{info_k8s_node_service, info_unit_price_service};
use crate::domain::metric::k8s::common::dto::{
    CommonMetricValuesDto, FilesystemMetricDto, MetricGetResponseDto, MetricScope, MetricSeriesDto,
    NetworkMetricDto, UniversalMetricPointDto,
};
use crate::domain::metric::k8s::common::dto::metric_k8s_raw_summary_dto::MetricRawSummaryResponseDto;
use crate::domain::metric::k8s::common::service_helpers::{
    apply_costs, build_cost_summary_dto, build_cost_trend_dto, build_efficiency_value,
    build_raw_summary_value, resolve_time_window, TimeWindow, BYTES_PER_GB,
};
use crate::domain::metric::k8s::common::util::k8s_metric_repository_resolve::resolve_k8s_metric_repository;
use crate::domain::metric::k8s::common::util::k8s_metric_repository_variant::K8sMetricRepositoryVariant;

fn fetch_node_points(
    repo: &K8sMetricRepositoryVariant,
    node_name: &str,
    window: &TimeWindow,
) -> Result<Vec<UniversalMetricPointDto>> {
    let rows = match repo {
        K8sMetricRepositoryVariant::NodeMinute(r) => r.get_row_between(node_name, window.start, window.end),
        K8sMetricRepositoryVariant::NodeHour(r) => r.get_row_between(node_name, window.start, window.end),
        K8sMetricRepositoryVariant::NodeDay(r) => r.get_row_between(node_name, window.start, window.end),
        _ => Ok(vec![]),
    }?;

    Ok(rows.into_iter().map(metric_node_entity_to_point).collect())
}

fn metric_node_entity_to_point(entity: MetricNodeEntity) -> UniversalMetricPointDto {
    UniversalMetricPointDto {
        time: entity.time,
        cpu_memory: CommonMetricValuesDto {
            cpu_usage_nano_cores: entity.cpu_usage_nano_cores.map(|v| v as f64),
            cpu_usage_core_nano_seconds: entity.cpu_usage_core_nano_seconds.map(|v| v as f64),
            memory_usage_bytes: entity.memory_usage_bytes.map(|v| v as f64),
            memory_working_set_bytes: entity.memory_working_set_bytes.map(|v| v as f64),
            memory_rss_bytes: entity.memory_rss_bytes.map(|v| v as f64),
            memory_page_faults: entity.memory_page_faults.map(|v| v as f64),
        },
        filesystem: Some(FilesystemMetricDto {
            used_bytes: entity.fs_used_bytes.map(|v| v as f64),
            capacity_bytes: entity.fs_capacity_bytes.map(|v| v as f64),
            inodes_used: entity.fs_inodes_used.map(|v| v as f64),
            inodes: entity.fs_inodes.map(|v| v as f64),
        }),
        network: Some(NetworkMetricDto {
            rx_bytes: entity.network_physical_rx_bytes.map(|v| v as f64),
            tx_bytes: entity.network_physical_tx_bytes.map(|v| v as f64),
            rx_errors: entity.network_physical_rx_errors.map(|v| v as f64),
            tx_errors: entity.network_physical_tx_errors.map(|v| v as f64),
        }),
        ..Default::default()
    }
}

async fn build_node_raw_data(
    q: RangeQuery,
    target: Option<String>,
) -> Result<(MetricGetResponseDto, Vec<InfoNodeEntity>)> {
    let window = resolve_time_window(&q);
    let repo = resolve_k8s_metric_repository(&MetricScope::Node, &window.granularity);

    let node_infos = if let Some(node_name) = target.clone() {
        vec![info_k8s_node_service::get_info_k8s_node(node_name).await?]
    } else {
        info_k8s_node_service::list_k8s_nodes().await?
    };

    let mut series = Vec::new();
    for node in node_infos.iter() {
        let node_name = node
            .node_name
            .clone()
            .ok_or_else(|| anyhow!("Node record missing name"))?;

        let points = fetch_node_points(&repo, &node_name, &window)?;
        series.push(MetricSeriesDto {
            key: node_name.clone(),
            name: node_name.clone(),
            scope: MetricScope::Node,
            points,
        });
    }

    let response = MetricGetResponseDto {
        start: window.start,
        end: window.end,
        scope: "node".to_string(),
        target,
        granularity: window.granularity.clone(),
        series,
    };

    Ok((response, node_infos))
}

fn sum_node_allocations(nodes: &[InfoNodeEntity]) -> (f64, f64, f64) {
    let mut total_cpu = 0.0;
    let mut total_mem_bytes = 0.0;
    let mut total_storage_bytes = 0.0;

    for node in nodes {
        total_cpu += node.cpu_allocatable_cores.unwrap_or(0) as f64;
        total_mem_bytes += node.memory_allocatable_bytes.unwrap_or(0) as f64;
        total_storage_bytes += node.ephemeral_storage_allocatable_bytes.unwrap_or(0) as f64;
    }

    (
        total_cpu,
        total_mem_bytes / BYTES_PER_GB,
        total_storage_bytes / BYTES_PER_GB,
    )
}


pub async fn get_metric_k8s_nodes_raw(q: RangeQuery) -> Result<Value> {
    let (response, _) = build_node_raw_data(q, None).await?;
    Ok(serde_json::to_value(response)?)
}

pub async fn get_metric_k8s_nodes_raw_summary(q: RangeQuery) -> Result<Value> {
    let (response, node_infos) = build_node_raw_data(q, None).await?;
    build_raw_summary_value(&response, MetricScope::Node, node_infos.len())
}

pub async fn get_metric_k8s_nodes_raw_efficiency(q: RangeQuery) -> Result<Value> {
    let (summary_value, node_infos) = {
        let (response, infos) = build_node_raw_data(q.clone(), None).await?;
        let summary_json = build_raw_summary_value(&response, MetricScope::Node, infos.len())?;
        (summary_json, infos)
    };

    let summary: MetricRawSummaryResponseDto = serde_json::from_value(summary_value)?;
    let (total_cpu, total_mem, total_storage) = sum_node_allocations(&node_infos);
    build_efficiency_value(summary, MetricScope::Node, total_cpu, total_mem, total_storage)
}

pub async fn get_metric_k8s_node_raw(node_name: String, q: RangeQuery) -> Result<Value> {
    let (response, _) = build_node_raw_data(q, Some(node_name.clone())).await?;
    Ok(serde_json::to_value(response)?)
}

pub async fn get_metric_k8s_node_raw_summary(node_name: String, q: RangeQuery) -> Result<Value> {
    let (response, _) = build_node_raw_data(q, Some(node_name)).await?;
    build_raw_summary_value(&response, MetricScope::Node, 1)
}

pub async fn get_metric_k8s_node_raw_efficiency(node_name: String, q: RangeQuery) -> Result<Value> {
    let (response, node_infos) = build_node_raw_data(q.clone(), Some(node_name)).await?;
    let summary_value = build_raw_summary_value(&response, MetricScope::Node, 1)?;
    let summary: MetricRawSummaryResponseDto = serde_json::from_value(summary_value)?;
    let (total_cpu, total_mem, total_storage) = sum_node_allocations(&node_infos);
    build_efficiency_value(summary, MetricScope::Node, total_cpu, total_mem, total_storage)
}

async fn build_node_cost_response(
    q: RangeQuery,
    target: Option<String>,
    unit_prices: InfoUnitPriceEntity,
) -> Result<MetricGetResponseDto> {
    let (mut response, _) = build_node_raw_data(q, target).await?;
    apply_costs(&mut response, &unit_prices);
    Ok(response)
}

pub async fn get_metric_k8s_nodes_cost(q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_node_cost_response(q, None, unit_prices).await?;
    Ok(serde_json::to_value(response)?)
}

pub async fn get_metric_k8s_nodes_cost_summary(q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_node_cost_response(q, None, unit_prices.clone()).await?;
    let dto = build_cost_summary_dto(&response, MetricScope::Node, None, &unit_prices);
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_nodes_cost_trend(q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_node_cost_response(q, None, unit_prices).await?;
    let dto = build_cost_trend_dto(&response, MetricScope::Node, None)?;
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_node_cost(node_name: String, q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_node_cost_response(q, Some(node_name.clone()), unit_prices).await?;
    Ok(serde_json::to_value(response)?)
}

pub async fn get_metric_k8s_node_cost_summary(node_name: String, q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_node_cost_response(q, Some(node_name.clone()), unit_prices.clone()).await?;
    let dto = build_cost_summary_dto(&response, MetricScope::Node, Some(node_name), &unit_prices);
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_node_cost_trend(node_name: String, q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_node_cost_response(q, Some(node_name.clone()), unit_prices).await?;
    let dto = build_cost_trend_dto(&response, MetricScope::Node, Some(node_name))?;
    Ok(serde_json::to_value(dto)?)
}
