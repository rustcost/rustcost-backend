use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::pod::metric_pod_entity::MetricPodEntity;
use crate::scheduler::tasks::collectors::k8s::summary_dto::{NetworkStats, PodSummary};

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

        // Filesystem (from ephemeral-storage)
        fs_used_bytes: pod.ephemeral_storage.as_ref().and_then(|fs| fs.used_bytes),
        fs_capacity_bytes: pod.ephemeral_storage.as_ref().and_then(|fs| fs.capacity_bytes),
        fs_inodes_used: pod.ephemeral_storage.as_ref().and_then(|fs| fs.inodes_used),
        fs_inodes: pod.ephemeral_storage.as_ref().and_then(|fs| fs.inodes),
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
