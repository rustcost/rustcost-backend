use anyhow::Result;
use tracing::{debug};
use crate::scheduler::tasks::processors::hourly::pod::task::process_pod_minute_to_hour;
use crate::scheduler::tasks::processors::hourly::node::task::process_node_minute_to_hour;
use crate::scheduler::tasks::processors::hourly::container::task::process_container_minute_to_hour;

pub async fn run() -> Result<()> {
    debug!("Running hourly aggregation task...");

    process_pod_minute_to_hour().await.expect("TODO: panic message");
    process_container_minute_to_hour().await.expect("TODO: panic message");
    process_node_minute_to_hour().await.expect("TODO: panic message");

    Ok(())
}

