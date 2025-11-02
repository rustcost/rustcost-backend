use anyhow::Result;
use tracing::{debug, error};

pub async fn run() -> Result<()> {
    debug!("Running hour task (aggregation + summarization)...");

    if let Err(e) = super::processors::hour::run().await {
        error!(?e, "hour aggregator failed");
    }

    Ok(())
}
