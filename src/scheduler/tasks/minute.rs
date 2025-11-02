use anyhow::Result;
use tracing::{debug, error};

pub async fn run() -> Result<()> {
    debug!("Running minutely task (collectors + summarizers)...");

    // Info check (safe and fast)
    let info = super::info::load_info_state().await?;
    debug!("Version: {}", info.version.git_version);
    debug!("Settings: {:?}", info.settings);


    // --- Collectors ---
    if let Err(e) = super::collectors::k8s::run().await {
        error!(?e, "K8s collector failed");
    }

    if let Err(e) = super::collectors::rustexporter::run().await {
        error!(?e, "RustExporter collector failed");
    }

    Ok(())
}

