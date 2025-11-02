use super::tasks::{day_task, hour_task, minute_task};
// src/scheduler/schedule.rs
use anyhow::Result;
use chrono::{Timelike, Utc};
use tokio::sync::broadcast;
use tokio::time::{interval, sleep, Duration, MissedTickBehavior};
use tracing::{debug, error, info, warn};
use chrono::{Duration as ChronoDuration};

/// Entry point — start all periodic background tasks.
/// Call this once from your main() function.
pub async fn start_all_tasks(shutdown: broadcast::Receiver<()>) {
    let mut s1 = shutdown.resubscribe();
    let mut s2 = shutdown.resubscribe();
    let mut s3 = shutdown.resubscribe();

    tokio::spawn(async move { run_minute_loop(&mut s1).await });
    tokio::spawn(async move { run_hour_loop(&mut s2).await });
    tokio::spawn(async move { run_day_loop(&mut s3).await });
}

/// Runs every aligned minute (e.g., 12:00:00, 12:01:00 …)
pub async fn run_minute_loop(shutdown: &mut broadcast::Receiver<()>) {
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

/// Runs an hourly loop that fires at HH:00:30 each hour (e.g., 01:00:30, 02:00:30 …)
pub async fn run_hour_loop(shutdown: &mut broadcast::Receiver<()>) {
    align_to_next_hour_plus_30s().await;

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

/// Runs daily at 00:30:30 UTC.
pub async fn run_day_loop(shutdown: &mut broadcast::Receiver<()>) {
    align_to_next_midnight_plus_30m30s().await;

    let mut ticker = interval(Duration::from_secs(86_400)); // 24h
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

/// Aligns to next full hour + 30 seconds
async fn align_to_next_hour_plus_30s() {
    let now = Utc::now();
    let next_hour = now
        .with_minute(0)
        .and_then(|t| t.with_second(30))
        .and_then(|t| t.with_nanosecond(0))
        .map(|t| {
            if t > now {
                t
            } else {
                // if already past HH:00:30 this hour, jump to next hour + 30s
                t + chrono::Duration::hours(1)
            }
        })
        .unwrap();

    let wait = (next_hour - now).to_std().unwrap_or(Duration::from_secs(0));
    info!("Aligning hour job: sleeping {:?} until {}", wait, next_hour);
    sleep(wait).await;
}

/// Sleeps until the next 00:30:30 UTC moment.
async fn align_to_next_midnight_plus_30m30s() {
    let now = Utc::now();

    // Build today's 00:30:30
    let today_target = now
        .with_hour(0)
        .and_then(|t| t.with_minute(30))
        .and_then(|t| t.with_second(30))
        .and_then(|t| t.with_nanosecond(0))
        .unwrap();

    // If already past today's 00:30:30, use tomorrow's
    let target = if now < today_target {
        today_target
    } else {
        today_target + ChronoDuration::days(1)
    };

    let wait = (target - now).to_std().unwrap_or(Duration::from_secs(0));
    info!("Aligning day job: sleeping {:?} until {}", wait, target);
    sleep(wait).await;
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
        debug!(task = name, attempt = i + 1, "Task start");
        match task().await {
            Ok(_) => {
                debug!(task = name, attempt = i + 1, "Task succeeded");
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
