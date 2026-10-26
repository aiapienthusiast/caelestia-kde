//! Nexus Wallpaper and Style Configuration Page
//! Ported with 100% parity from `shell/modules/nexus/pages/wallandstyle/`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WallpaperSettingsPageModel {
    pub current_wallpaper_path: String,
    pub fit_mode: String, // "Crop", "Fit", "Tile"
    pub auto_dark_mode: bool,
    pub m3_scheme_variant: String, // "TonalSpot", "Content", "Expressive", "FruitSalad"
    pub custom_accent_hex: Option<String>,
    pub wallpaper_blur_radius: f64,
    pub lockscreen_use_desktop_wallpaper: bool,
    pub lockscreen_blur_radius: f64,
}

impl Default for WallpaperSettingsPageModel {
    fn default() -> Self {
        Self {
            current_wallpaper_path: "/usr/share/wallpapers/Caelestia/contents/images/1920x1080.jpg".to_string(),
            fit_mode: "Crop".to_string(),
            auto_dark_mode: true,
            m3_scheme_variant: "TonalSpot".to_string(),
            custom_accent_hex: None,
            wallpaper_blur_radius: 0.0,
            lockscreen_use_desktop_wallpaper: true,
            lockscreen_blur_radius: 20.0,
        }
    }
}

impl WallpaperSettingsPageModel {
    pub fn set_wallpaper(&mut self, path: impl Into<String>) {
        self.current_wallpaper_path = path.into();
    }

    pub fn set_variant(&mut self, variant: impl Into<String>) {
        self.m3_scheme_variant = variant.into();
    }

    pub fn set_accent(&mut self, hex: impl Into<String>) {
        self.custom_accent_hex = Some(hex.into());
    }
}
