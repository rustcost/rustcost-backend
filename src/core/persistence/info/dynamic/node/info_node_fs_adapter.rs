use super::info_node_entity::InfoNodeEntity;
use crate::core::persistence::info::dynamic::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use anyhow::{Context, Result};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};

/// File-based FS adapter for the `InfoNodeEntity`.
///
/// Each node has its own file at `data/info/node/{node_name}/info.rci`.
/// The adapter supports read/write/update/delete operations using a
/// simple key–value text format, designed to be both human-readable and
/// easy to parse.
pub struct InfoNodeFsAdapter;

impl InfoDynamicFsAdapterTrait<InfoNodeEntity> for InfoNodeFsAdapter {
    /// Reads the node info file into memory.
    /// Returns a default entity if the file does not exist.
    fn read(&self, node_name: &str) -> Result<InfoNodeEntity> {
        let path = format!("data/info/node/{}/info.rci", node_name);

        if !Path::new(&path).exists() {
            return Ok(InfoNodeEntity::default());
        }

        let file = File::open(&path).context("Failed to open node info file")?;
        let reader = BufReader::new(file);
        let mut v = InfoNodeEntity::default();

        for line in reader.lines() {
            let line = line?;
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim().to_uppercase();
                let val = val.trim().to_string();

                match key.as_str() {
                    "NODE_NAME" => v.node_name = Some(val),
                    "NODE_UID" => v.node_uid = Some(val),
                    "CREATION_TIMESTAMP" => v.creation_timestamp = Some(val.parse().unwrap_or_default()),
                    "RESOURCE_VERSION" => v.resource_version = Some(val),
                    "LAST_UPDATED_INFO_AT" => v.last_updated_info_at = Some(val.parse().unwrap_or_default()),
                    "DELETED" => v.deleted = Some(val == "true"),
                    "LAST_CHECK_DELETED_COUNT" => v.last_check_deleted_count = val.parse().ok(),
                    "HOSTNAME" => v.hostname = Some(val),
                    "INTERNAL_IP" => v.internal_ip = Some(val),
                    "ARCHITECTURE" => v.architecture = Some(val),
                    "OS_IMAGE" => v.os_image = Some(val),
                    "KERNEL_VERSION" => v.kernel_version = Some(val),
                    "KUBELET_VERSION" => v.kubelet_version = Some(val),
                    "CONTAINER_RUNTIME" => v.container_runtime = Some(val),
                    "OPERATING_SYSTEM" => v.operating_system = Some(val),
                    "CPU_CAPACITY_CORES" => v.cpu_capacity_cores = val.parse().ok(),
                    "MEMORY_CAPACITY_BYTES" => v.memory_capacity_bytes = val.parse().ok(),
                    "POD_CAPACITY" => v.pod_capacity = val.parse().ok(),
                    "EPHEMERAL_STORAGE_CAPACITY_BYTES" => v.ephemeral_storage_capacity_bytes = val.parse().ok(),
                    "CPU_ALLOCATABLE_CORES" => v.cpu_allocatable_cores = val.parse().ok(),
                    "MEMORY_ALLOCATABLE_BYTES" => v.memory_allocatable_bytes = val.parse().ok(),
                    "EPHEMERAL_STORAGE_ALLOCATABLE_BYTES" => v.ephemeral_storage_allocatable_bytes = val.parse().ok(),
                    "POD_ALLOCATABLE" => v.pod_allocatable = val.parse().ok(),
                    "READY" => v.ready = Some(val == "true"),
                    "TAINTS" => v.taints = Some(val),
                    "LABEL" => v.label = Some(val),
                    "ANNOTATION" => v.annotation = Some(val),
                    "IMAGE_COUNT" => v.image_count = val.parse().ok(),
                    "IMAGE_NAMES" => v.image_names = Some(val.split(',').map(|s| s.trim().to_string()).collect()),
                    "IMAGE_TOTAL_SIZE_BYTES" => v.image_total_size_bytes = val.parse().ok(),
                    _ => {}
                }
            }
        }

        Ok(v)
    }

    /// Creates the node info file.
    fn insert(&self, data: &InfoNodeEntity) -> Result<()> {
        // Safely get the node name or return an error if missing
        let node_name = data
            .node_name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing node_name in InfoNodeEntity"))?;

        Self::create_node_dir_if_missing(node_name)?;
        self.write(node_name, data)
    }

    /// Updates the node info file.
    fn update(&self, data: &InfoNodeEntity) -> Result<()> {
        // Safely get the node name or return an error if missing
        let node_name = data
            .node_name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing node_name in InfoNodeEntity"))?;

        // Create directory if missing
        Self::create_node_dir_if_missing(node_name)?;

        // Write node info
        self.write(node_name, data)
    }

    /// Deletes the node info file if present.
    fn delete(&self, node_name: &str) -> Result<()> {
        let path = format!("data/info/node/{}/info.rci", node_name);
        if Path::new(&path).exists() {
            fs::remove_file(&path).context("Failed to delete node info file")?;
        }
        Ok(())
    }

    fn exists(&self, node_name: &str) -> Result<bool> {
        let path = format!("data/info/node/{}/info.rci", node_name);
        Ok(Path::new(&path).exists())
    }

}

