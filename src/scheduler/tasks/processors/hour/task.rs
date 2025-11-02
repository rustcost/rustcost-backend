use anyhow::Result;
use tracing::{debug};
use crate::scheduler::tasks::processors::hour::pod::task::process_pod_minute_to_hour;
use crate::scheduler::tasks::processors::hour::node::task::process_node_minute_to_hour;
use crate::scheduler::tasks::processors::hour::container::task::process_container_minute_to_hour;

pub async fn run() -> Result<()> {
    debug!("Running hour aggregation task...");

    process_pod_minute_to_hour().await.expect("TODO: panic message");
    process_container_minute_to_hour().await.expect("TODO: panic message");
    process_node_minute_to_hour().await.expect("TODO: panic message");

    Ok(())
}

