//! Brightness Service
//! Parity with `shell/services/Brightness.qml` and `brightnesswatcher.cpp`.
//! Manages display backlight brightness via direct sysfs reads and brightnessctl.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug, PartialEq)]
pub struct BrightnessState {
    pub current_pct: u32,
    pub max_val: u32,
    pub current_val: u32,
    pub device: String,
}

impl Default for BrightnessState {
    fn default() -> Self {
        Self {
            current_pct: 100,
            max_val: 100,
            current_val: 100,
            device: "default".to_string(),
        }
    }
}

pub struct BrightnessService {
    pub state: BrightnessState,
    sysfs_path: Option<PathBuf>,
}

impl BrightnessService {
    pub fn new() -> Self {
        let sysfs_path = Self::find_backlight_device();
        let mut service = Self {
            state: BrightnessState::default(),
            sysfs_path,
        };
        service.poll();
        service
    }

    /// Locate backlight device in /sys/class/backlight
    fn find_backlight_device() -> Option<PathBuf> {
        let backlight_dir = Path::new("/sys/class/backlight");
        if let Ok(entries) = fs::read_dir(backlight_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.join("brightness").exists() && path.join("max_brightness").exists() {
                    return Some(path);
                }
            }
        }
        None
    }

    /// Refresh current brightness from sysfs or brightnessctl
    pub fn poll(&mut self) {
        if let Some(ref path) = self.sysfs_path {
            if let (Ok(curr_str), Ok(max_str)) = (
                fs::read_to_string(path.join("brightness")),
                fs::read_to_string(path.join("max_brightness")),
            ) {
                if let (Ok(curr), Ok(max)) = (curr_str.trim().parse::<u32>(), max_str.trim().parse::<u32>()) {
                    if max > 0 {
                        self.state.current_val = curr;
                        self.state.max_val = max;
                        self.state.current_pct = ((curr as f32 / max as f32) * 100.0).round() as u32;
                        self.state.device = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                        return;
                    }
                }
            }
        }

        // Fallback to brightnessctl -m
        if let Ok(output) = Command::new("brightnessctl").arg("-m").output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                if let Some(parsed) = Self::parse_brightnessctl_output(&text) {
                    self.state = parsed;
                }
            }
        }
    }

    /// Parse machine-readable output from `brightnessctl -m`
    /// e.g. "intel_backlight,backlight,19200,100%,19200"
    pub fn parse_brightnessctl_output(output: &str) -> Option<BrightnessState> {
        let first_line = output.lines().next()?.trim();
        let parts: Vec<&str> = first_line.split(',').collect();
        if parts.len() < 5 {
            return None;
        }

        let device = parts[0].to_string();
        let current_val = parts[2].parse::<u32>().ok()?;
        let pct_str = parts[3].trim_end_matches('%');
        let current_pct = pct_str.parse::<u32>().ok()?;
        let max_val = parts[4].parse::<u32>().ok()?;

        Some(BrightnessState {
            current_pct,
            max_val,
            current_val,
            device,
        })
    }

    /// Set brightness percentage (1% to 100%, clamped to prevent black screen)
    pub fn set_percent(&mut self, pct: u32) {
        let clamped = pct.clamp(1, 100);
        self.state.current_pct = clamped;
        let _ = Command::new("brightnessctl")
            .args(["set", &format!("{}%", clamped)])
            .spawn();
    }

    /// Step brightness by delta percentage, e.g. +5 or -5
    pub fn step(&mut self, delta: i32) {
        let new_pct = (self.state.current_pct as i32 + delta).clamp(1, 100) as u32;
        self.set_percent(new_pct);
    }
}
