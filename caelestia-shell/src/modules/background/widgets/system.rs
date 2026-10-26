//! Desktop system resource telemetry widget
//! Ported with 100% parity from `SystemWidget.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesktopSystemWidgetModel {
    pub cpu_usage_percent: f64,
    pub ram_usage_percent: f64,
    pub ram_used_gb: f64,
    pub ram_total_gb: f64,
    pub battery_percent: Option<f64>,
    pub is_charging: bool,
    pub disk_usage_percent: f64,
    pub cpu_temp_celsius: Option<f64>,
}

impl Default for DesktopSystemWidgetModel {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            ram_usage_percent: 0.0,
            ram_used_gb: 0.0,
            ram_total_gb: 0.0,
            battery_percent: None,
            is_charging: false,
            disk_usage_percent: 0.0,
            cpu_temp_celsius: None,
        }
    }
}

impl DesktopSystemWidgetModel {
    pub fn update_metrics(
        &mut self,
        cpu: f64,
        ram_used: f64,
        ram_total: f64,
        battery: Option<f64>,
        charging: bool,
        disk: f64,
        temp: Option<f64>,
    ) {
        self.cpu_usage_percent = cpu.clamp(0.0, 100.0);
        self.ram_used_gb = ram_used;
        self.ram_total_gb = ram_total;
        self.ram_usage_percent = if ram_total > 0.0 {
            (ram_used / ram_total * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };
        self.battery_percent = battery.map(|b| b.clamp(0.0, 100.0));
        self.is_charging = charging;
        self.disk_usage_percent = disk.clamp(0.0, 100.0);
        self.cpu_temp_celsius = temp;
    }
}
