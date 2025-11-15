use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use crate::api::dto::metrics_dto::RangeQuery;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use crate::domain::metric::k8s::common::dto::{
    CommonMetricValuesDto, CostMetricDto, FilesystemMetricDto, MetricGetResponseDto, MetricGranularity,
    MetricScope, MetricSeriesDto, UniversalMetricPointDto,
};
use crate::domain::metric::k8s::common::dto::metric_k8s_cost_summary_dto::{
    MetricCostSummaryDto, MetricCostSummaryResponseDto,
};
use crate::domain::metric::k8s::common::dto::metric_k8s_cost_trend_dto::{
    MetricCostTrendDto, MetricCostTrendResponseDto,
};
use crate::domain::metric::k8s::common::dto::metric_k8s_raw_efficiency_dto::{
    MetricRawEfficiencyDto, MetricRawEfficiencyResponseDto,
};
use crate::domain::metric::k8s::common::dto::metric_k8s_raw_summary_dto::{
    MetricRawSummaryDto, MetricRawSummaryResponseDto,
};
use crate::domain::metric::k8s::common::util::k8s_metric_determine_granularity::determine_granularity;
use std::collections::HashMap;
use tracing::error;
use tracing::log::warn;

pub const BYTES_PER_GB: f64 = 1_073_741_824.0;

#[derive(Clone)]
pub struct TimeWindow {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub granularity: MetricGranularity,
}

pub fn resolve_time_window(q: &RangeQuery) -> TimeWindow {
    let start = q
        .start
        .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        .unwrap_or_else(|| Utc::now() - chrono::Duration::hours(1));

    let end = q
        .end
        .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
        .unwrap_or_else(Utc::now);

    let granularity = match q.granularity.clone() {
        Some(g) => {
            // Soft validation: log but never fail
            if let Err(err) = validate_granularity(start, end, g.clone()) {
                warn!("Invalid granularity override {:?}: {}", g, err);
                // fallback to automatic granularity
                determine_granularity(start, end)
            } else {
                g
            }
        }
        None => determine_granularity(start, end),
    };

    TimeWindow { start, end, granularity }
}


pub fn validate_granularity(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    granularity: MetricGranularity,
) -> Result<(), String> {
    let diff = end - start;

    match granularity {
        MetricGranularity::Minute => {
            if diff > chrono::Duration::hours(3) {
                return Err("minute granularity cannot be used for ranges > 3 hours".into());
            }
        }
        MetricGranularity::Hour => {
            if diff > chrono::Duration::days(3) {
                return Err("hour granularity cannot be used for ranges > 3 days".into());
            }
        }
        MetricGranularity::Day => { /* always allowed */ }
    }

    Ok(())
}

pub fn build_raw_summary_value(
    metrics: &MetricGetResponseDto,
    scope: MetricScope,
    member_count: usize,
) -> Result<Value> {
    let mut total_cpu = 0.0;
    let mut max_cpu = 0.0;
    let mut total_mem = 0.0;
    let mut max_mem = 0.0;
    let mut total_storage = 0.0;
    let mut max_storage = 0.0;
    let mut total_network = 0.0;
    let mut max_network = 0.0;
    let mut point_count = 0.0;

    for series in &metrics.series {
        for point in &series.points {
            let cpu = point.cpu_memory.cpu_usage_nano_cores.unwrap_or(0.0) / 1_000_000_000.0;
            let mem_gb = point.cpu_memory.memory_usage_bytes.unwrap_or(0.0) / BYTES_PER_GB;
            let fs_gb = point
                .filesystem
                .as_ref()
                .and_then(|fs| fs.used_bytes)
                .unwrap_or(0.0)
                / BYTES_PER_GB;
            let net_gb = point
                .network
                .as_ref()
                .map(|n| {
                    (n.rx_bytes.unwrap_or(0.0) + n.tx_bytes.unwrap_or(0.0)) / BYTES_PER_GB
                })
                .unwrap_or(0.0);

            total_cpu += cpu;
            total_mem += mem_gb;
            total_storage += fs_gb;
            total_network += net_gb;

            if cpu > max_cpu {
                max_cpu = cpu;
            }
            if mem_gb > max_mem {
                max_mem = mem_gb;
            }
            if fs_gb > max_storage {
                max_storage = fs_gb;
            }
            if net_gb > max_network {
                max_network = net_gb;
            }

            point_count += 1.0;
        }
    }

    if point_count == 0.0 {
        return Ok(json!({ "status": "no data" }));
    }

    let summary = MetricRawSummaryDto {
        avg_cpu_cores: total_cpu / point_count,
        max_cpu_cores: max_cpu,
        avg_memory_gb: total_mem / point_count,
        max_memory_gb: max_mem,
        avg_storage_gb: total_storage / point_count,
        max_storage_gb: max_storage,
        avg_network_gb: total_network / point_count,
        max_network_gb: max_network,
        node_count: member_count,
    };

    let dto = MetricRawSummaryResponseDto {
        start: metrics.start,
        end: metrics.end,
        scope,
        granularity: metrics.granularity.clone(),
        summary,
    };

    Ok(serde_json::to_value(dto)?)
}

