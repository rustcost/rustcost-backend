use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::core::client::k8s::client_k8s_pod_dto::Pod;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;

pub fn map_pod_to_info_pod_entity(pod: &Pod) -> Result<InfoPodEntity> {
    let metadata = &pod.metadata;
    let spec = &pod.spec;
    let status = pod.status.as_ref();

    // --- Parse timestamps ---
    let creation_timestamp = metadata
        .creation_timestamp
        .as_ref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let start_time = status
        .and_then(|s| s.start_time.as_ref())
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let last_updated_info_at = Some(Utc::now());

    // --- Identity ---
    let pod_name = Some(metadata.name.clone());
    let namespace = Some(metadata.namespace.clone());
    let pod_uid = Some(metadata.uid.clone());

    // --- Owner reference ---
    let (owner_kind, owner_name, owner_uid) = metadata
        .owner_references
        .as_ref()
        .and_then(|owners| owners.first())
        .map(|o| (o.kind.clone(), o.name.clone(), o.uid.clone()))
        .unwrap_or((None, None, None));

    // --- Node association ---
    let node_name = spec.node_name.clone();
    let host_ip = status.and_then(|s| s.host_ip.clone());
    let pod_ip = status.and_then(|s| s.pod_ip.clone());

    // --- Status ---
    let qos_class = status.and_then(|s| s.qos_class.clone());
    let phase = status.and_then(|s| s.phase.clone());

    let ready = status
        .and_then(|s| s.conditions.as_ref())
        .and_then(|conds| {
            conds.iter()
                .find(|c| c.type_field == "Ready")
                .map(|c| c.status == "True")
        });

    let restart_count = status.map(|s| {
        s.container_statuses
            .iter()
            .map(|c| c.restart_count as u32)
            .sum::<u32>()
    });

    // --- Containers ---
    let containers = &spec.containers;
    let container_count = Some(containers.len() as u32);
    let container_names = Some(containers.iter().map(|c| c.name.clone()).collect());
    let container_images = Some(
        containers
            .iter()
            .map(|c| c.image.clone().unwrap_or_default())
            .collect::<Vec<_>>(),
    );

    // ✅ no .ports field → safely skip
    let container_ports: Option<Vec<u16>> = None;

    // --- Container runtime info ---
    let (container_ids, image_ids, container_started_at) = status
        .map(|s| {
            let mut ids = vec![];
            let mut image_ids = vec![];
            let mut started = vec![];

            for cs in &s.container_statuses {
                if let Some(id) = &cs.container_id {
                    ids.push(id.clone());
                }
                if let Some(iid) = &cs.image_id {
                    image_ids.push(iid.clone());
                }
                if let Some(state) = &cs.state {
                    if let Some(running) = &state.running {
                        if let Some(start_str) = &running.started_at {
                            if let Ok(dt) = DateTime::parse_from_rfc3339(start_str) {
                                started.push(dt.with_timezone(&Utc));
                            }
                        }
                    }
                }
            }
            (
                if ids.is_empty() { None } else { Some(ids) },
                if image_ids.is_empty() { None } else { Some(image_ids) },
                if started.is_empty() { None } else { Some(started) },
            )
        })
        .unwrap_or((None, None, None));

    // --- Spec-level settings ---
    let restart_policy = spec.restart_policy.clone();
    let scheduler_name = spec.scheduler_name.clone();
    let service_account = spec.service_account_name.clone();
    let termination_grace_period_seconds = spec.termination_grace_period_seconds;

    // --- Volumes ---
    let volume_count = spec.volumes.as_ref().map(|v| v.len() as u32);
    let volume_names = spec
        .volumes
        .as_ref()
        .map(|v| v.iter().map(|vol| vol.name.clone()).collect());
    let pvc_names = spec.volumes.as_ref().map(|v| {
        v.iter()
            .filter_map(|vol| vol.persistent_volume_claim.as_ref())
            .filter_map(|m| m.get("claimName").cloned())
            .collect::<Vec<String>>()
    });

    // --- Tolerations ---
    let tolerations = spec.tolerations.as_ref().map(|tols| {
        tols.iter()
            .map(|t| {
                format!(
                    "{}={} ({:?})",
                    t.key.clone().unwrap_or_default(),
                    t.value.clone().unwrap_or_default(),
                    t.effect.clone().unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
    });

    // --- Metadata ---
    let label = metadata
        .labels
        .as_ref()
        .map(|l| serde_json::to_string(l).unwrap_or_default());
    let annotation = metadata
        .annotations
        .as_ref()
        .map(|a| serde_json::to_string(a).unwrap_or_default());

    Ok(InfoPodEntity {
        pod_name,
        namespace,
        pod_uid,
        creation_timestamp,
        start_time,
        resource_version: metadata.resource_version.clone(),
        last_updated_info_at,
        deleted: Some(false),
        node_name,
        host_ip,
        pod_ip,
        qos_class,
        phase,
        ready,
        restart_count,
        owner_kind,
        owner_name,
        owner_uid,
        container_count,
        container_names,
        container_images,
        container_ids,
        container_started_at,
        image_ids,
        container_ports,
        restart_policy,
        scheduler_name,
        service_account,
        volume_count,
        volume_names,
        pvc_names,
        termination_grace_period_seconds,
        tolerations,
        label,
        annotation,
        ..Default::default()
    })
}
