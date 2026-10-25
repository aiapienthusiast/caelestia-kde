//! Bar Configuration
//! Parity with `Config.bar` in `shell/modules/bar/`.

use crate::modules::drawers::BarPosition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BarConfig {
    pub position: BarPosition,
    pub thickness: f32,
    pub scale: f32,
    pub show_on_hover: bool,
    pub islands: bool,
    pub island_spacing: f32,
    pub clock_show_seconds: bool,
    pub clock_24h: bool,
    pub workspaces_shown: usize,
    pub workspaces_show_unoccupied: bool,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            position: BarPosition::Top,
            thickness: 48.0,
            scale: 1.0,
            show_on_hover: true,
            islands: true,
            island_spacing: 12.0,
            clock_show_seconds: false,
            clock_24h: true,
            workspaces_shown: 6,
            workspaces_show_unoccupied: true,
        }
    }
}
