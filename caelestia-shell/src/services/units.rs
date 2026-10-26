//! Units & Measurement Formatting Service
//! 100% parity with `shell/services/Units.qml`.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemperatureUnit {
    Celsius = 0,
    Fahrenheit = 1,
    Kelvin = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeedUnit {
    KilometersPerHour,
    MilesPerHour,
    MetersPerSecond,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistanceUnit {
    Meters,
    Kilometers,
    Miles,
}

pub struct UnitsService;

impl UnitsService {
    /// Converts a temperature in Celsius to specified unit
    pub fn to_temperature(celsius: f64, unit: TemperatureUnit) -> f64 {
        match unit {
            TemperatureUnit::Celsius => celsius,
            TemperatureUnit::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
            TemperatureUnit::Kelvin => celsius + 273.15,
        }
    }

    /// Formats temperature with symbol
    pub fn format_temp(celsius: f64, unit: TemperatureUnit, compact: bool) -> String {
        let converted = Self::to_temperature(celsius, unit);
        let rounded = converted.round() as i64;

        if compact {
            match unit {
                TemperatureUnit::Kelvin => format!("{} K", rounded),
                _ => format!("{}°", rounded),
            }
        } else {
            match unit {
                TemperatureUnit::Celsius => format!("{}°C", rounded),
                TemperatureUnit::Fahrenheit => format!("{}°F", rounded),
                TemperatureUnit::Kelvin => format!("{} K", rounded),
            }
        }
    }

    /// Scales raw byte count to human-readable binary or decimal representation
    pub fn format_bytes(bytes: u64, use_binary: bool) -> String {
        let (units, divisor): (&[&str], f64) = if use_binary {
            (&["B", "KiB", "MiB", "GiB", "TiB"], 1024.0)
        } else {
            (&["B", "KB", "MB", "GB", "TB"], 1000.0)
        };

        let mut size = bytes as f64;
        let mut idx = 0;

        while size >= divisor && idx < units.len() - 1 {
            size /= divisor;
            idx += 1;
        }

        if idx == 0 {
            format!("{} {}", bytes, units[0])
        } else {
            format!("{:.1} {}", size, units[idx])
        }
    }

    /// Formats data transfer rate (e.g. "1.2 MiB/s")
    pub fn format_rate(bytes_per_sec: u64, use_binary: bool) -> String {
        let base = Self::format_bytes(bytes_per_sec, use_binary);
        format!("{}/s", base)
    }

    /// Formats speed
    pub fn format_speed(kmh: f64, unit: SpeedUnit) -> String {
        match unit {
            SpeedUnit::KilometersPerHour => format!("{:.1} km/h", kmh),
            SpeedUnit::MilesPerHour => format!("{:.1} mph", kmh * 0.621371),
            SpeedUnit::MetersPerSecond => format!("{:.1} m/s", kmh / 3.6),
        }
    }

    /// Formats human-readable relative time (e.g. "just now", "5m ago", "2h ago", "yesterday")
    pub fn format_relative_time(elapsed_secs: u64) -> String {
        if elapsed_secs < 60 {
            "just now".to_string()
        } else if elapsed_secs < 3600 {
            let mins = elapsed_secs / 60;
            format!("{}m ago", mins)
        } else if elapsed_secs < 86400 {
            let hours = elapsed_secs / 3600;
            format!("{}h ago", hours)
        } else if elapsed_secs < 172800 {
            "yesterday".to_string()
        } else {
            let days = elapsed_secs / 86400;
            format!("{}d ago", days)
        }
    }
}
