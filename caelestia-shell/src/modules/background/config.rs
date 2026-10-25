//! Background Subsystem Configuration
//! Complete parity with `shell/modules/background/` and `Config.background`.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Wallpaper image fit modes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WallpaperFitMode {
    Cover,
    Contain,
    Stretch,
    Center,
    Tile,
}

impl Default for WallpaperFitMode {
    fn default() -> Self {
        Self::Cover
    }
}

/// Desktop clock widget layout modes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DesktopClockLayout {
    Wide,
    Compact,
    Stacked,
}

impl Default for DesktopClockLayout {
    fn default() -> Self {
        Self::Wide
    }
}

/// Configuration options for background wallpaper and desktop widgets
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundConfig {
    pub enabled: bool,
    pub wallpaper_enabled: bool,
    pub wallpaper_path: Option<PathBuf>,
    pub slideshow_enabled: bool,
    pub slideshow_interval_mins: u32,
    pub slideshow_random: bool,
    pub fit_mode: WallpaperFitMode,
    pub desktop_icons_enabled: bool,
    pub desktop_clock_enabled: bool,
    pub desktop_clock_layout: DesktopClockLayout,
}

impl Default for BackgroundConfig {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        Self {
            enabled: true,
            wallpaper_enabled: true,
            wallpaper_path: Some(PathBuf::from(format!("{}/Pictures/Wallpapers/default.png", home))),
            slideshow_enabled: false,
            slideshow_interval_mins: 15,
            slideshow_random: true,
            fit_mode: WallpaperFitMode::Cover,
            desktop_icons_enabled: true,
            desktop_clock_enabled: true,
            desktop_clock_layout: DesktopClockLayout::Wide,
        }
    }
}
