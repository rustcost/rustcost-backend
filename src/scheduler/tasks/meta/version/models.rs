use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub date: String,
    pub major: String,
    pub minor: String,
    pub git_version: String,
    pub git_commit: String,
    pub build_date: String,
    pub go_version: String,
    pub compiler: String,
    pub platform: String,
}