pub fn apply_costs(response: &mut MetricGetResponseDto, unit_prices: &InfoUnitPriceEntity) {
    for series in &mut response.series {
        for point in &mut series.points {
            let cpu_cost_usd = point.cpu_memory.cpu_usage_nano_cores.map(|nano| {
                let cores = nano / 1_000_000_000.0;
                cores * (unit_prices.cpu_core_hour / 3600.0)
            });

            let memory_cost_usd = point.cpu_memory.memory_usage_bytes.map(|bytes| {
                let gb = bytes / BYTES_PER_GB;
                gb * (unit_prices.memory_gb_hour / 3600.0)
            });

            let storage_cost_usd = point
                .filesystem
                .as_ref()
                .and_then(|fs| fs.used_bytes)
                .map(|bytes| {
                    let gb = bytes / BYTES_PER_GB;
                    gb * (unit_prices.storage_gb_hour / 3600.0)
                });

            let total_cost_usd = Some(
                cpu_cost_usd.unwrap_or(0.0)
                    + memory_cost_usd.unwrap_or(0.0)
                    + storage_cost_usd.unwrap_or(0.0),
            );

            point.cost = Some(CostMetricDto {
                total_cost_usd,
                cpu_cost_usd,
                memory_cost_usd,
                storage_cost_usd,
            });
        }
    }
}

pub fn build_cost_summary_dto(
    metrics: &MetricGetResponseDto,
    scope: MetricScope,
    target: Option<String>,
    unit_prices: &InfoUnitPriceEntity,
) -> MetricCostSummaryResponseDto {
    let mut summary = MetricCostSummaryDto::default();

    for series in &metrics.series {
        for point in &series.points {
            if let Some(cost) = &point.cost {
                summary.cpu_cost_usd += cost.cpu_cost_usd.unwrap_or(0.0);
                summary.memory_cost_usd += cost.memory_cost_usd.unwrap_or(0.0);

                let ephemeral_cost = point
                    .filesystem
                    .as_ref()
                    .and_then(|fs| fs.used_bytes)
                    .map(|b| b / BYTES_PER_GB * unit_prices.storage_gb_hour / 3600.0)
                    .unwrap_or(0.0);

                let persistent_cost = point
                    .storage
                    .as_ref()
                    .and_then(|s| s.persistent.as_ref())
                    .and_then(|fs| fs.used_bytes)
                    .map(|b| b / BYTES_PER_GB * unit_prices.storage_gb_hour / 3600.0)
                    .unwrap_or(0.0);

                let network_cost = point
                    .network
                    .as_ref()
                    .map(|n| {
                        let rx_gb = n.rx_bytes.unwrap_or(0.0) / BYTES_PER_GB;
                        let tx_gb = n.tx_bytes.unwrap_or(0.0) / BYTES_PER_GB;
                        (rx_gb + tx_gb) * unit_prices.network_external_gb / 3600.0
                    })
                    .unwrap_or(0.0);

                summary.ephemeral_storage_cost_usd += ephemeral_cost;
                summary.persistent_storage_cost_usd += persistent_cost;
                summary.network_cost_usd += network_cost;

                summary.total_cost_usd += cost.total_cost_usd.unwrap_or(0.0)
                    + ephemeral_cost
                    + persistent_cost
                    + network_cost;
            }
        }
    }

    MetricCostSummaryResponseDto {
        start: metrics.start,
        end: metrics.end,
        scope,
        target,
        granularity: metrics.granularity.clone(),
        summary,
    }
}

pub fn build_cost_trend_dto(
    metrics: &MetricGetResponseDto,
    scope: MetricScope,
    target: Option<String>,
) -> Result<MetricCostTrendResponseDto> {
    let mut points: Vec<(f64, f64)> = Vec::new();

    for (series_idx, series) in metrics.series.iter().enumerate() {
        for (point_idx, point) in series.points.iter().enumerate() {
            if let Some(cost) = &point.cost {
                if let Some(total) = cost.total_cost_usd {
                    points.push(((series_idx * 1000 + point_idx) as f64, total));
                }
            }
        }
    }

    if points.is_empty() {
        return Err(anyhow!("no cost data available for trend analysis"));
    }

    let start_cost = points.first().map(|(_, y)| *y).unwrap_or(0.0);
    let end_cost = points.last().map(|(_, y)| *y).unwrap_or(0.0);
    let diff = end_cost - start_cost;
    let growth_rate = if start_cost > 0.0 {
        (diff / start_cost) * 100.0
    } else {
        0.0
    };

    let n = points.len() as f64;
    let sum_x: f64 = points.iter().map(|(x, _)| x).sum();
    let sum_y: f64 = points.iter().map(|(_, y)| y).sum();
    let sum_xx: f64 = points.iter().map(|(x, _)| x * x).sum();
    let sum_xy: f64 = points.iter().map(|(x, y)| x * y).sum();

    let slope = if n * sum_xx - sum_x * sum_x != 0.0 {
        (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x)
    } else {
        0.0
    };

    let predicted_next = points.last().map(|_| end_cost + slope);

    Ok(MetricCostTrendResponseDto {
        start: metrics.start,
        end: metrics.end,
        scope,
        target,
        granularity: metrics.granularity.clone(),
        trend: MetricCostTrendDto {
            start_cost_usd: start_cost,
            end_cost_usd: end_cost,
            cost_diff_usd: diff,
            growth_rate_percent: growth_rate,
            regression_slope_usd_per_granularity: slope,
            predicted_next_cost_usd: predicted_next,
        },
    })
}

