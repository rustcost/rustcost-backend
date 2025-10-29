use anyhow::Result;
use tracing::{debug, error};

pub async fn run() -> Result<()> {
    debug!("Running minutely task (collectors + summarizers)...");

    // Meta check (safe and fast)
    let meta = super::meta::load_meta_state().await?;
    debug!("Version: {}", meta.version.git_version);
    debug!("Settings: {:?}", meta.settings);


    // --- Collectors ---
    if let Err(e) = super::collectors::k8s::run().await {
        error!(?e, "K8s collector failed");
    }

    if let Err(e) = super::collectors::rustexporter::run().await {
        error!(?e, "RustExporter collector failed");
    }

    Ok(())
}

