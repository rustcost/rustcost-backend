use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use anyhow::{anyhow, Context, Result};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};
use tracing::log::debug;
use crate::core::persistence::info::path::{info_k8s_pod_key_dir_path, info_k8s_pod_file_path, info_k8s_container_file_path};

/// File-based FS adapter for `InfoPodEntity`.
///
/// Each pod has its own file at `data/info/pod/{pod_uid}/info.rci`.
/// Uses a simple key–value text format for human readability.
pub struct InfoPodFsAdapter;

impl InfoDynamicFsAdapterTrait<InfoPodEntity> for InfoPodFsAdapter {
    /// Reads the pod info file into memory.
    /// Returns a default entity if the file does not exist.
    fn read(&self, pod_uid: &str) -> Result<InfoPodEntity> {

        let path = info_k8s_pod_file_path(pod_uid);
        if !Path::new(&path).exists() {
            return Err(anyhow!("Missing pod info file '{}'", path.display()));
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

                    // Team / Service / Env
                    "TEAM" => v.team = Some(val),
                    "SERVICE" => v.service = Some(val),
                    "ENV" => v.env = Some(val),
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
        // 1️⃣ Ensure we have a Pod UID
        let pod_uid = data
            .pod_uid
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing pod_uid in InfoPodEntity"))?;

        // 2️⃣ Ensure the directory exists
        Self::create_pod_dir_if_missing(pod_uid)
            .with_context(|| format!("Failed to prepare pod directory for '{}'", pod_uid))?;

        // 3️⃣ Log what we’re about to persist (optional but strongly recommended)
        debug!(
            "📝 Updating InfoPodEntity '{}': {}",
            pod_uid,
            serde_json::to_string_pretty(data).unwrap_or_default()
        );

        // 4️⃣ Perform atomic write
        self.write(pod_uid, data)
            .with_context(|| format!("Failed to write pod info for '{}'", pod_uid))?;

        // 5️⃣ Confirm
        debug!("✅ Successfully updated InfoPodEntity '{}'", pod_uid);
        Ok(())
    }

    /// Deletes the pod info file if present.
    fn delete(&self, pod_uid: &str) -> Result<()> {
        let path = info_k8s_pod_file_path(pod_uid);
        if Path::new(&path).exists() {
            fs::remove_file(&path).context("Failed to delete pod info file")?;
        }
        Ok(())
    }

    fn exists(&self, pod_uid: &str) -> Result<bool> {
        let path = info_k8s_pod_file_path(pod_uid);
        Ok(Path::new(&path).exists())
    }
}

impl InfoPodFsAdapter {
    /// Ensures the pod info directory exists.
    pub fn create_pod_dir_if_missing(pod_uid: &str) -> Result<()> {
        let path = info_k8s_pod_key_dir_path(pod_uid);
        fs::create_dir_all(&path).context("Failed to create pod info directory")?;
        Ok(())
    }

    pub fn write(&self, pod_uid: &str, data: &InfoPodEntity) -> Result<()> {
        let dir = info_k8s_pod_key_dir_path(pod_uid);
        fs::create_dir_all(&dir).context("Failed to create pod info directory")?;

        let tmp_path = dir.join("info.rci.tmp");
        let final_path = dir.join("info.rci");

        let mut f = File::create(&tmp_path).context("Failed to create temp file")?;

        // --- helper macros --------------------------------------------------

        macro_rules! write_field {
            ($key:expr, $val:expr) => {{
                let val_str = match &$val {
                    Some(v) => v.to_string(),
                    None => String::new(),
                };
                debug!("✏️  Writing {}: {}", $key, val_str);
                writeln!(f, "{}:{}", $key, val_str)?;
            }};
        }

        macro_rules! write_vec {
            ($key:expr, $val:expr) => {{
                let val_str = match &$val {
                    Some(v) if !v.is_empty() => v.join(","),
                    _ => String::new(),
                };
                debug!("✏️  Writing {}: {}", $key, val_str);
                writeln!(f, "{}:{}", $key, val_str)?;
            }};
        }

        macro_rules! write_datetime {
            ($key:expr, $val:expr) => {{
                let val_str = match &$val {
                    Some(dt) => dt.to_rfc3339(),
                    None => String::new(),
                };
                debug!("✏️  Writing {}: {}", $key, val_str);
                writeln!(f, "{}:{}", $key, val_str)?;
            }};
        }

        // --- Identity ---
        write_field!("POD_NAME", data.pod_name);
        write_field!("NAMESPACE", data.namespace);
        write_field!("POD_UID", data.pod_uid);

        // --- Lifecycle ---
        write_datetime!("CREATION_TIMESTAMP", data.creation_timestamp);
        write_datetime!("START_TIME", data.start_time);
        write_field!("RESOURCE_VERSION", data.resource_version);
        write_datetime!("LAST_UPDATED_INFO_AT", data.last_updated_info_at);
        write_field!("DELETED", data.deleted.map(|v| v.to_string()));
        write_field!(
            "LAST_CHECK_DELETED_COUNT",
            data.last_check_deleted_count.map(|v| v.to_string())
        );

        // --- Node association ---
        write_field!("NODE_NAME", data.node_name);
        write_field!("HOST_IP", data.host_ip);
        write_field!("POD_IP", data.pod_ip);

        // --- Status ---
        write_field!("QOS_CLASS", data.qos_class);
        write_field!("PHASE", data.phase);
        write_field!("READY", data.ready.map(|v| v.to_string()));
        write_field!("RESTART_COUNT", data.restart_count.map(|v| v.to_string()));

        // --- Owner ---
        write_field!("OWNER_KIND", data.owner_kind);
        write_field!("OWNER_NAME", data.owner_name);
        write_field!("OWNER_UID", data.owner_uid);

        // --- Containers ---
        write_field!("CONTAINER_COUNT", data.container_count.map(|v| v.to_string()));
        write_vec!("CONTAINER_NAMES", data.container_names);
        write_vec!("CONTAINER_IMAGES", data.container_images);
        write_vec!("CONTAINER_IDS", data.container_ids);
        write_vec!("IMAGE_IDS", data.image_ids);

        let ports_str = data
            .container_ports
            .as_ref()
            .map(|v| v.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(","))
            .unwrap_or_default();
        writeln!(f, "CONTAINER_PORTS:{}", ports_str)?;

        write_field!("RESTART_POLICY", data.restart_policy);
        write_field!("SCHEDULER_NAME", data.scheduler_name);
        write_field!("SERVICE_ACCOUNT", data.service_account);

        // --- Volumes ---
        write_field!("VOLUME_COUNT", data.volume_count.map(|v| v.to_string()));
        write_vec!("VOLUME_NAMES", data.volume_names);
        write_vec!("PVC_NAMES", data.pvc_names);
        write_vec!("MOUNT_PATHS", data.mount_paths);
        write_field!(
            "TERMINATION_GRACE_PERIOD_SECONDS",
            data.termination_grace_period_seconds.map(|v| v.to_string())
        );
        write_vec!("TOLERATIONS", data.tolerations);

        // --- Metadata ---
        write_field!("LABEL", data.label);
        write_field!("ANNOTATION", data.annotation);

        write_field!("TEAM", data.team.clone());
        write_field!("SERVICE", data.service.clone());
        write_field!("ENV", data.env.clone());

        f.flush()?;
        fs::rename(&tmp_path, &final_path).context("Failed to finalize pod info file")?;

        debug!("💾 Successfully wrote info.rci for '{}'", pod_uid);
        Ok(())
    }
}

