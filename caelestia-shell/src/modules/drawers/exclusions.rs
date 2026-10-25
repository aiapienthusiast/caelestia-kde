//! Exclusions Subsystem
//! Complete parity with `shell/modules/drawers/Exclusions.qml`.
//!
//! Calculates screen margins, struts, and exclusive zones for Wayland layer-shell,
//! ensuring regular tiled and maximised windows do not overlap the bar or pinned sidebar.

use serde::{Deserialize, Serialize};

/// Edge position for the bar
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for BarPosition {
    fn default() -> Self {
        Self::Top
    }
}

/// 2D Rectangle for usable workspace area
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsableArea {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Configuration inputs for computing exclusion zones
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExclusionConfig {
    pub screen_width: f64,
    pub screen_height: f64,
    pub bar_position: BarPosition,
    pub bar_exclusive_zone: f64,
    pub border_thickness: f64,
    pub sidebar_enabled: bool,
    pub sidebar_pinned: bool,
    pub sidebar_visible: bool,
    pub sidebar_suspended: bool,
    pub sidebar_width: f64,
}

impl Default for ExclusionConfig {
    fn default() -> Self {
        Self {
            screen_width: 1920.0,
            screen_height: 1080.0,
            bar_position: BarPosition::Top,
            bar_exclusive_zone: 48.0,
            border_thickness: 8.0,
            sidebar_enabled: true,
            sidebar_pinned: false,
            sidebar_visible: false,
            sidebar_suspended: false,
            sidebar_width: 320.0,
        }
    }
}

/// Computed exclusive zones and usable desktop area
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExclusionZones {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub usable_area: UsableArea,
}

impl ExclusionZones {
    /// Compute exclusive zones matching Exclusions.qml logic exactly
    pub fn compute(config: &ExclusionConfig) -> Self {
        let reserve_sidebar = config.sidebar_pinned
            && config.sidebar_enabled
            && (config.sidebar_visible || config.sidebar_suspended);

        let left = if config.bar_position == BarPosition::Left {
            config.bar_exclusive_zone
        } else {
            config.border_thickness
        } + if reserve_sidebar && config.bar_position == BarPosition::Right {
            config.sidebar_width
        } else {
            0.0
        };

        let top = if config.bar_position == BarPosition::Top {
            config.bar_exclusive_zone
        } else {
            config.border_thickness
        };

        let right = if config.bar_position == BarPosition::Right {
            config.bar_exclusive_zone
        } else {
            config.border_thickness
        } + if reserve_sidebar && config.bar_position != BarPosition::Right {
            config.sidebar_width
        } else {
            0.0
        };

        let bottom = if config.bar_position == BarPosition::Bottom {
            config.bar_exclusive_zone
        } else {
            config.border_thickness
        };

        let usable_width = (config.screen_width - left - right).max(100.0);
        let usable_height = (config.screen_height - top - bottom).max(100.0);

        Self {
            left,
            top,
            right,
            bottom,
            usable_area: UsableArea {
                x: left,
                y: top,
                width: usable_width,
                height: usable_height,
            },
        }
    }
}
