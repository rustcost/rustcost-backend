use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::api::dto::{info_dto::K8sListQuery, metrics_dto::RangeQuery};
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::domain::info::service::{info_k8s_pod_service, info_unit_price_service};
use crate::domain::metric::k8s::common::dto::{MetricGetResponseDto, MetricScope, MetricSeriesDto};
use crate::domain::metric::k8s::common::service_helpers::{aggregate_cost_points, aggregate_points, apply_costs, build_cost_summary_dto, build_cost_trend_dto, build_raw_summary_value};
use crate::domain::metric::k8s::pod::service::build_pod_response_from_infos;

fn group_pods_by_namespace(pods: Vec<InfoPodEntity>) -> HashMap<String, Vec<InfoPodEntity>> {
    let mut map: HashMap<String, Vec<InfoPodEntity>> = HashMap::new();
    for pod in pods {
        if let Some(ns) = &pod.namespace {
            map.entry(ns.clone()).or_default().push(pod);
        }
    }
    map
}

fn build_namespace_response(
    namespace: &str,
    per_pod_response: &MetricGetResponseDto,
) -> MetricGetResponseDto {
    let aggregated_points =
        aggregate_points(per_pod_response.series.iter().flat_map(|s| s.points.clone()).collect());

    MetricGetResponseDto {
        start: per_pod_response.start,
        end: per_pod_response.end,
        scope: "namespace".to_string(),
        target: Some(namespace.to_string()),
        granularity: per_pod_response.granularity.clone(),
        series: vec![MetricSeriesDto {
            name: namespace.to_string(),
            scope: MetricScope::Namespace,
            points: aggregated_points,
        }],
    }
}

async fn build_namespace_cost_response(
    namespace: &str,
    mut per_pod_response: MetricGetResponseDto,
) -> Result<MetricGetResponseDto> {
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    apply_costs(&mut per_pod_response, &unit_prices);
    let points = aggregate_cost_points(&per_pod_response.series);

    Ok(MetricGetResponseDto {
        start: per_pod_response.start,
        end: per_pod_response.end,
        scope: "namespace".to_string(),
        target: Some(namespace.to_string()),
        granularity: per_pod_response.granularity,
        series: vec![MetricSeriesDto {
            name: namespace.to_string(),
            scope: MetricScope::Namespace,
            points,
        }],
    })
}

async fn namespace_pods(namespace: &str) -> Result<Vec<InfoPodEntity>> {
    let pods = info_k8s_pod_service::list_k8s_pods(K8sListQuery {
        namespace: Some(namespace.to_string()),
        label_selector: None,
        node_name: None,
    })
    .await?;

    if pods.is_empty() {
        return Err(anyhow!("namespace '{}' has no pods", namespace));
    }

    Ok(pods)
}

pub async fn get_metric_k8s_namespaces_raw(q: RangeQuery) -> Result<Value> {
    let all_pods = info_k8s_pod_service::list_k8s_pods(K8sListQuery {
        namespace: None,
        label_selector: None,
        node_name: None,
    })
    .await?;

    let ns_map = group_pods_by_namespace(all_pods);
    if ns_map.is_empty() {
        return Ok(json!({ "status": "no data" }));
    }

    let mut series = Vec::new();
    let mut base_response: Option<MetricGetResponseDto> = None;

    for (ns, pods) in ns_map {
        let per_pod = build_pod_response_from_infos(q.clone(), pods, Some(ns.clone()))?;
        let aggregated = build_namespace_response(&ns, &per_pod);
        if base_response.is_none() {
            base_response = Some(aggregated.clone());
        }
        series.push(aggregated.series[0].clone());
    }

    if let Some(mut base) = base_response {
        base.target = None;
        base.series = series;
        return Ok(serde_json::to_value(base)?);
    }

    Ok(json!({ "status": "no data" }))
}

pub async fn get_metric_k8s_namespace_raw(namespace: String, q: RangeQuery) -> Result<Value> {
    let pods = namespace_pods(&namespace).await?;
    let per_pod = build_pod_response_from_infos(q, pods, Some(namespace.clone()))?;
    let aggregated = build_namespace_response(&namespace, &per_pod);
    Ok(serde_json::to_value(aggregated)?)
}

