//! Clock Bar Widget
//! Complete parity with `shell/modules/bar/components/Clock.qml`.

use chrono::{Local, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClockSnapshot {
    pub time_str: String,
    pub date_str: String,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

pub struct BarClockModel;

impl BarClockModel {
    pub fn current(use_24h: bool, show_seconds: bool) -> ClockSnapshot {
        let now = Local::now();
        let hours = now.hour();
        let minutes = now.minute();
        let seconds = now.second();

        let time_str = if use_24h {
            if show_seconds {
                format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
            } else {
                format!("{:02}:{:02}", hours, minutes)
            }
        } else {
            let (is_pm, h12) = now.hour12();
            let period = if is_pm { "PM" } else { "AM" };
            if show_seconds {
                format!("{:02}:{:02}:{:02} {}", h12, minutes, seconds, period)
            } else {
                format!("{:02}:{:02} {}", h12, minutes, period)
            }
        };

        let date_str = now.format("%a, %b %e").to_string();

        ClockSnapshot {
            time_str,
            date_str,
            hours,
            minutes,
            seconds,
        }
    }
}
