use serde::{Deserialize, Serialize};

/// Full /stats/summary response
#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub node: NodeSummary,
    pub pods: Option<Vec<PodSummary>>,
}

/* ---------------- Node Level ---------------- */

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSummary {
    pub node_name: String,
    pub start_time: String,
    pub system_containers: Option<Vec<SystemContainer>>,
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub network: Option<NetworkStats>,
    pub fs: Option<FsStats>,
    pub runtime: Option<RuntimeFs>,
    pub rlimit: Option<Rlimit>,
    pub swap: Option<SwapStats>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemContainer {
    pub name: String,
    pub start_time: String,
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub swap: Option<SwapStats>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuStats {
    pub time: String,
    pub usage_nano_cores: Option<u64>,
    pub usage_core_nano_seconds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryStats {
    pub time: String,
    pub available_bytes: Option<u64>,
    pub usage_bytes: Option<u64>, // ✅ make optional — some entries omit it
    pub working_set_bytes: Option<u64>,
    pub rss_bytes: Option<u64>,
    pub page_faults: Option<u64>,
    pub major_page_faults: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapStats {
    pub time: String,
    pub swap_available_bytes: Option<u64>,
    pub swap_usage_bytes: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsStats {
    pub time: Option<String>,
    pub available_bytes: Option<u64>,
    pub capacity_bytes: Option<u64>,
    pub used_bytes: Option<u64>,
    pub inodes_free: Option<u64>,
    pub inodes: Option<u64>,
    pub inodes_used: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeFs {
    pub image_fs: Option<FsStats>,
    pub container_fs: Option<FsStats>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlimit {
    pub time: String,
    pub maxpid: u64,
    pub curproc: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkStats {
    pub time: String,
    pub name: Option<String>,
    pub rx_bytes: Option<u64>,
    pub rx_errors: Option<u64>,
    pub tx_bytes: Option<u64>,
    pub tx_errors: Option<u64>,
    pub interfaces: Option<Vec<NetworkInterface>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterface {
    pub name: String,
    pub rx_bytes: Option<u64>,
    pub rx_errors: Option<u64>,
    pub tx_bytes: Option<u64>,
    pub tx_errors: Option<u64>,
}

/* ---------------- Pod Level ---------------- */

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSummary {
    pub pod_ref: PodRef,
    pub start_time: String,
    pub containers: Vec<ContainerSummary>,
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub network: Option<NetworkStats>,

    #[serde(rename = "ephemeral-storage")]
    pub ephemeral_storage: Option<FsStats>,

    // ✅ Some pods have no volumes
    pub volume: Option<Vec<VolumeStats>>,

    pub process_stats: Option<ProcessStats>,
    pub swap: Option<SwapStats>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodRef {
    pub name: String,
    pub namespace: String,
    pub uid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerSummary {
    pub name: String,
    pub start_time: String,
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub rootfs: Option<FsStats>,
    pub logs: Option<FsStats>,
    pub swap: Option<SwapStats>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeStats {
    pub time: Option<String>,
    pub available_bytes: Option<u64>,
    pub capacity_bytes: Option<u64>,
    pub used_bytes: Option<u64>,
    pub inodes_free: Option<u64>,
    pub inodes: Option<u64>,
    pub inodes_used: Option<u64>,
    pub name: String,
    pub pvc_ref: Option<PvcRef>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PvcRef {
    pub name: Option<String>,
    pub namespace: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStats {
    pub process_count: Option<u64>, // ✅ your JSON has "null"
}