pub fn build_efficiency_value(
    summary: MetricRawSummaryResponseDto,
    scope: MetricScope,
    total_cpu_alloc: f64,
    total_mem_alloc_gb: f64,
    total_storage_alloc_gb: f64,
) -> Result<Value> {
    let cpu_eff = if total_cpu_alloc > 0.0 {
        (summary.summary.avg_cpu_cores / total_cpu_alloc).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let mem_eff = if total_mem_alloc_gb > 0.0 {
        (summary.summary.avg_memory_gb / total_mem_alloc_gb).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let storage_eff = if total_storage_alloc_gb > 0.0 {
        (summary.summary.avg_storage_gb / total_storage_alloc_gb).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let dto = MetricRawEfficiencyResponseDto {
        start: summary.start,
        end: summary.end,
        scope,
        granularity: summary.granularity,
        efficiency: MetricRawEfficiencyDto {
            cpu_efficiency: cpu_eff,
            memory_efficiency: mem_eff,
            storage_efficiency: storage_eff,
            overall_efficiency: (cpu_eff + mem_eff + storage_eff) / 3.0,
            total_cpu_allocatable_cores: total_cpu_alloc,
            total_memory_allocatable_gb: total_mem_alloc_gb,
            total_storage_allocatable_gb: total_storage_alloc_gb,
        },
    };

    Ok(serde_json::to_value(dto)?)
}
pub fn aggregate_points(points: Vec<UniversalMetricPointDto>) -> Vec<UniversalMetricPointDto> {
    let mut map: HashMap<i64, Vec<UniversalMetricPointDto>> = HashMap::new();

    for point in points {
        map.entry(point.time.timestamp()).or_default().push(point);
    }

    let mut aggregated = Vec::new();

    for (ts, pts) in map {
        let len = pts.len() as f64;
        if len == 0.0 {
            continue;
        }

        let mut cpu_usage = 0.0;
        let mut memory_usage = 0.0;
        let mut fs_used = 0.0;
        let mut fs_capacity = 0.0;

        for p in &pts {
            cpu_usage += p.cpu_memory.cpu_usage_nano_cores.unwrap_or(0.0);
            memory_usage += p.cpu_memory.memory_usage_bytes.unwrap_or(0.0);

            if let Some(fs) = &p.filesystem {
                fs_used += fs.used_bytes.unwrap_or(0.0);
                fs_capacity += fs.capacity_bytes.unwrap_or(0.0);
            }
        }

        let time = chrono::DateTime::<Utc>::from_timestamp(ts, 0).unwrap_or_else(|| Utc::now());

        aggregated.push(UniversalMetricPointDto {
            time,
            cpu_memory: CommonMetricValuesDto {
                cpu_usage_nano_cores: Some(cpu_usage / len),
                memory_usage_bytes: Some(memory_usage / len),
                ..Default::default()
            },
            filesystem: Some(FilesystemMetricDto {
                used_bytes: Some(fs_used / len),
                capacity_bytes: Some(fs_capacity / len),
                ..Default::default()
            }),
            ..Default::default()
        });
    }

    aggregated.sort_by_key(|p| p.time);
    aggregated
}

pub fn aggregate_cost_points(series: &[MetricSeriesDto]) -> Vec<UniversalMetricPointDto> {
    let mut map: HashMap<i64, (chrono::DateTime<Utc>, f64, f64, f64, f64)> = HashMap::new();

    for s in series {
        for point in &s.points {
            if let Some(cost) = &point.cost {
                let entry = map
                    .entry(point.time.timestamp())
                    .or_insert((point.time, 0.0, 0.0, 0.0, 0.0));

                entry.1 += cost.total_cost_usd.unwrap_or(0.0);
                entry.2 += cost.cpu_cost_usd.unwrap_or(0.0);
                entry.3 += cost.memory_cost_usd.unwrap_or(0.0);
                entry.4 += cost.storage_cost_usd.unwrap_or(0.0);
            }
        }
    }

    let mut aggregated = Vec::new();

    for (_, (time, total, cpu, mem, storage)) in map {
        aggregated.push(UniversalMetricPointDto {
            time,
            cost: Some(CostMetricDto {
                total_cost_usd: Some(total),
                cpu_cost_usd: Some(cpu),
                memory_cost_usd: Some(mem),
                storage_cost_usd: Some(storage),
            }),
            ..Default::default()
        });
    }

    aggregated.sort_by_key(|p| p.time);
    aggregated
}
