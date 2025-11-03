use crate::core::persistence::info::dynamic::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::dynamic::container::info_container_entity::InfoContainerEntity;
use anyhow::{Context, Result};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};
use crate::core::persistence::storage_path::{info_container_dir_path, info_container_file_path};

/// File-based FS adapter for `InfoContainerEntity`.
///
/// Each container has its own file at:
/// `data/info/container/{pod_uid}-{container_name}/info.rci`
pub struct InfoContainerFsAdapter;

impl InfoDynamicFsAdapterTrait<InfoContainerEntity> for InfoContainerFsAdapter {
    /// Reads the container info file into memory.
    fn read(&self, container_key: &str) -> Result<InfoContainerEntity> {
        let path = info_container_file_path(container_key);
        if !Path::new(&path).exists() {
            return Ok(InfoContainerEntity::default());
        }

        let file = File::open(&path).context("Failed to open container info file")?;
        let reader = BufReader::new(file);
        let mut v = InfoContainerEntity::default();

        for line in reader.lines() {
            let line = line?;
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim().to_uppercase();
                let val = val.trim().to_string();

                match key.as_str() {
                    // Identity
                    "POD_UID" => v.pod_uid = Some(val),
                    "CONTAINER_NAME" => v.container_name = Some(val),
                    "NAMESPACE" => v.namespace = Some(val),

                    // Lifecycle
                    "CREATION_TIMESTAMP" => v.creation_timestamp = val.parse().ok(),
                    "START_TIME" => v.start_time = val.parse().ok(),
                    "CONTAINER_ID" => v.container_id = Some(val),
                    "IMAGE" => v.image = Some(val),
                    "IMAGE_ID" => v.image_id = Some(val),

                    // Status
                    "STATE" => v.state = Some(val),
                    "REASON" => v.reason = Some(val),
                    "MESSAGE" => v.message = Some(val),
                    "EXIT_CODE" => v.exit_code = val.parse().ok(),
                    "RESTART_COUNT" => v.restart_count = val.parse().ok(),
                    "READY" => v.ready = Some(val == "true"),

                    // Node association
                    "NODE_NAME" => v.node_name = Some(val),
                    "HOST_IP" => v.host_ip = Some(val),
                    "POD_IP" => v.pod_ip = Some(val),

                    // Resources
                    "CPU_REQUEST_MILLICORES" => v.cpu_request_millicores = val.parse().ok(),
                    "MEMORY_REQUEST_BYTES" => v.memory_request_bytes = val.parse().ok(),
                    "CPU_LIMIT_MILLICORES" => v.cpu_limit_millicores = val.parse().ok(),
                    "MEMORY_LIMIT_BYTES" => v.memory_limit_bytes = val.parse().ok(),

                    // Volumes
                    "VOLUME_MOUNTS" => v.volume_mounts = Some(val.split(',').map(|s| s.trim().to_string()).collect()),
                    "VOLUME_DEVICES" => v.volume_devices = Some(val.split(',').map(|s| s.trim().to_string()).collect()),

                    // Metadata
                    "LABELS" => v.labels = Some(val),
                    "ANNOTATIONS" => v.annotations = Some(val),

                    // Bookkeeping
                    "LAST_UPDATED_INFO_AT" => v.last_updated_info_at = val.parse().ok(),
                    "DELETED" => v.deleted = Some(val == "true"),
                    "LAST_CHECK_DELETED_COUNT" => v.last_check_deleted_count = val.parse().ok(),

                    _ => {}
                }
            }
        }

