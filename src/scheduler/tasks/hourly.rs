use anyhow::Result;
use tracing::{debug, error};

pub async fn run() -> Result<()> {
    debug!("Running hourly task (aggregation + summarization)...");

    if let Err(e) = super::processors::hourly::run().await {
        error!(?e, "Hourly aggregator failed");
    }

    Ok(())
}
