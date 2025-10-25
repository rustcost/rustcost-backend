use anyhow::Result;
use tracing::{info, error};

pub async fn run() -> Result<()> {
    info!("Running daily task (aggregation + retention)...");

    if let Err(e) = super::processors::daily::run().await {
        error!(?e, "Daily aggregator failed");
    }

    if let Err(e) = super::processors::retention::run().await {
        error!(?e, "Retention cleanup failed");
    }

    Ok(())
}
