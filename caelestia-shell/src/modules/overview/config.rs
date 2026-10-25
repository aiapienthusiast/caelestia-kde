//! Overview Subsystem Configuration
//! Complete parity with `shell/modules/overview/` and `Config.overview`.

use serde::{Deserialize, Serialize};

/// Overview window grid layout algorithm
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OverviewLayoutType {
    /// Layered dynamic packing (KDE style)
    Kde = 0,
    /// Aspect-ratio scaling row-based packing (GNOME style)
    Gnome = 1,
}

impl Default for OverviewLayoutType {
    fn default() -> Self {
        Self::Kde
    }
}

/// Overview module configuration options
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverviewConfig {
    pub layout_type: OverviewLayoutType,
    pub hover_scale: f32,
    pub edge_band: f32,
    pub column_spacing: f32,
    pub row_spacing: f32,
    pub vim_keybinds: bool,
    pub occupied_bg: bool,
    pub border_thickness_ratio: f32,
}

impl Default for OverviewConfig {
    fn default() -> Self {
        Self {
            layout_type: OverviewLayoutType::Kde,
            hover_scale: 1.02,
            edge_band: 140.0,
            column_spacing: 16.0,
            row_spacing: 16.0,
            vim_keybinds: true,
            occupied_bg: true,
            border_thickness_ratio: 0.15,
        }
    }
}
