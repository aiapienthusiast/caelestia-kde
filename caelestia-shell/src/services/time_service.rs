//! Real-Time Clock & Relative Time Service
//! 100% parity with `shell/services/Time.qml`.

use chrono::{DateTime, Local, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeSnapshot {
    pub formatted_time: String,
    pub formatted_date: String,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub is_pm: bool,
    pub iso_string: String,
}

pub struct TimeService;

impl TimeService {
    /// Produces a snapshot of current system time with configurable 12h/24h and seconds
    pub fn snapshot(twelve_hour: bool, show_seconds: bool) -> TimeSnapshot {
        let now = Local::now();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();
        let is_pm = hour >= 12;

        let time_format = match (twelve_hour, show_seconds) {
            (true, true) => "%I:%M:%S %p",
            (true, false) => "%I:%M %p",
            (false, true) => "%H:%M:%S",
            (false, false) => "%H:%M",
        };

        TimeSnapshot {
            formatted_time: now.format(time_format).to_string(),
            formatted_date: now.format("%A, %B %d, %Y").to_string(),
            hour,
            minute,
            second,
            is_pm,
            iso_string: now.to_rfc3339(),
        }
    }

    /// Formats relative time elapsed from a given timestamp
    pub fn time_ago(past: DateTime<Local>) -> String {
        let now = Local::now();
        let duration = now.signed_duration_since(past);
        let seconds = duration.num_seconds().max(0) as u64;
        super::units::UnitsService::format_relative_time(seconds)
    }
}
