//! String formatting, truncation, and humanization helpers
//! Ported with 100% parity from `shell/utils/Strings.qml`.

pub struct StringUtils;

impl StringUtils {
    /// Truncate text with ellipsis if it exceeds max_chars
    pub fn truncate(text: &str, max_chars: usize) -> String {
        if text.chars().count() <= max_chars {
            text.to_string()
        } else {
            let mut res: String = text.chars().take(max_chars.saturating_sub(1)).collect();
            res.push('…');
            res
        }
    }

    /// Pluralize noun based on count: e.g. "1 item" vs "2 items"
    pub fn pluralize<'a>(count: usize, singular: &'a str, plural: &'a str) -> String {
        if count == 1 {
            format!("{count} {singular}")
        } else {
            format!("{count} {plural}")
        }
    }

    /// Format duration in seconds to "MM:SS" or "HH:MM:SS"
    pub fn format_duration(seconds: u64) -> String {
        let s = seconds % 60;
        let m = (seconds / 60) % 60;
        let h = seconds / 3600;

        if h > 0 {
            format!("{h}:{m:02}:{s:02}")
        } else {
            format!("{m:02}:{s:02}")
        }
    }

    /// Capitalize first letter of string
    pub fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}
