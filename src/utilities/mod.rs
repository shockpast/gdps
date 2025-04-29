pub mod crypto;
pub mod database;
pub mod gd;

use chrono::{DateTime, Utc};
use rand::Rng;

pub fn make_time(timestamp: i64) -> String {
    let date = DateTime::<Utc>::from_timestamp(timestamp, 0)
        .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).unwrap());
    let now = Utc::now();

    let duration = now.signed_duration_since(date);
    let total_seconds = duration.num_seconds();

    let years = duration.num_days() / 365;
    let months = (duration.num_days() % 365) / 30;
    let days = (duration.num_days() % 365) % 30;
    let hours = (total_seconds % 86_400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if years > 0 {
        return format!("{} year{}", years, if years == 1 { "" } else { "s" });
    }
    if months > 0 {
        return format!("{} month{}", months, if months == 1 { "" } else { "s" });
    }
    if days > 0 {
        return format!("{} day{}", days, if days == 1 { "" } else { "s" });
    }
    if hours > 0 {
        return format!("{} hour{}", hours, if hours == 1 { "" } else { "s" });
    }
    if minutes > 0 {
        return format!("{} minute{}", minutes, if minutes == 1 { "" } else { "s" });
    }

    format!("{} second{}", seconds, if seconds == 1 { "" } else { "s" })
}

pub fn rand_ascii(length: usize) -> String {
    rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