pub async fn get_metric_k8s_namespace_raw_summary(namespace: String, q: RangeQuery) -> Result<Value> {
    let pods = namespace_pods(&namespace).await?;
    let per_pod = build_pod_response_from_infos(q, pods.clone(), Some(namespace.clone()))?;
    let aggregated = build_namespace_response(&namespace, &per_pod);
    build_raw_summary_value(&aggregated, MetricScope::Namespace, pods.len())
}

pub async fn get_metric_k8s_namespaces_raw_summary(q: RangeQuery) -> Result<Value> {
    let all_pods = info_k8s_pod_service::list_k8s_pods(K8sListQuery {
        namespace: None,
        label_selector: None,
        node_name: None,
    })
    .await?;

    if all_pods.is_empty() {
        return Ok(json!({ "status": "no data" }));
    }

    let per_pod = build_pod_response_from_infos(q, all_pods.clone(), None)?;
    let aggregated = build_namespace_response("all", &per_pod);
    build_raw_summary_value(&aggregated, MetricScope::Namespace, all_pods.len())
}

pub async fn get_metric_k8s_namespace_raw_efficiency(namespace: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_supported",
        "message": "Namespace efficiency not supported yet"
    }))
}

pub async fn get_metric_k8s_namespaces_raw_efficiency(_q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_supported",
        "message": "Namespace efficiency not supported yet"
    }))
}

async fn build_namespace_cost(
    namespace: Option<String>,
    q: RangeQuery,
) -> Result<MetricGetResponseDto> {
    let pods = if let Some(ns) = namespace.clone() {
        namespace_pods(&ns).await?
    } else {
        info_k8s_pod_service::list_k8s_pods(K8sListQuery {
            namespace: None,
            label_selector: None,
            node_name: None,
        })
        .await?
    };

    if pods.is_empty() {
        return Err(anyhow!("no pods available for namespace cost calculation"));
    }

    let per_pod = build_pod_response_from_infos(q, pods, namespace.clone())?;
    Ok(build_namespace_response(namespace.as_deref().unwrap_or("all"), &per_pod))
}

pub async fn get_metric_k8s_namespaces_cost(q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(None, q).await?;
    Ok(serde_json::to_value(aggregated)?)
}

pub async fn get_metric_k8s_namespace_cost(namespace: String, q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(Some(namespace), q).await?;
    Ok(serde_json::to_value(aggregated)?)
}

pub async fn get_metric_k8s_namespaces_cost_summary(q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(None, q.clone()).await?;
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let mut cost_response = aggregated.clone();
    apply_costs(&mut cost_response, &unit_prices);
    let dto = build_cost_summary_dto(&cost_response, MetricScope::Namespace, None, &unit_prices);
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_namespace_cost_summary(namespace: String, q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(Some(namespace.clone()), q.clone()).await?;
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let mut cost_response = aggregated.clone();
    apply_costs(&mut cost_response, &unit_prices);
    let dto = build_cost_summary_dto(
        &cost_response,
        MetricScope::Namespace,
        Some(namespace),
        &unit_prices,
    );
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_namespaces_cost_trend(q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(None, q.clone()).await?;
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let mut cost_response = aggregated.clone();
    apply_costs(&mut cost_response, &unit_prices);
    let dto = build_cost_trend_dto(&cost_response, MetricScope::Namespace, None)?;
    Ok(serde_json::to_value(dto)?)
}

pub async fn get_metric_k8s_namespace_cost_trend(namespace: String, q: RangeQuery) -> Result<Value> {
    let aggregated = build_namespace_cost(Some(namespace.clone()), q.clone()).await?;
    let unit_prices = info_unit_price_service::get_info_unit_prices().await?;
    let mut cost_response = aggregated.clone();
    apply_costs(&mut cost_response, &unit_prices);
    let dto = build_cost_trend_dto(
        &cost_response,
        MetricScope::Namespace,
        Some(namespace),
    )?;
    Ok(serde_json::to_value(dto)?)
}
