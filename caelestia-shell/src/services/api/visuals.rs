//! Visual Theming and Display Visuals API Bridge
//! Parity with `shell/services/api/VisualsApi.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use crate::services::brightness::BrightnessService;
use crate::services::night_light::NightLightService;
use crate::services::wallpaper_service::WallpaperService;
use std::sync::{Arc, Mutex, OnceLock};

/// Visuals API providing wallpaper inspection, Material 3 dynamic colours, and night light
#[derive(Clone)]
pub struct VisualsApi {
    pub wallpaper: Arc<Mutex<WallpaperService>>,
    pub night_light: Arc<Mutex<NightLightService>>,
    pub brightness: Arc<Mutex<BrightnessService>>,
}

impl Default for VisualsApi {
    fn default() -> Self {
        Self::new()
    }
}

impl VisualsApi {
    pub fn new() -> Self {
        Self {
            wallpaper: Arc::new(Mutex::new(WallpaperService::default())),
            night_light: Arc::new(Mutex::new(NightLightService::new())),
            brightness: Arc::new(Mutex::new(BrightnessService::new())),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<VisualsApi> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Read currently loaded Material 3 dynamic color scheme
    pub fn colours(&self) -> CaelestiaColours {
        CaelestiaColours::load()
    }

    /// Primary accent color hex
    pub fn accent_color(&self) -> u32 {
        self.colours().primary
    }

    /// Active wallpaper image path
    pub fn active_wallpaper(&self) -> String {
        self.wallpaper
            .lock()
            .map(|w| w.current_wallpaper().to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Set active desktop wallpaper
    pub fn set_wallpaper(&self, path: &str) {
        if let Ok(w) = self.wallpaper.lock() {
            let _ = w.set_wallpaper(path);
        }
    }

    /// Current display brightness level (0 to 100)
    pub fn brightness_percent(&self) -> u32 {
        self.brightness.lock().map_or(100, |b| b.state.current_pct)
    }

    /// Set display brightness percentage (1 to 100)
    pub fn set_brightness(&self, percent: u32) {
        if let Ok(mut b) = self.brightness.lock() {
            b.set_percent(percent);
        }
    }

    /// Check if Night Light (blue light filter) is active
    pub fn night_light_active(&self) -> bool {
        self.night_light.lock().map_or(false, |n| n.state.running)
    }
}
