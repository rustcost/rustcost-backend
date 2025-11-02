pub mod collectors;
pub mod processors;
mod minute;
mod hour;
mod day;
mod info;

pub use day::run as day_task;
pub use hour::run as hour_task;
pub use minute::run as minute_task;

