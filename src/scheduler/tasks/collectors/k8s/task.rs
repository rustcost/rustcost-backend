use anyhow::Result;

pub async fn run() -> Result<String> {
    tracing::info!("TASK TEST");
    // placeholder logic
    Ok("HELLO".to_string())
}

#[cfg(test)]
mod tests {
    // Import everything from the parent module
    use super::*;

    // `tokio::test` allows async tests
    #[tokio::test]
    async fn test_run_returns_hello() {
        let result = run().await.unwrap();
        assert_eq!(result, "HELLO");
    }
}