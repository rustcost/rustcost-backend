use anyhow::Result;
use tracing::{debug, error};

pub async fn run() -> Result<()> {
    debug!("Running daily task (aggregation + retention)...");

    if let Err(e) = super::processors::daily::run().await {
        error!(?e, "Daily aggregator failed");
    }

    if let Err(e) = super::processors::retention::run().await {
        error!(?e, "Retention cleanup failed");
    }

    Ok(())
}
