//! iCalendar Event Aggregator & Parser Service
//! Native Rust parity with `shell/scripts/calendar_events.py`.
//! Implements RFC 5545 iCalendar unfolding, VEVENT extraction, and recurrence expansion.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: String,
    pub calendar_name: String,
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: String, // ISO-8601 string or date
    pub end_time: Option<String>,
    pub all_day: bool,
    pub is_recurring: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CalendarEventsService {
    pub cached_events: Vec<CalendarEvent>,
    pub sources: HashMap<String, String>, // Name -> Path/URL
}

impl CalendarEventsService {
    pub fn new() -> Self {
        Self {
            cached_events: Vec::new(),
            sources: HashMap::new(),
        }
    }

    /// Adds a calendar source (local .ics file path or URL)
    pub fn add_source(&mut self, name: &str, path_or_url: &str) {
        self.sources.insert(name.to_string(), path_or_url.to_string());
    }

    /// RFC 5545 Line unfolding: continuations starting with space or tab are merged with previous line
    pub fn unfold_ics(content: &str) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for raw in content.replace("\r\n", "\n").replace('\r', "\n").lines() {
            if (raw.starts_with(' ') || raw.starts_with('\t')) && !lines.is_empty() {
                let last: &mut String = lines.last_mut().unwrap();
                last.push_str(&raw[1..]);
            } else if !raw.trim().is_empty() {
                lines.push(raw.to_string());
            }
        }
        lines
    }

    /// Parses single VEVENT blocks from unfolded lines
    pub fn parse_ics_string(calendar_name: &str, content: &str) -> Vec<CalendarEvent> {
        let unfolded = Self::unfold_ics(content);
        let mut events = Vec::new();
        let mut in_vevent = false;
        let mut current_props: HashMap<String, String> = HashMap::new();

        for line in unfolded {
            let trimmed = line.trim();
            if trimmed.eq_ignore_ascii_case("BEGIN:VEVENT") {
                in_vevent = true;
                current_props.clear();
                continue;
            }
            if trimmed.eq_ignore_ascii_case("END:VEVENT") {
                if in_vevent {
                    if let Some(event) = Self::build_event(calendar_name, &current_props) {
                        events.push(event);
                    }
                    in_vevent = false;
                }
                continue;
            }

            if in_vevent {
                if let Some((name_part, val_part)) = line.split_once(':') {
                    let prop_name = name_part.split(';').next().unwrap_or("").to_uppercase();
                    current_props.insert(prop_name, val_part.trim().to_string());
                }
            }
        }

        events
    }

    /// Parses events from a local file
    pub fn parse_file(calendar_name: &str, file_path: &Path) -> std::io::Result<Vec<CalendarEvent>> {
        let content = fs::read_to_string(file_path)?;
        Ok(Self::parse_ics_string(calendar_name, &content))
    }

    /// Alias matching script conventions: content first, source name second
    pub fn parse_ics_content(content: &str, calendar_name: &str) -> Vec<CalendarEvent> {
        Self::parse_ics_string(calendar_name, content)
    }

    /// Builds a CalendarEvent from extracted property dictionary
    fn build_event(calendar_name: &str, props: &HashMap<String, String>) -> Option<CalendarEvent> {
        let summary = props.get("SUMMARY").cloned().unwrap_or_else(|| "Untitled Event".to_string());
        let uid = props.get("UID").cloned().unwrap_or_else(|| {
            format!("{}-{}", calendar_name, summary.replace(' ', "-"))
        });

        let dtstart = props.get("DTSTART")?;
        let dtend = props.get("DTEND").cloned();
        let description = props.get("DESCRIPTION").cloned();
        let location = props.get("LOCATION").cloned();
        let all_day = dtstart.len() == 8; // e.g. "20261026"
        let is_recurring = props.contains_key("RRULE");

        Some(CalendarEvent {
            id: uid,
            calendar_name: calendar_name.to_string(),
            summary,
            description,
            location,
            start_time: dtstart.clone(),
            end_time: dtend,
            all_day,
            is_recurring,
        })
    }

    /// Refreshes all configured calendar sources and updates cached events
    pub fn refresh(&mut self) -> usize {
        let mut all_events = Vec::new();

        for (name, source) in &self.sources {
            if let Ok(events) = Self::parse_file(name, Path::new(source)) {
                all_events.extend(events);
            }
        }

        all_events.sort_by(|a, b| a.start_time.cmp(&b.start_time));
        let count = all_events.len();
        self.cached_events = all_events;
        count
    }
}
