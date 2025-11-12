use chrono::{DateTime, Duration, Utc};
use crate::domain::metric::k8s::common::dto::{MetricGranularity};


/// Determine granularity based on duration between start and end.
pub fn determine_granularity(start: DateTime<Utc>, end: DateTime<Utc>) -> MetricGranularity {
    let diff = end - start;

    if diff < Duration::hours(3) {
        MetricGranularity::Minute
    } else if diff < Duration::days(3) {
        MetricGranularity::Hour
    } else {
        MetricGranularity::Day
    }
}
