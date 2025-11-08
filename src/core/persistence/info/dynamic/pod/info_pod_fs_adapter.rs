use crate::core::persistence::info::dynamic::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::dynamic::pod::info_pod_entity::InfoPodEntity;
use anyhow::{Context, Result};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};
use crate::core::persistence::info::path::{info_pod_dir_path, info_pod_file_path};
use crate::core::persistence::metrics::k8s::path::metric_k8s_pod_key_dir_path;

/// File-based FS adapter for `InfoPodEntity`.
///
/// Each pod has its own file at `data/info/pod/{pod_uid}/info.rci`.
/// Uses a simple key–value text format for human readability.
pub struct InfoPodFsAdapter;

impl InfoDynamicFsAdapterTrait<InfoPodEntity> for InfoPodFsAdapter {
    /// Reads the pod info file into memory.
    /// Returns a default entity if the file does not exist.
    fn read(&self, pod_uid: &str) -> Result<InfoPodEntity> {
        let path = info_pod_file_path(pod_uid);

        if !Path::new(&path).exists() {
            return Ok(InfoPodEntity::default());
        }

        let file = File::open(&path).context("Failed to open pod info file")?;
        let reader = BufReader::new(file);
        let mut v = InfoPodEntity::default();

        for line in reader.lines() {
            let line = line?;
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim().to_uppercase();
                let val = val.trim().to_string();

                match key.as_str() {
                    // Identity
                    "POD_NAME" => v.pod_name = Some(val),
                    "NAMESPACE" => v.namespace = Some(val),
                    "POD_UID" => v.pod_uid = Some(val),

                    // Lifecycle
                    "CREATION_TIMESTAMP" => v.creation_timestamp = val.parse().ok(),
                    "START_TIME" => v.start_time = val.parse().ok(),
                    "RESOURCE_VERSION" => v.resource_version = Some(val),
                    "LAST_UPDATED_INFO_AT" => v.last_updated_info_at = val.parse().ok(),
                    "DELETED" => v.deleted = Some(val == "true"),
                    "LAST_CHECK_DELETED_COUNT" => v.last_check_deleted_count = val.parse().ok(),

                    // Node association
                    "NODE_NAME" => v.node_name = Some(val),
                    "HOST_IP" => v.host_ip = Some(val),
                    "POD_IP" => v.pod_ip = Some(val),

                    // Status
                    "QOS_CLASS" => v.qos_class = Some(val),
                    "PHASE" => v.phase = Some(val),
                    "READY" => v.ready = Some(val == "true"),
                    "RESTART_COUNT" => v.restart_count = val.parse().ok(),

                    // Owner
                    "OWNER_KIND" => v.owner_kind = Some(val),
                    "OWNER_NAME" => v.owner_name = Some(val),
                    "OWNER_UID" => v.owner_uid = Some(val),

                    // Containers
                    "CONTAINER_COUNT" => v.container_count = val.parse().ok(),
                    "CONTAINER_NAMES" => v.container_names = Some(val.split(',').map(|s| s.trim().to_string()).collect()),
                    "CONTAINER_IMAGES" => v.container_images = Some(val.split(',').map(|s| s.trim().to_string()).collect()),
                    "CONTAINER_IDS" => v.container_ids = Some(val.split(',').map(|s| s.trim().to_string()).collect()),
                    "IMAGE_IDS" => v.image_ids = Some(val.split(',').map(|s| s.trim().to_string()).collect()),
                    "CONTAINER_PORTS" => {
                        v.container_ports = Some(
                            val.split(',')
                                .filter_map(|s| s.trim().parse::<u16>().ok())
                                .collect(),
                        )
                    }
                    "RESTART_POLICY" => v.restart_policy = Some(val),
                    "SCHEDULER_NAME" => v.scheduler_name = Some(val),
                    "SERVICE_ACCOUNT" => v.service_account = Some(val),

                    // Volumes
                    "VOLUME_COUNT" => v.volume_count = val.parse().ok(),
                    "VOLUME_NAMES" => v.volume_names = Some(val.split(',').map(|s| s.trim().to_string()).collect()),
                    "PVC_NAMES" => v.pvc_names = Some(val.split(',').map(|s| s.trim().to_string()).collect()),
                    "MOUNT_PATHS" => v.mount_paths = Some(val.split(',').map(|s| s.trim().to_string()).collect()),
                    "TERMINATION_GRACE_PERIOD_SECONDS" => v.termination_grace_period_seconds = val.parse().ok(),
                    "TOLERATIONS" => v.tolerations = Some(val.split(',').map(|s| s.trim().to_string()).collect()),

                    // Metadata
                    "LABEL" => v.label = Some(val),
                    "ANNOTATION" => v.annotation = Some(val),

                    _ => {}
                }
            }
        }

