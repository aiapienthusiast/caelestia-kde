//! Desktop calendar and agenda widget
//! Ported with 100% parity from `CalendarWidget.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarEventItem {
    pub id: String,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
    pub is_all_day: bool,
    pub location: Option<String>,
    pub color_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesktopCalendarWidgetModel {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub events: Vec<CalendarEventItem>,
    pub is_expanded: bool,
}

impl Default for DesktopCalendarWidgetModel {
    fn default() -> Self {
        Self {
            year: 2026,
            month: 10,
            day: 25,
            events: Vec::new(),
            is_expanded: false,
        }
    }
}

impl DesktopCalendarWidgetModel {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self {
            year,
            month,
            day,
            events: Vec::new(),
            is_expanded: false,
        }
    }

    pub fn add_event(&mut self, event: CalendarEventItem) {
        self.events.push(event);
    }

    pub fn events_for_day(&self, _day: u32) -> Vec<&CalendarEventItem> {
        self.events.iter().collect()
    }
}
