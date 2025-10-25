use anyhow::Result;
use tracing::{info, error};

pub async fn run() -> Result<()> {
    info!("Running hourly task (aggregation + summarization)...");

    if let Err(e) = super::processors::hourly::run().await {
        error!(?e, "Hourly aggregator failed");
    }

    Ok(())
}
