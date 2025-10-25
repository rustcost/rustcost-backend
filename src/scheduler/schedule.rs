// src/scheduler/schedule.rs
use anyhow::Result;
use chrono::{Timelike, Utc};
use tokio::sync::broadcast;
use tokio::time::{sleep, interval, Duration, MissedTickBehavior};
use tracing::{error, info, warn};

use super::tasks::{minute_task, hour_task, day_task};

/// Entry point — start all periodic background tasks.
/// Call this once from your main() function.
pub async fn start_all_tasks(mut shutdown: broadcast::Receiver<()>) {
    let mut s1 = shutdown.resubscribe();
    let mut s2 = shutdown.resubscribe();
    let mut s3 = shutdown.resubscribe();

    tokio::spawn(async move { run_minute_loop(&mut s1).await });
    tokio::spawn(async move { run_hour_loop(&mut s2).await });
    tokio::spawn(async move { run_day_loop(&mut s3).await });
}

/// Runs every aligned minute (e.g., 12:00:00, 12:01:00 …)
async fn run_minute_loop(shutdown: &mut broadcast::Receiver<()>) {
    align_to_next_minute().await;
    let mut ticker = interval(Duration::from_secs(60));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if let Err(e) = retry_task("minute", minute_task).await {
                    error!(?e, "minute_task failed");
                }
            }
            _ = shutdown.recv() => {
                info!("Minute loop shutting down");
                break;
            }
        }
    }
}

/// Runs every aligned hour (e.g., 01:00:00, 02:00:00 …)
async fn run_hour_loop(shutdown: &mut broadcast::Receiver<()>) {
    align_to_next_hour().await;
    let mut ticker = interval(Duration::from_secs(3600));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if let Err(e) = retry_task("hour", hour_task).await {
                    error!(?e, "hour_task failed");
                }
            }
            _ = shutdown.recv() => {
                info!("Hour loop shutting down");
                break;
            }
        }
    }
}

/// Runs daily at 00:00 UTC (midnight)
async fn run_day_loop(shutdown: &mut broadcast::Receiver<()>) {
    align_to_next_midnight().await;
    let mut ticker = interval(Duration::from_secs(86_400));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if let Err(e) = retry_task("day", day_task).await {
                    error!(?e, "day_task failed");
                }
            }
            _ = shutdown.recv() => {
                info!("Day loop shutting down");
                break;
            }
        }
    }
}

//
// Alignment helpers
//

async fn align_to_next_minute() {
    let now = Utc::now();
    let wait = 60 - now.second() as u64;
    if wait < 60 {
        info!(wait_sec = wait, "Aligning to next minute");
        sleep(Duration::from_secs(wait)).await;
    }
}

async fn align_to_next_hour() {
    let now = Utc::now();
    let secs_until_next_hour = 3600 - (now.minute() as u64 * 60 + now.second() as u64);
    info!(wait_sec = secs_until_next_hour, "Aligning to next hour");
    sleep(Duration::from_secs(secs_until_next_hour)).await;
}

async fn align_to_next_midnight() {
    let now = Utc::now();
    let secs_until_next_day =
        24 * 3600 - (now.hour() as u64 * 3600 + now.minute() as u64 * 60 + now.second() as u64);
    info!(wait_sec = secs_until_next_day, "Aligning to midnight");
    sleep(Duration::from_secs(secs_until_next_day)).await;
}

//
// Retry wrapper with simple backoff
//

async fn retry_task<Fut, F>(name: &str, task: F) -> Result<()>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<()>> + Send,
{
    let retries = [1u64, 3, 10]; // seconds between retries
    for (i, delay) in retries.iter().enumerate() {
        info!(task = name, attempt = i + 1, "Task start");
        match task().await {
            Ok(_) => {
                info!(task = name, attempt = i + 1, "Task succeeded");
                return Ok(());
            }
            Err(e) => {
                warn!(task = name, attempt = i + 1, ?e, "Task failed");
                if i < retries.len() - 1 {
                    sleep(Duration::from_secs(*delay)).await;
                } else {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}
