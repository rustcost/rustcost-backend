use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashSet;

use crate::api::dto::{info_dto::K8sListQuery, metrics_dto::RangeQuery};
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::core::persistence::metrics::k8s::pod::day::metric_pod_day_api_repository_trait::MetricPodDayApiRepository;
use crate::core::persistence::metrics::k8s::pod::hour::metric_pod_hour_api_repository_trait::MetricPodHourApiRepository;
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;
use crate::core::persistence::metrics::k8s::pod::minute::metric_pod_minute_api_repository_trait::MetricPodMinuteApiRepository;
use crate::domain::info::service::{
    info_k8s_container_service, info_k8s_pod_service, info_unit_price_service,
};
use crate::domain::metric::k8s::common::dto::{
    CommonMetricValuesDto, FilesystemMetricDto, MetricGetResponseDto, MetricScope, MetricSeriesDto,
    NetworkMetricDto, StorageMetricDto, UniversalMetricPointDto,
};
use crate::domain::metric::k8s::common::dto::metric_k8s_raw_summary_dto::MetricRawSummaryResponseDto;
use crate::domain::metric::k8s::common::service_helpers::{
    apply_costs, build_cost_summary_dto, build_cost_trend_dto, build_efficiency_value,
    build_raw_summary_value, resolve_time_window, TimeWindow, BYTES_PER_GB,
};
use crate::domain::metric::k8s::common::util::k8s_metric_repository_resolve::resolve_k8s_metric_repository;
use crate::domain::metric::k8s::common::util::k8s_metric_repository_variant::K8sMetricRepositoryVariant;

fn fetch_pod_points(
    repo: &K8sMetricRepositoryVariant,
    pod_uid: &str,
    window: &TimeWindow,
) -> Result<Vec<UniversalMetricPointDto>> {
    let rows = match repo {
        K8sMetricRepositoryVariant::PodMinute(r) => {
            r.get_row_between(window.start, window.end, pod_uid, None, None)
        }
        K8sMetricRepositoryVariant::PodHour(r) => r.get_row_between(pod_uid, window.start, window.end),
        K8sMetricRepositoryVariant::PodDay(r) => r.get_row_between(pod_uid, window.start, window.end),
        _ => Ok(vec![]),
    }?;

    Ok(rows.into_iter().map(metric_pod_entity_to_point).collect())
}