        Ok(v)
    }

    /// Creates the pod info file.
    fn insert(&self, data: &InfoPodEntity) -> Result<()> {
        let pod_uid = data
            .pod_uid
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing pod_uid in InfoPodEntity"))?;

        Self::create_pod_dir_if_missing(pod_uid)?;
        self.write(pod_uid, data)
    }

    /// Updates the pod info file.
    fn update(&self, data: &InfoPodEntity) -> Result<()> {
        let pod_uid = data
            .pod_uid
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing pod_uid in InfoPodEntity"))?;

        Self::create_pod_dir_if_missing(pod_uid)?;
        self.write(pod_uid, data)
    }

    /// Deletes the pod info file if present.
    fn delete(&self, pod_uid: &str) -> Result<()> {
        let path = info_pod_file_path(pod_uid);
        if Path::new(&path).exists() {
            fs::remove_file(&path).context("Failed to delete pod info file")?;
        }
        Ok(())
    }

    fn exists(&self, pod_uid: &str) -> Result<bool> {
        let path = info_pod_file_path(pod_uid);
        Ok(Path::new(&path).exists())
    }
}

impl InfoPodFsAdapter {
    /// Ensures the pod info directory exists.
    pub fn create_pod_dir_if_missing(pod_uid: &str) -> Result<()> {
        let path = info_pod_dir_path(pod_uid);
        fs::create_dir_all(&path).context("Failed to create pod info directory")?;
        Ok(())
    }

    /// Writes the info.rci file atomically.
    fn write(&self, pod_uid: &str, data: &InfoPodEntity) -> Result<()> {
        let dir = metric_k8s_pod_key_dir_path(pod_uid);
        fs::create_dir_all(&dir).context("Failed to create pod info directory")?;

        let tmp_path = dir.join("info.rci.tmp");
        let final_path = dir.join("info.rci");

        let mut f = File::create(&tmp_path).context("Failed to create temp file")?;

        macro_rules! write_field {
        ($key:expr, $val:expr) => {
            let val_str = $val.clone().map_or(String::new(), |v| v.to_string());
            writeln!(f, "{}:{}", $key, val_str)?;
        };
    }

        // Identity
        write_field!("POD_NAME", data.pod_name);
        write_field!("NAMESPACE", data.namespace);
        write_field!("POD_UID", data.pod_uid);

        // Lifecycle
        write_field!("CREATION_TIMESTAMP", data.creation_timestamp.map(|t| t.to_string()));
        write_field!("START_TIME", data.start_time.map(|t| t.to_string()));
        write_field!("RESOURCE_VERSION", data.resource_version);
        write_field!("LAST_UPDATED_INFO_AT", data.last_updated_info_at.map(|t| t.to_string()));
        write_field!("DELETED", data.deleted.map(|v| v.to_string()));
        write_field!("LAST_CHECK_DELETED_COUNT", data.last_check_deleted_count.map(|v| v.to_string()));

        // Node association
        write_field!("NODE_NAME", data.node_name);
        write_field!("HOST_IP", data.host_ip);
        write_field!("POD_IP", data.pod_ip);

        // Status
        write_field!("QOS_CLASS", data.qos_class);
        write_field!("PHASE", data.phase);
        write_field!("READY", data.ready.map(|v| v.to_string()));
        write_field!("RESTART_COUNT", data.restart_count.map(|v| v.to_string()));

        // Owner
        write_field!("OWNER_KIND", data.owner_kind);
        write_field!("OWNER_NAME", data.owner_name);
        write_field!("OWNER_UID", data.owner_uid);

        // Containers
        write_field!("CONTAINER_COUNT", data.container_count.map(|v| v.to_string()));
        write_field!("CONTAINER_NAMES", data.container_names.clone().map(|v| v.join(",")));
        write_field!("CONTAINER_IMAGES", data.container_images.clone().map(|v| v.join(",")));
        write_field!("CONTAINER_IDS", data.container_ids.clone().map(|v| v.join(",")));
        write_field!("IMAGE_IDS", data.image_ids.clone().map(|v| v.join(",")));
        write_field!("CONTAINER_PORTS", data.container_ports.clone().map(|v| v.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(",")));
        write_field!("RESTART_POLICY", data.restart_policy);
        write_field!("SCHEDULER_NAME", data.scheduler_name);
        write_field!("SERVICE_ACCOUNT", data.service_account);

        // Volumes
        write_field!("VOLUME_COUNT", data.volume_count.map(|v| v.to_string()));
        write_field!("VOLUME_NAMES", data.volume_names.clone().map(|v| v.join(",")));
        write_field!("PVC_NAMES", data.pvc_names.clone().map(|v| v.join(",")));
        write_field!("MOUNT_PATHS", data.mount_paths.clone().map(|v| v.join(",")));
        write_field!("TERMINATION_GRACE_PERIOD_SECONDS", data.termination_grace_period_seconds.map(|v| v.to_string()));
        write_field!("TOLERATIONS", data.tolerations.clone().map(|v| v.join(",")));

        // Metadata
        write_field!("LABEL", data.label);
        write_field!("ANNOTATION", data.annotation);

        f.flush()?;
        fs::rename(&tmp_path, &final_path).context("Failed to finalize pod info file")?;
        Ok(())
    }
}
