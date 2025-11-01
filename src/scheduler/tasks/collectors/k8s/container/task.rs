use crate::core::persistence::info::dynamic::container::info_container_collector_repository_trait::InfoContainerCollectorRepository;
use crate::core::persistence::metrics::container::minute::metric_container_minute_collector_repository_trait::MetricContainerMinuteCollectorRepository;
use crate::core::persistence::metrics::container::minute::metric_container_minute_fs_adapter::MetricContainerMinuteFsAdapter;
use crate::scheduler::tasks::collectors::k8s::container::metric_container_minute_collector_repository::MetricContainerMinuteCollectorRepositoryImpl;
use crate::scheduler::tasks::collectors::k8s::summary_dto::Summary;
use anyhow::Result;
use crate::scheduler::tasks::collectors::k8s::container::info_container_minute_collector_mapper::map_container_summary_to_info;
use crate::scheduler::tasks::collectors::k8s::container::info_container_minute_collector_repository::InfoContainerCollectorRepositoryImpl;
use crate::scheduler::tasks::collectors::k8s::container::metric_container_minute_collector_mapper::map_container_summary_to_metrics;

/// Collects container-level info and metrics from the node summary.
pub async fn handle_container(summary: &Summary) -> Result<bool> {
    let mut any_created = false;

    // Step 1: Return early if no pods
    let pods = match &summary.pods {
        Some(p) if !p.is_empty() => p,
        _ => return Ok(false),
    };

    // Step 2: Iterate each pod and its containers
    for pod in pods {
        let pod_uid = &pod.pod_ref.uid;
        let namespace = &pod.pod_ref.namespace;
        let node_name = &summary.node.node_name;

        // Skip pods that have no containers
        if pod.containers.is_empty() {
            continue;
        }

        for container in &pod.containers {
            // Compose a unique key for this container
            let container_key = format!("{}-{}", pod_uid, container.name);

            // ---- Info section ----
            let info_repo = InfoContainerCollectorRepositoryImpl::default();
            let container_info =
                map_container_summary_to_info(container, pod_uid, namespace, node_name);
            let created = info_repo.create_if_missing(&container_key, &container_info)?;
            if created {
                any_created = true;
            }

            // ---- Metrics section ----
            let metric_repo = MetricContainerMinuteCollectorRepositoryImpl {
                adapter: MetricContainerMinuteFsAdapter,
            };
            let metrics_dto = map_container_summary_to_metrics(container);
            metric_repo.append_row(&container_key, &metrics_dto)?;
        }
    }

    Ok(any_created)
}
