use crate::core::persistence::info::dynamic::pod::info_pod_collector_repository_trait::InfoPodCollectorRepository;
use crate::core::persistence::metrics::pod::minute::metric_pod_minute_collector_repository_trait::MetricPodMinuteCollectorRepository;
use crate::core::persistence::metrics::pod::minute::metric_pod_minute_fs_adapter::MetricPodMinuteFsAdapter;
use crate::scheduler::tasks::collectors::k8s::pod::info_pod_minute_collector_mapper::map_pod_summary_to_info;
use crate::scheduler::tasks::collectors::k8s::pod::info_pod_minute_collector_repository::InfoPodCollectorRepositoryImpl;
use crate::scheduler::tasks::collectors::k8s::pod::metric_pod_minute_collector_mapper::map_pod_summary_to_metrics;
use crate::scheduler::tasks::collectors::k8s::pod::metric_pod_minute_collector_repository::MetricPodMinuteCollectorRepositoryImpl;
use crate::scheduler::tasks::collectors::k8s::summary_dto::Summary;
use anyhow::Result;

pub async fn handle_pod(summary: &Summary) -> Result<bool> {
    let mut any_created = false;

    // Step 1: If there are no pods, return early
    let pods = match &summary.pods {
        Some(p) if !p.is_empty() => p,
        _ => return Ok(false),
    };

    // Step 2: Iterate each pod
    for pod in pods {
        let pod_uid = &pod.pod_ref.uid;

        // ---- Info section ----
        let info_repo = InfoPodCollectorRepositoryImpl::default();
        let pod_info = map_pod_summary_to_info(pod, &summary.node.node_name);
        let created = info_repo.create_if_missing(pod_uid, &pod_info)?;
        if created {
            any_created = true;
        }

        // ---- Metrics section ----
        let metric_repo = MetricPodMinuteCollectorRepositoryImpl {
            adapter: MetricPodMinuteFsAdapter,
        };
        let metrics_dto = map_pod_summary_to_metrics(pod);
        metric_repo.append_row(pod_uid, &metrics_dto)?;
    }

    Ok(any_created)
}
