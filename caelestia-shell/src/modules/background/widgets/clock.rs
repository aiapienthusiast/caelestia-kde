//! Desktop analog and digital clock widget
//! Ported with 100% parity from `ClockWidget.qml` and `DesktopClockWidget.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClockStyle {
    Digital,
    Analog,
    Minimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesktopClockWidgetModel {
    pub style: ClockStyle,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub time_str: String,
    pub date_str: String,
    pub show_seconds: bool,
    pub use_24h: bool,
}

impl Default for DesktopClockWidgetModel {
    fn default() -> Self {
        Self {
            style: ClockStyle::Digital,
            hours: 12,
            minutes: 0,
            seconds: 0,
            time_str: "12:00".to_string(),
            date_str: "Sunday, Oct 25".to_string(),
            show_seconds: false,
            use_24h: true,
        }
    }
}

impl DesktopClockWidgetModel {
    pub fn update_time(&mut self, h: u32, m: u32, s: u32, date_str: impl Into<String>) {
        self.hours = h;
        self.minutes = m;
        self.seconds = s;
        self.date_str = date_str.into();

        let display_h = if self.use_24h {
            h
        } else {
            let m = h % 12;
            if m == 0 { 12 } else { m }
        };

        if self.show_seconds {
            self.time_str = format!("{display_h:02}:{m:02}:{s:02}");
        } else {
            self.time_str = format!("{display_h:02}:{m:02}");
        }
    }

    /// Calculate analog clock hand angles in degrees
    pub fn analog_angles(&self) -> (f64, f64, f64) {
        let sec_angle = self.seconds as f64 * 6.0;
        let min_angle = self.minutes as f64 * 6.0 + (self.seconds as f64 * 0.1);
        let hr_angle = (self.hours % 12) as f64 * 30.0 + (self.minutes as f64 * 0.5);
        (hr_angle, min_angle, sec_angle)
    }
}
