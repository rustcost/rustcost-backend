use std::{env, fs, path::Path};
use std::path::PathBuf;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use crate::core::persistence::storage_path::get_rustcost_base_path;

pub fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    // Read log directory from env or fallback

    let rustcost_log_dir = env::var("RUSTCOST_LOG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| get_rustcost_base_path().join("logs"));

    let rustcost_log_level = env::var("RUSTCOST_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    if !Path::new(&rustcost_log_dir).exists() {
        fs::create_dir_all(&rustcost_log_dir).expect("Failed to create log directory");
    }

    // Daily rotation
    let file_appender = rolling::daily(&rustcost_log_dir, "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let fmt_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_target(false)
        .with_level(true)
        .with_ansi(false);

    let filter_layer = EnvFilter::new(rustcost_log_level);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    tracing::info!(
        "✅ Tracing initialized — daily logs in {}/app.log.YYYY-MM-DD",
        rustcost_log_dir.display()
    );

    guard
}
