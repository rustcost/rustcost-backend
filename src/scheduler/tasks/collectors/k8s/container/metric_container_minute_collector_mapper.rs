use crate::core::persistence::metrics::container::metric_container_entity::MetricContainerEntity;
use crate::scheduler::tasks::collectors::k8s::summary_dto::{ContainerSummary, NetworkStats};
use chrono::Utc;

/// Maps a Kubernetes ContainerSummary (from Kubelet /stats/summary) into MetricContainerEntity.
pub fn map_container_summary_to_metrics(container: &ContainerSummary) -> MetricContainerEntity {
    // --- Use CPU timestamp as primary metric timestamp ---
    let time = chrono::DateTime::parse_from_rfc3339(&container.cpu.time)
        .map(|t| t.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    // --- Aggregate ephemeral FS (rootfs + logs) ---
    let (fs_used, fs_capacity, fs_inodes_used, fs_inodes) = sum_fs_stats(container);

    MetricContainerEntity {
        time,

        // CPU
        cpu_usage_nano_cores: container.cpu.usage_nano_cores,
        cpu_usage_core_nano_seconds: container.cpu.usage_core_nano_seconds,

        // Memory
        memory_usage_bytes: container.memory.usage_bytes,
        memory_working_set_bytes: container.memory.working_set_bytes,
        memory_rss_bytes: container.memory.rss_bytes,
        memory_page_faults: container.memory.page_faults,

        // Ephemeral filesystem (rootfs + logs)
        fs_used_bytes: fs_used,
        fs_capacity_bytes: fs_capacity,
        fs_inodes_used: fs_inodes_used,
        fs_inodes: fs_inodes,


    }
}

/// Sums rootfs + logs usage for container ephemeral storage.
fn sum_fs_stats(container: &ContainerSummary) -> (
    Option<u64>,
    Option<u64>,
    Option<u64>,
    Option<u64>,
) {
    let mut used = 0u64;
    let mut capacity = 0u64;
    let mut inodes_used = 0u64;
    let mut inodes = 0u64;

    if let Some(rootfs) = &container.rootfs {
        used += rootfs.used_bytes.unwrap_or(0);
        capacity += rootfs.capacity_bytes.unwrap_or(0);
        inodes_used += rootfs.inodes_used.unwrap_or(0);
        inodes += rootfs.inodes.unwrap_or(0);
    }

    if let Some(logs) = &container.logs {
        used += logs.used_bytes.unwrap_or(0);
        capacity += logs.capacity_bytes.unwrap_or(0);
        inodes_used += logs.inodes_used.unwrap_or(0);
        inodes += logs.inodes.unwrap_or(0);
    }

    (
        Some(used),
        Some(capacity),
        Some(inodes_used),
        Some(inodes),
    )
}