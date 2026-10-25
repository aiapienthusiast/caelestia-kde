//! Battery Monitor Daemon & Warning Trigger
//! Parity with `shell/modules/BatteryMonitor.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatteryWarnLevel {
    pub level: u8,
    pub warned: bool,
    pub title: String,
    pub message: String,
    pub icon: String,
    pub critical: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatteryToastEvent {
    ChargerPlugged,
    ChargerUnplugged,
    WarnLevelTriggered {
        title: String,
        message: String,
        icon: String,
        critical: bool,
    },
}

#[derive(Debug, Clone)]
pub struct BatteryMonitor {
    pub settled: bool,
    pub on_battery: bool,
    pub last_percentage: u8,
    pub warn_levels: Vec<BatteryWarnLevel>,
}

impl Default for BatteryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl BatteryMonitor {
    pub fn new() -> Self {
        Self {
            settled: false,
            on_battery: false,
            last_percentage: 100,
            warn_levels: vec![
                BatteryWarnLevel {
                    level: 20,
                    warned: false,
                    title: "Battery low".to_string(),
                    message: "20% battery remaining".to_string(),
                    icon: "battery_android_alert".to_string(),
                    critical: false,
                },
                BatteryWarnLevel {
                    level: 10,
                    warned: false,
                    title: "Battery critical".to_string(),
                    message: "10% battery remaining, plug in charger".to_string(),
                    icon: "battery_android_alert".to_string(),
                    critical: true,
                },
            ],
        }
    }

    pub fn set_settled(&mut self, settled: bool) {
        self.settled = settled;
    }

    /// Reacts to on_battery changes and produces notification events.
    pub fn update_power_state(&mut self, on_battery: bool, notify_charging: bool) -> Option<BatteryToastEvent> {
        let changed = self.on_battery != on_battery;
        self.on_battery = on_battery;

        if !self.on_battery {
            // Reset warnings when charging
            for w in &mut self.warn_levels {
                w.warned = false;
            }
        }

        if changed && self.settled && notify_charging {
            if on_battery {
                Some(BatteryToastEvent::ChargerUnplugged)
            } else {
                Some(BatteryToastEvent::ChargerPlugged)
            }
        } else {
            None
        }
    }

    /// Reacts to battery percentage drops and returns warning events when crossing thresholds.
    pub fn update_percentage(&mut self, percentage: u8) -> Vec<BatteryToastEvent> {
        self.last_percentage = percentage;
        let mut events = Vec::new();

        if !self.on_battery {
            return events;
        }

        for w in &mut self.warn_levels {
            if percentage <= w.level && !w.warned {
                w.warned = true;
                events.push(BatteryToastEvent::WarnLevelTriggered {
                    title: w.title.clone(),
                    message: w.message.clone(),
                    icon: w.icon.clone(),
                    critical: w.critical,
                });
            }
        }

        events
    }
}