fn metric_pod_entity_to_point(entity: MetricPodEntity) -> UniversalMetricPointDto {
    let ephemeral_fs = FilesystemMetricDto {
        used_bytes: entity.es_used_bytes.map(|v| v as f64),
        capacity_bytes: entity.es_capacity_bytes.map(|v| v as f64),
        inodes_used: entity.es_inodes_used.map(|v| v as f64),
        inodes: entity.es_inodes.map(|v| v as f64),
    };

    let persistent_fs = FilesystemMetricDto {
        used_bytes: entity.pv_used_bytes.map(|v| v as f64),
        capacity_bytes: entity.pv_capacity_bytes.map(|v| v as f64),
        inodes_used: entity.pv_inodes_used.map(|v| v as f64),
        inodes: entity.pv_inodes.map(|v| v as f64),
    };

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
        filesystem: Some(ephemeral_fs.clone()),
        storage: Some(StorageMetricDto {
            ephemeral: Some(ephemeral_fs),
            persistent: Some(persistent_fs),
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

async fn build_pod_raw_data(
    q: RangeQuery,
    target_pod_uid: Option<String>,
) -> Result<(MetricGetResponseDto, Vec<InfoPodEntity>)> {
    let pod_infos = if let Some(pod_uid) = target_pod_uid.clone() {
        vec![info_k8s_pod_service::get_info_k8s_pod(pod_uid).await?]
    } else {
        info_k8s_pod_service::list_k8s_pods(K8sListQuery {
            namespace: q.namespace.clone(),
            label_selector: None,
            node_name: None,
        })
        .await?
    };

    let response = build_pod_series_for_infos(&q, &pod_infos, target_pod_uid.clone())?;
    Ok((response, pod_infos))
}

fn build_pod_series_for_infos(
    q: &RangeQuery,
    pod_infos: &[InfoPodEntity],
    target: Option<String>,
) -> Result<MetricGetResponseDto> {
    let window = resolve_time_window(q);
    let repo = resolve_k8s_metric_repository(&MetricScope::Pod, &window.granularity);

    let mut series = Vec::new();
    for pod in pod_infos.iter() {
        let pod_uid = pod
            .pod_uid
            .clone()
            .ok_or_else(|| anyhow!("Pod record missing UID"))?;

        let points = fetch_pod_points(&repo, &pod_uid, &window)?;
        let name = pod
            .pod_name
            .clone()
            .unwrap_or_else(|| pod_uid.clone());

        series.push(MetricSeriesDto {
            name,
            scope: MetricScope::Pod,
            points,
        });
    }

    Ok(MetricGetResponseDto {
        start: window.start,
        end: window.end,
        scope: "pod".to_string(),
        target,
        granularity: window.granularity,
        series,
    })
}

pub(crate) fn build_pod_response_from_infos(
    q: RangeQuery,
    pod_infos: Vec<InfoPodEntity>,
    target: Option<String>,
) -> Result<MetricGetResponseDto> {
    build_pod_series_for_infos(&q, &pod_infos, target)
}

fn collect_pod_uids(pods: &[InfoPodEntity]) -> Vec<String> {
    pods.iter()
        .filter_map(|p| p.pod_uid.clone())
        .collect::<Vec<_>>()
}

fn derive_namespace_hint(pods: &[InfoPodEntity]) -> Option<String> {
    let namespaces: HashSet<_> = pods
        .iter()
        .filter_map(|p| p.namespace.clone())
        .collect();

    if namespaces.len() == 1 {
        namespaces.into_iter().next()
    } else {
        None
    }
}

fn sum_container_requests(
    containers: &[InfoContainerEntity],
    target_pods: &HashSet<String>,
) -> (f64, f64) {
    let mut total_cpu = 0.0;
    let mut total_memory_gb = 0.0;

    for container in containers {
        if let Some(pod_uid) = &container.pod_uid {
            if target_pods.contains(pod_uid) {
                total_cpu += container.cpu_request_millicores.unwrap_or(0) as f64 / 1000.0;
                total_memory_gb += container.memory_request_bytes.unwrap_or(0) as f64 / BYTES_PER_GB;
            }
        }
    }

    (total_cpu, total_memory_gb)
}

async fn build_pod_cost_response(
    q: RangeQuery,
    target: Option<String>,
    unit_prices: InfoUnitPriceEntity,
) -> Result<MetricGetResponseDto> {
    let (mut response, _) = build_pod_raw_data(q, target).await?;
    apply_costs(&mut response, &unit_prices);
    Ok(response)
}

pub async fn get_metric_k8s_pods_raw(q: RangeQuery) -> Result<Value> {
    let (response, _) = build_pod_raw_data(q, None).await?;
    Ok(serde_json::to_value(response)?)
}

pub async fn get_metric_k8s_pods_raw_summary(q: RangeQuery) -> Result<Value> {
    let (response, pod_infos) = build_pod_raw_data(q, None).await?;
    build_raw_summary_value(&response, MetricScope::Pod, pod_infos.len())
}

pub async fn get_metric_k8s_pods_raw_efficiency(q: RangeQuery) -> Result<Value> {
    let (response, pod_infos) = build_pod_raw_data(q.clone(), None).await?;
    let summary_value = build_raw_summary_value(&response, MetricScope::Pod, pod_infos.len())?;
    let summary: MetricRawSummaryResponseDto = serde_json::from_value(summary_value)?;

    let pod_uids = collect_pod_uids(&pod_infos);
    if pod_uids.is_empty() {
        return Err(anyhow!("no pods available for efficiency calculation"));
    }

    let namespace_hint = q.namespace.or_else(|| derive_namespace_hint(&pod_infos));
    let containers = info_k8s_container_service::list_k8s_containers(K8sListQuery {
        namespace: namespace_hint,
        label_selector: None,
        node_name: None,
    })
    .await?;

    let target_set: HashSet<String> = pod_uids.into_iter().collect();
    let (total_cpu, total_mem_gb) = sum_container_requests(&containers, &target_set);
    let total_storage_gb = summary.summary.max_storage_gb;

    build_efficiency_value(
        summary,
        MetricScope::Pod,
        total_cpu,
        total_mem_gb,
        total_storage_gb,
    )
}

pub async fn get_metric_k8s_pod_raw(pod_uid: String, q: RangeQuery) -> Result<Value> {
    let (response, _) = build_pod_raw_data(q, Some(pod_uid)).await?;
    Ok(serde_json::to_value(response)?)
}

pub async fn get_metric_k8s_pod_raw_summary(pod_uid: String, q: RangeQuery) -> Result<Value> {
    let (response, _) = build_pod_raw_data(q, Some(pod_uid)).await?;
    build_raw_summary_value(&response, MetricScope::Pod, 1)
}

pub async fn get_metric_k8s_pod_raw_efficiency(pod_uid: String, q: RangeQuery) -> Result<Value> {
    let (response, pod_infos) = build_pod_raw_data(q.clone(), Some(pod_uid.clone())).await?;
    let summary_value = build_raw_summary_value(&response, MetricScope::Pod, 1)?;
    let summary: MetricRawSummaryResponseDto = serde_json::from_value(summary_value)?;

    let namespace_hint = pod_infos
        .first()
        .and_then(|p| p.namespace.clone())
        .or(q.namespace);

    let containers = info_k8s_container_service::list_k8s_containers(K8sListQuery {
        namespace: namespace_hint,
        label_selector: None,
        node_name: None,
    })
    .await?;

    let mut target = HashSet::new();
    target.insert(pod_uid);
    let (total_cpu, total_mem_gb) = sum_container_requests(&containers, &target);
    let total_storage_gb = summary.summary.max_storage_gb;

    build_efficiency_value(
        summary,
        MetricScope::Pod,
        total_cpu,
        total_mem_gb,
        total_storage_gb,
    )
}

pub async fn get_metric_k8s_pods_cost(q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_pod_cost_response(q, None, unit_prices).await?;
    Ok(serde_json::to_value(response)?)
}

pub async fn get_metric_k8s_pods_cost_summary(q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_pod_cost_response(q, None, unit_prices.clone()).await?;
    let dto = build_cost_summary_dto(&response, MetricScope::Pod, None, &unit_prices);
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_pods_cost_trend(q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_pod_cost_response(q, None, unit_prices).await?;
    let dto = build_cost_trend_dto(&response, MetricScope::Pod, None)?;
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_pod_cost(pod_uid: String, q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_pod_cost_response(q, Some(pod_uid.clone()), unit_prices).await?;
    Ok(serde_json::to_value(response)?)
}

pub async fn get_metric_k8s_pod_cost_summary(pod_uid: String, q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response =
        build_pod_cost_response(q, Some(pod_uid.clone()), unit_prices.clone()).await?;
    let dto = build_cost_summary_dto(&response, MetricScope::Pod, Some(pod_uid), &unit_prices);
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_pod_cost_trend(pod_uid: String, q: RangeQuery) -> Result<Value> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let response = build_pod_cost_response(q, Some(pod_uid.clone()), unit_prices).await?;
    let dto = build_cost_trend_dto(&response, MetricScope::Pod, Some(pod_uid))?;
    Ok(serde_json::to_value(dto)?)
}
