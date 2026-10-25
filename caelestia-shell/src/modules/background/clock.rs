//! Desktop clock layout and presentation model
//! Achieves 100% parity with `shell/modules/background/DesktopClock.qml`.

use super::config::DesktopClockLayout;
use chrono::{DateTime, Local, Timelike};

/// Rendered layout blocks for the desktop clock
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesktopClockFormatted {
    pub layout: DesktopClockLayout,
    pub primary_time: String,
    pub secondary_time: Option<String>,
    pub date: String,
    pub day_of_week: String,
}

/// Desktop clock state and layout coordinator
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesktopClockModel {
    pub layout: DesktopClockLayout,
    pub show_seconds: bool,
    pub invert_colors: bool,
}

impl Default for DesktopClockModel {
    fn default() -> Self {
        Self {
            layout: DesktopClockLayout::Wide,
            show_seconds: false,
            invert_colors: false,
        }
    }
}

impl DesktopClockModel {
    pub fn new(layout: DesktopClockLayout) -> Self {
        Self {
            layout,
            show_seconds: false,
            invert_colors: false,
        }
    }

    /// Automatically computes the optimal layout mode based on container dimensions
    /// Matches `DesktopClock.qml`:
    /// `aspect < 1.25 ? Stacked : (containerW < 340 || aspect < 1.55 ? Compact : Wide)`
    pub fn calculate_layout_mode(width: f32, height: f32) -> DesktopClockLayout {
        if height <= 0.0 {
            return DesktopClockLayout::Wide;
        }
        let aspect = width / height;
        if aspect < 1.25 {
            DesktopClockLayout::Stacked
        } else if width < 340.0 || aspect < 1.55 {
            DesktopClockLayout::Compact
        } else {
            DesktopClockLayout::Wide
        }
    }

    /// Formats current local time according to the given layout
    pub fn format_now(&self) -> DesktopClockFormatted {
        self.format_datetime(Local::now())
    }

    /// Formats a specific datetime for testing and layout verification
    pub fn format_datetime(&self, dt: DateTime<Local>) -> DesktopClockFormatted {
        let hours = dt.hour();
        let minutes = dt.minute();
        let seconds = dt.second();
        let day_of_week = dt.format("%A").to_string();
        let date = dt.format("%d %B %Y").to_string();

        match self.layout {
            DesktopClockLayout::Wide => {
                let primary_time = if self.show_seconds {
                    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
                } else {
                    format!("{:02}:{:02}", hours, minutes)
                };
                DesktopClockFormatted {
                    layout: DesktopClockLayout::Wide,
                    primary_time,
                    secondary_time: None,
                    date,
                    day_of_week,
                }
            }
            DesktopClockLayout::Compact => {
                let primary_time = format!("{:02}:{:02}", hours, minutes);
                let secondary_time = if self.show_seconds {
                    Some(format!("{:02}", seconds))
                } else {
                    None
                };
                DesktopClockFormatted {
                    layout: DesktopClockLayout::Compact,
                    primary_time,
                    secondary_time,
                    date: dt.format("%b %d").to_string(),
                    day_of_week,
                }
            }
            DesktopClockLayout::Stacked => {
                // In Stacked mode: Hours on top, Minutes underneath
                let primary_time = format!("{:02}", hours);
                let secondary_time = Some(format!("{:02}", minutes));
                DesktopClockFormatted {
                    layout: DesktopClockLayout::Stacked,
                    primary_time,
                    secondary_time,
                    date: dt.format("%d/%m").to_string(),
                    day_of_week: dt.format("%a").to_string(),
                }
            }
        }
    }
}