        Ok(v)
    }

    /// Inserts (creates) a container info file.
    fn insert(&self, data: &InfoContainerEntity) -> Result<()> {
        let key = Self::container_key(data)?;
        Self::create_container_dir_if_missing(&key)?;
        self.write(&key, data)
    }

    /// Updates a container info file.
    fn update(&self, data: &InfoContainerEntity) -> Result<()> {
        let container_key = Self::container_key(data)?;
        Self::create_container_dir_if_missing(&container_key)?;
        self.write(&container_key, data)
    }

    /// Deletes the container info file.
    fn delete(&self, key: &str) -> Result<()> {
        let path = info_container_file_path(key);
        if Path::new(&path).exists() {
            fs::remove_file(&path).context("Failed to delete container info file")?;
        }
        Ok(())
    }

    fn exists(&self, container_key: &str) -> Result<bool> {
        let path = info_container_file_path(container_key);
        Ok(Path::new(&path).exists())
    }
}

impl InfoContainerFsAdapter {
    /// Builds the unique key (directory name) for the container.
    fn container_key(data: &InfoContainerEntity) -> Result<String> {
        let pod_uid = data
            .pod_uid
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing pod_uid"))?;
        let container_name = data
            .container_name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing container_name"))?;
        Ok(format!("{}-{}", pod_uid, container_name))
    }

    /// Ensures the container directory exists.
    pub fn create_container_dir_if_missing(container_key: &str) -> Result<()> {
        let path = info_container_file_path(container_key);
        fs::create_dir_all(&path).context("Failed to create container info directory")?;
        Ok(())
    }

    /// Writes the info.rci file atomically.


    fn write(&self, container_key: &str, data: &InfoContainerEntity) -> Result<()> {
        let dir = info_container_dir_path(container_key);
        fs::create_dir_all(&dir).context("Failed to create container info directory")?;

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
        write_field!("POD_UID", data.pod_uid);
        write_field!("CONTAINER_NAME", data.container_name);
        write_field!("NAMESPACE", data.namespace);

        // Lifecycle
        write_field!("CREATION_TIMESTAMP", data.creation_timestamp.map(|t| t.to_string()));
        write_field!("START_TIME", data.start_time.map(|t| t.to_string()));
        write_field!("CONTAINER_ID", data.container_id);
        write_field!("IMAGE", data.image);
        write_field!("IMAGE_ID", data.image_id);

        // Status
        write_field!("STATE", data.state);
        write_field!("REASON", data.reason);
        write_field!("MESSAGE", data.message);
        write_field!("EXIT_CODE", data.exit_code.map(|v| v.to_string()));
        write_field!("RESTART_COUNT", data.restart_count.map(|v| v.to_string()));
        write_field!("READY", data.ready.map(|v| v.to_string()));

        // Node association
        write_field!("NODE_NAME", data.node_name);
        write_field!("HOST_IP", data.host_ip);
        write_field!("POD_IP", data.pod_ip);

        // Resources
        write_field!("CPU_REQUEST_MILLICORES", data.cpu_request_millicores.map(|v| v.to_string()));
        write_field!("MEMORY_REQUEST_BYTES", data.memory_request_bytes.map(|v| v.to_string()));
        write_field!("CPU_LIMIT_MILLICORES", data.cpu_limit_millicores.map(|v| v.to_string()));
        write_field!("MEMORY_LIMIT_BYTES", data.memory_limit_bytes.map(|v| v.to_string()));

        // Volumes
        write_field!("VOLUME_MOUNTS", data.volume_mounts.clone().map(|v| v.join(",")));
        write_field!("VOLUME_DEVICES", data.volume_devices.clone().map(|v| v.join(",")));

        // Metadata
        write_field!("LABELS", data.labels);
        write_field!("ANNOTATIONS", data.annotations);

        // Bookkeeping
        write_field!("LAST_UPDATED_INFO_AT", data.last_updated_info_at.map(|t| t.to_string()));
        write_field!("DELETED", data.deleted.map(|v| v.to_string()));
        write_field!("LAST_CHECK_DELETED_COUNT", data.last_check_deleted_count.map(|v| v.to_string()));

        f.flush()?;
        fs::rename(&tmp_path, &final_path).context("Failed to finalize container info file")?;
        Ok(())
    }
}
