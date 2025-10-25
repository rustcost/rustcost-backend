use anyhow::Result;
use tracing::{info, error};

pub async fn run() -> Result<()> {
    info!("Running minutely task (collectors + summarizers)...");

    // --- Collectors ---
    if let Err(e) = super::collectors::k8s::run().await {
        error!(?e, "K8s collector failed");
    }

    if let Err(e) = super::collectors::rustexporter::run().await {
        error!(?e, "RustExporter collector failed");
    }

    Ok(())
}
