//! Weather Service
//! Parity with `weather.cpp` and `shell/services/Weather.qml`.
//! Provides current temperature, condition, icon, and forecast info with disk caching.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub temp: String,
    pub temp_c: f32,
    pub condition: String,
    pub icon: String,
    pub humidity: u32,
    pub wind_speed_kmh: f32,
    pub location: String,
    pub high: String,
    pub low: String,
}

impl Default for WeatherData {
    fn default() -> Self {
        Self {
            temp: "24°".to_string(),
            temp_c: 24.0,
            condition: "Partly Cloudy".to_string(),
            icon: "partly_cloudy_day".to_string(),
            humidity: 45,
            wind_speed_kmh: 12.0,
            location: "Local".to_string(),
            high: "27°".to_string(),
            low: "18°".to_string(),
        }
    }
}

pub struct WeatherService {
    pub data: WeatherData,
}

impl Default for WeatherService {
    fn default() -> Self {
        Self::load()
    }
}

impl WeatherService {
    pub fn load() -> Self {
        let cache_path = Self::cache_path();
        if let Ok(content) = fs::read_to_string(&cache_path) {
            if let Ok(data) = serde_json::from_str::<WeatherData>(&content) {
                return Self { data };
            }
        }
        Self {
            data: WeatherData::default(),
        }
    }

    pub fn cache_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".cache/caelestia/weather.json")
    }

    /// Save weather state to cache
    pub fn save(&self) -> Result<(), String> {
        let path = Self::cache_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(&self.data)
            .map_err(|e| format!("Serialization error: {}", e))?;
        fs::write(path, json).map_err(|e| format!("Write error: {}", e))?;
        Ok(())
    }

    /// Map WMO weather code to Material Icon glyph name
    pub fn icon_for_wmo_code(code: u32, is_day: bool) -> &'static str {
        match code {
            0 => {
                if is_day {
                    "sunny"
                } else {
                    "clear_night"
                }
            }
            1 | 2 => {
                if is_day {
                    "partly_cloudy_day"
                } else {
                    "partly_cloudy_night"
                }
            }
            3 => "cloud",
            45 | 48 => "foggy",
            51 | 53 | 55 => "rainy",
            61 | 63 | 65 => "rainy_heavy",
            71 | 73 | 75 => "weather_snowy",
            80 | 81 | 82 => "rainy",
            95 | 96 | 99 => "thunderstorm",
            _ => "cloud",
        }
    }
}