impl InfoNodeFsAdapter {
    /// Ensures the per-node data directories exist.
    /// Creates: data/metrics/nodes/{node_name}/{m,h,d}
    pub fn create_node_dir_if_missing(node_name: &str) -> Result<()> {
        let base = format!("data/metrics/nodes/{node_name}");
        for sub in ["m", "h", "d"] {
            let path = format!("{base}/{sub}");
            if let Err(e) = std::fs::create_dir_all(&path) {
                if e.kind() != std::io::ErrorKind::AlreadyExists {
                    return Err(e.into());
                }
            }
        }
        Ok(())
    }

    /// Internal helper: atomically writes a node info file.
    fn write(&self, node_name: &str, data: &InfoNodeEntity) -> Result<()> {
        let dir = format!("data/info/node/{}", node_name);
        fs::create_dir_all(&dir).context("Failed to create node info directory")?;

        let tmp_path = format!("{}/info.rci.tmp", dir);
        let final_path = format!("{}/info.rci", dir);

        let mut f = File::create(&tmp_path).context("Failed to create temp file")?;

        macro_rules! write_field {
            ($key:expr, $val:expr) => {
                writeln!(f, "{}:{}", $key, $val.unwrap_or_default())?;
            };
        }

        write_field!("NODE_NAME", data.node_name.clone());
        write_field!("NODE_UID", data.node_uid.clone());
        write_field!("CREATION_TIMESTAMP", data.creation_timestamp.clone().map(|t| t.to_string()));
        write_field!("RESOURCE_VERSION", data.resource_version.clone());
        write_field!("LAST_UPDATED_INFO_AT", data.last_updated_info_at.clone().map(|t| t.to_string()));
        write_field!("DELETED", data.deleted.map(|v| v.to_string()));
        write_field!("LAST_CHECK_DELETED_COUNT", data.last_check_deleted_count.map(|v| v.to_string()));
        write_field!("HOSTNAME", data.hostname.clone());
        write_field!("INTERNAL_IP", data.internal_ip.clone());
        write_field!("ARCHITECTURE", data.architecture.clone());
        write_field!("OS_IMAGE", data.os_image.clone());
        write_field!("KERNEL_VERSION", data.kernel_version.clone());
        write_field!("KUBELET_VERSION", data.kubelet_version.clone());
        write_field!("CONTAINER_RUNTIME", data.container_runtime.clone());
        write_field!("OPERATING_SYSTEM", data.operating_system.clone());
        write_field!("CPU_CAPACITY_CORES", data.cpu_capacity_cores.map(|v| v.to_string()));
        write_field!("MEMORY_CAPACITY_BYTES", data.memory_capacity_bytes.map(|v| v.to_string()));
        write_field!("POD_CAPACITY", data.pod_capacity.map(|v| v.to_string()));
        write_field!("EPHEMERAL_STORAGE_CAPACITY_BYTES", data.ephemeral_storage_capacity_bytes.map(|v| v.to_string()));
        write_field!("CPU_ALLOCATABLE_CORES", data.cpu_allocatable_cores.map(|v| v.to_string()));
        write_field!("MEMORY_ALLOCATABLE_BYTES", data.memory_allocatable_bytes.map(|v| v.to_string()));
        write_field!("EPHEMERAL_STORAGE_ALLOCATABLE_BYTES", data.ephemeral_storage_allocatable_bytes.map(|v| v.to_string()));
        write_field!("POD_ALLOCATABLE", data.pod_allocatable.map(|v| v.to_string()));
        write_field!("READY", data.ready.map(|v| v.to_string()));
        write_field!("TAINTS", data.taints.clone());
        write_field!("LABEL", data.label.clone());
        write_field!("ANNOTATION", data.annotation.clone());
        write_field!("IMAGE_COUNT", data.image_count.map(|v| v.to_string()));
        write_field!("IMAGE_NAMES", data.image_names.clone().map(|v| v.join(",")));
        write_field!("IMAGE_TOTAL_SIZE_BYTES", data.image_total_size_bytes.map(|v| v.to_string()));

        f.flush()?;
        fs::rename(&tmp_path, &final_path).context("Failed to finalize node info file")?;
        Ok(())
    }
}
