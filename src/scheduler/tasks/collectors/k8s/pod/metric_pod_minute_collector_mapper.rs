use crate::core::persistence::metrics::pod::metric_pod_entity::MetricPodEntity;
use crate::scheduler::tasks::collectors::k8s::summary_dto::{NetworkStats, PodSummary, VolumeStats};
use chrono::Utc;

pub fn map_pod_summary_to_metrics(pod: &PodSummary) -> MetricPodEntity {
    // --- Compute summed physical network stats ---
    let (rx, tx, rx_err, tx_err) = pod
        .network
        .as_ref()
        .and_then(|net| sum_network_interfaces(net))
        .unwrap_or_else(|| {
            (
                pod.network.as_ref().and_then(|n| n.rx_bytes),
                pod.network.as_ref().and_then(|n| n.tx_bytes),
                pod.network.as_ref().and_then(|n| n.rx_errors),
                pod.network.as_ref().and_then(|n| n.tx_errors),
            )
        });

    // --- Use CPU timestamp as primary metric timestamp ---
    let time = chrono::DateTime::parse_from_rfc3339(&pod.cpu.time)
        .map(|t| t.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    let (
        _es_used,
        _es_capacity,
        _es_inodes_used,
        _es_inodes,
        pv_used,
        pv_capacity,
        pv_inodes_used,
        pv_inodes,
    ) = sum_volume_stats(&pod.volume);

    MetricPodEntity {
        time,

        // CPU
        cpu_usage_nano_cores: pod.cpu.usage_nano_cores,
        cpu_usage_core_nano_seconds: pod.cpu.usage_core_nano_seconds,

        // Memory
        memory_usage_bytes: pod.memory.usage_bytes,
        memory_working_set_bytes: pod.memory.working_set_bytes,
        memory_rss_bytes: pod.memory.rss_bytes,
        memory_page_faults: pod.memory.page_faults,

        // Network (summed)
        network_physical_rx_bytes: rx,
        network_physical_tx_bytes: tx,
        network_physical_rx_errors: rx_err,
        network_physical_tx_errors: tx_err,

        // Ephemeral storage (summary.ephemeral-storage)
        es_used_bytes: pod.ephemeral_storage.as_ref().and_then(|fs| fs.used_bytes),
        es_capacity_bytes: pod.ephemeral_storage.as_ref().and_then(|fs| fs.capacity_bytes),
        es_inodes_used: pod.ephemeral_storage.as_ref().and_then(|fs| fs.inodes_used),
        es_inodes: pod.ephemeral_storage.as_ref().and_then(|fs| fs.inodes),

        // Persistent storage (PVC-backed)
        pv_used_bytes: pv_used,
        pv_capacity_bytes: pv_capacity,
        pv_inodes_used: pv_inodes_used,
        pv_inodes: pv_inodes,

    }
}

fn sum_network_interfaces(net: &NetworkStats) -> Option<(Option<u64>, Option<u64>, Option<u64>, Option<u64>)> {
    net.interfaces.as_ref().map(|interfaces| {
        let (rx, tx, rx_err, tx_err) = interfaces.iter().fold((0, 0, 0, 0), |acc, iface| {
            (
                acc.0 + iface.rx_bytes.unwrap_or(0),
                acc.1 + iface.tx_bytes.unwrap_or(0),
                acc.2 + iface.rx_errors.unwrap_or(0),
                acc.3 + iface.tx_errors.unwrap_or(0),
            )
        });
        (Some(rx), Some(tx), Some(rx_err), Some(tx_err))
    })
}

/// Sums volume metrics into ephemeral (es_*) and persistent (pv_*) categories.
///
/// Volumes with `pvcRef == Some(_)` are treated as PersistentVolumes (PV),
/// and those with `pvcRef == None` are treated as ephemeral (ES).
///
/// Returns:
/// (es_used, es_capacity, es_inodes_used, es_inodes, pv_used, pv_capacity, pv_inodes_used, pv_inodes)
/// Sums volume metrics into ephemeral (es_*) and persistent (pv_*) categories.
/// Volumes with `pvc_ref == Some(_)` are considered persistent (PV),
/// others are ephemeral (ES).
fn sum_volume_stats(volumes: &Option<Vec<VolumeStats>>) -> (
    Option<u64>, Option<u64>, Option<u64>, Option<u64>, // ES
    Option<u64>, Option<u64>, Option<u64>, Option<u64>, // PV
) {
    let mut es_used = 0u64;
    let mut es_capacity = 0u64;
    let mut es_inodes_used = 0u64;
    let mut es_inodes = 0u64;

    let mut pv_used = 0u64;
    let mut pv_capacity = 0u64;
    let mut pv_inodes_used = 0u64;
    let mut pv_inodes = 0u64;

    if let Some(vols) = volumes {
        for v in vols {
            let used = v.used_bytes.unwrap_or(0);
            let cap = v.capacity_bytes.unwrap_or(0);
            let inodes_used = v.inodes_used.unwrap_or(0);
            let inodes = v.inodes.unwrap_or(0);

            if v.pvc_ref.is_some() {
                // PersistentVolumeClaim-backed
                pv_used += used;
                pv_capacity += cap;
                pv_inodes_used += inodes_used;
                pv_inodes += inodes;
            } else {
                // Ephemeral volume
                es_used += used;
                es_capacity += cap;
                es_inodes_used += inodes_used;
                es_inodes += inodes;
            }
        }
    }

    (
        Some(es_used),
        Some(es_capacity),
        Some(es_inodes_used),
        Some(es_inodes),
        Some(pv_used),
        Some(pv_capacity),
        Some(pv_inodes_used),
        Some(pv_inodes),
    )
}
