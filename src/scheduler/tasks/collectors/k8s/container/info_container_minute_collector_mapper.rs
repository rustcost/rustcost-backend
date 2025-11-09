use crate::scheduler::tasks::collectors::k8s::summary_dto::ContainerSummary;
use chrono::{DateTime, Utc};
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;

/// Maps a Kubelet `ContainerSummary` into a persistent `InfoContainerEntity`.
///
/// `pod_uid` and `namespace` are provided externally since the container summary
/// does not include them.
pub fn map_container_summary_to_info(
    container: &ContainerSummary,
    pod_uid: &str,
    namespace: &str,
    node_name: &str,
) -> InfoContainerEntity {
    InfoContainerEntity {
        // --- Identity ---
        pod_uid: Some(pod_uid.to_string()),
        container_name: Some(container.name.clone()),
        namespace: Some(namespace.to_string()),

        // --- Lifecycle ---
        creation_timestamp: None,
        start_time: DateTime::parse_from_rfc3339(&container.start_time)
            .ok()
            .map(|t| t.with_timezone(&Utc)),

        // --- Node association ---
        node_name: Some(node_name.to_string()),

        // --- Bookkeeping ---
        last_updated_info_at: Some(Utc::now()),
        deleted: Some(false),
        last_check_deleted_count: Some(0),

        // Everything else not available from ContainerSummary — leave as None
        ..Default::default()
    }
}
