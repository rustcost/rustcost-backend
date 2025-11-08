//! Core path resolution utilities for persistence layer.

use std::{env, path::PathBuf};

/// Returns the base data path, using `RUSTCOST_BASE_PATH` env var if set.
/// Defaults to `data/` if not configured.
pub fn get_rustcost_base_path() -> PathBuf {
    env::var("RUSTCOST_BASE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("data"))
}

// Re-export info path builders from the new module
pub use crate::core::persistence::info::path::{
    info_setting_path,
    info_unit_price_path,
    info_version_path,
};
