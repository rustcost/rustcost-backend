use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::scheduler::tasks::collectors::k8s::summary_dto::PodSummary;
use chrono::{DateTime, Utc};

pub fn map_pod_summary_to_info(pod: &PodSummary, node_name: &str) -> InfoPodEntity {
    InfoPodEntity {
        pod_name: Some(pod.pod_ref.name.clone()),
        namespace: Some(pod.pod_ref.namespace.clone()),
        pod_uid: Some(pod.pod_ref.uid.clone()),
        node_name: Some(node_name.to_string()),
        start_time: DateTime::parse_from_rfc3339(&pod.start_time)
            .ok()
            .map(|t| t.with_timezone(&Utc)),
        container_count: Some(pod.containers.len() as u32),
        container_names: Some(pod.containers.iter().map(|c| c.name.clone()).collect()),
        ..Default::default()
    }
}
