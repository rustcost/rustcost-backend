use std::env;
use dotenvy::dotenv;
use tokio::sync::OnceCell;
use anyhow::Result;

#[derive(Debug)]
pub struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
}

impl Config {
    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Result<Config> {
    dotenv().ok();

    let server_config = ServerConfig {
        host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .unwrap(),
    };

    Ok(Config { server: server_config })
}

pub async fn config() -> &'static Config {
    CONFIG
        .get_or_init(|| async {
            init_config().await.expect("Failed to initialize config")
        })
        .await
}