use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Version metadata for RustCost.
///
/// This structure captures all relevant build and source control information
/// for a given RustCost binary. It is typically generated at build time and
/// written to the version information file (`version.rci`).
///
/// The structure supports serialization for inspection via the CLI, API,
/// or diagnostics endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoVersionEntity {
    /// Date when this version record was generated.
    pub date: String,

    /// Major version number (e.g. `"1"`).
    pub major: String,

    /// Minor version number (e.g. `"0"`).
    pub minor: String,

    /// Full Git tag or semantic version (e.g. `"v1.0.0"`).
    pub git_version: String,

    /// Git commit hash corresponding to this build.
    pub git_commit: String,

    /// Timestamp when the binary was built.
    pub build_date: String,

    /// Go toolchain version (for hybrid components).
    pub go_version: String,

    /// Compiler name and version used for building.
    pub compiler: String,

    /// Target platform triple (e.g. `"x86_64-unknown-linux-gnu"`).
    pub platform: String,

    /// UTC timestamp when this file was last parsed or updated.
    pub updated_at: DateTime<Utc>,
}

impl Default for InfoVersionEntity {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            date: String::new(),
            major: "0".into(),
            minor: "0".into(),
            git_version: "v0.0.0".into(),
            git_commit: String::new(),
            build_date: String::new(),
            go_version: String::new(),
            compiler: String::new(),
            platform: String::new(),
            updated_at: now,
        }
    }
}
