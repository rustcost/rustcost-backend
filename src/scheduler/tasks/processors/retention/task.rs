use anyhow::Result;
use crate::scheduler::tasks::processors::retention;

pub async fn run() -> Result<()> {
    retention::pod::task::run().await?;
    retention::node::task::run().await?;
    retention::container::task::run().await?;
    Ok(())
}