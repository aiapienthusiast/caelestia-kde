//! Dashboard Performance & Media Subviews: BatteryTank, HeroCard, and LyricsSelector
//! Ported with 100% parity from `BatteryTank.qml`, `HeroCard.qml`, and `LyricsAndSelector.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatteryTankModel {
    pub percentage: f64,
    pub is_charging: bool,
    pub is_plugged_in: bool,
    pub power_watts: f64,
    pub time_to_empty_secs: Option<u64>,
    pub time_to_full_secs: Option<u64>,
    pub health_percentage: f64,
    pub temperature_celsius: f64,
    pub fluid_wave_offset: f64,
}

impl Default for BatteryTankModel {
    fn default() -> Self {
        Self {
            percentage: 85.0,
            is_charging: false,
            is_plugged_in: true,
            power_watts: 18.5,
            time_to_empty_secs: Some(18000),
            time_to_full_secs: None,
            health_percentage: 96.0,
            temperature_celsius: 32.5,
            fluid_wave_offset: 0.0,
        }
    }
}

impl BatteryTankModel {
    pub fn formatted_time_remaining(&self) -> String {
        if self.is_charging {
            if let Some(secs) = self.time_to_full_secs {
                let mins = secs / 60;
                let hrs = mins / 60;
                return format!("{hrs}h {}m until full", mins % 60);
            }
            "Charging".to_string()
        } else if let Some(secs) = self.time_to_empty_secs {
            let mins = secs / 60;
            let hrs = mins / 60;
            format!("{hrs}h {}m remaining", mins % 60)
        } else {
            "Calculating...".to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroPerformanceCardModel {
    pub cpu_load: f64,
    pub cpu_freq_ghz: f64,
    pub cpu_temp_celsius: f64,
    pub gpu_load: f64,
    pub gpu_temp_celsius: f64,
    pub gpu_mem_used_mb: u64,
    pub gpu_mem_total_mb: u64,
    pub sparkline_samples: Vec<f64>,
}

impl Default for HeroPerformanceCardModel {
    fn default() -> Self {
        Self {
            cpu_load: 12.0,
            cpu_freq_ghz: 3.4,
            cpu_temp_celsius: 48.0,
            gpu_load: 5.0,
            gpu_temp_celsius: 42.0,
            gpu_mem_used_mb: 512,
            gpu_mem_total_mb: 8192,
            sparkline_samples: vec![10.0, 15.0, 12.0, 18.0, 14.0, 12.0],
        }
    }
}

impl HeroPerformanceCardModel {
    pub fn push_sample(&mut self, sample: f64) {
        self.sparkline_samples.push(sample.clamp(0.0, 100.0));
        if self.sparkline_samples.len() > 30 {
            self.sparkline_samples.remove(0);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LyricsProviderResult {
    pub provider_name: String,
    pub title: String,
    pub artist: String,
    pub is_synced: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LyricsSelectorModel {
    pub search_query: String,
    pub offset_ms: i64,
    pub available_results: Vec<LyricsProviderResult>,
    pub selected_idx: usize,
    pub is_searching: bool,
}

impl Default for LyricsSelectorModel {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            offset_ms: 0,
            available_results: Vec::new(),
            selected_idx: 0,
            is_searching: false,
        }
    }
}

impl LyricsSelectorModel {
    pub fn adjust_offset(&mut self, delta_ms: i64) {
        self.offset_ms += delta_ms;
    }
}
