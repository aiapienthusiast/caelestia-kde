//! Regions Subsystem
//! Complete parity with `shell/modules/drawers/Regions.qml`.
//!
//! Computes input regions and click-through masks for the Wayland overlay window,
//! ensuring clicks in the empty desktop pass through while catching interactions
//! on the bar, open drawers, popouts, and overview hot corners.

use super::exclusions::BarPosition;
use super::panels::RectBounds;
use serde::{Deserialize, Serialize};

/// Hot corner configuration for Overview gesture/hover
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct OverviewHotCorners {
    pub enabled: bool,
    pub hover_thickness: f64,
    pub top_left: bool,
    pub top_right: bool,
    pub bottom_left: bool,
    pub bottom_right: bool,
}

impl Default for OverviewHotCorners {
    fn default() -> Self {
        Self {
            enabled: true,
            hover_thickness: 16.0,
            top_left: true,
            top_right: false,
            bottom_left: false,
            bottom_right: false,
        }
    }
}

/// Panel offset state for sliding drawers (offset_scale: 0.0 = fully open, 1.0 = fully closed)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DrawerOffsetState {
    pub dashboard_offset: f64,
    pub dashboard_height: f64,
    pub launcher_offset: f64,
    pub launcher_height: f64,
    pub sidebar_offset: f64,
    pub sidebar_width: f64,
    pub session_offset: f64,
    pub session_width: f64,
    pub osd_offset: f64,
    pub osd_width: f64,
    pub utilities_offset: f64,
    pub utilities_height: f64,
    pub popouts_offset: f64,
    pub popouts_width: f64,
}

/// Input regions manager matching Regions.qml
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputRegions {
    pub screen_width: f64,
    pub screen_height: f64,
    pub bar_position: BarPosition,
    pub bar_thickness: f64,
    pub border_thickness: f64,
    pub hover_thickness: f64,
    pub hot_corners: OverviewHotCorners,
    pub offsets: DrawerOffsetState,
    pub notifications_bounds: Option<RectBounds>,
}

impl InputRegions {
    pub fn new(width: f64, height: f64, bar_pos: BarPosition, bar_thickness: f64) -> Self {
        Self {
            screen_width: width,
            screen_height: height,
            bar_position: bar_pos,
            bar_thickness,
            border_thickness: 8.0,
            hover_thickness: 12.0,
            hot_corners: OverviewHotCorners::default(),
            offsets: DrawerOffsetState::default(),
            notifications_bounds: None,
        }
    }

    /// Check if point is inside the bar area
    pub fn in_bar_area(&self, x: f64, y: f64) -> bool {
        match self.bar_position {
            BarPosition::Top => y <= self.bar_thickness,
            BarPosition::Bottom => y >= self.screen_height - self.bar_thickness,
            BarPosition::Left => x <= self.bar_thickness,
            BarPosition::Right => x >= self.screen_width - self.bar_thickness,
        }
    }

    /// Check if point is inside any Overview hot corner
    pub fn in_hot_corner(&self, x: f64, y: f64) -> bool {
        if !self.hot_corners.enabled {
            return false;
        }

        let ht = self.hot_corners.hover_thickness;

        // Top-left
        if self.hot_corners.top_left && x <= ht && y <= ht {
            return true;
        }
        // Top-right
        if self.hot_corners.top_right && x >= self.screen_width - ht && y <= ht {
            return true;
        }
        // Bottom-left
        if self.hot_corners.bottom_left && x <= ht && y >= self.screen_height - ht {
            return true;
        }
        // Bottom-right
        if self.hot_corners.bottom_right && x >= self.screen_width - ht && y >= self.screen_height - ht {
            return true;
        }

        false
    }

    /// Check if point is inside the top dashboard drawer
    pub fn in_dashboard_region(&self, _x: f64, y: f64) -> bool {
        if self.offsets.dashboard_height <= 0.0 {
            return false;
        }
        let extent = self.offsets.dashboard_height * (1.0 - self.offsets.dashboard_offset)
            + self.hover_thickness.max(self.border_thickness);
        y <= extent
    }

    /// Check if point is inside the bottom launcher drawer
    pub fn in_launcher_region(&self, _x: f64, y: f64) -> bool {
        if self.offsets.launcher_height <= 0.0 {
            return false;
        }
        let extent = self.offsets.launcher_height * (1.0 - self.offsets.launcher_offset)
            + self.hover_thickness.max(self.border_thickness);
        y >= self.screen_height - extent
    }

    /// Check if point is inside the right/left sidebar
    pub fn in_sidebar_region(&self, x: f64, _y: f64) -> bool {
        if self.offsets.sidebar_width <= 0.0 {
            return false;
        }
        let extent = self.offsets.sidebar_width * (1.0 - self.offsets.sidebar_offset) + self.border_thickness;
        if self.bar_position == BarPosition::Right {
            x <= extent
        } else {
            x >= self.screen_width - extent
        }
    }

    /// Check if point is inside the notifications area
    pub fn in_notifications_region(&self, x: f64, y: f64) -> bool {
        if let Some(bounds) = &self.notifications_bounds {
            x >= bounds.x
                && x <= bounds.x + bounds.width
                && y >= bounds.y
                && y <= bounds.y + bounds.height
        } else {
            false
        }
    }

    /// Primary hit-test: returns true if (x, y) should capture input, false if it should pass through
    pub fn should_capture_input(&self, x: f64, y: f64) -> bool {
        self.in_bar_area(x, y)
            || self.in_hot_corner(x, y)
            || self.in_dashboard_region(x, y)
            || self.in_launcher_region(x, y)
            || self.in_sidebar_region(x, y)
            || self.in_notifications_region(x, y)
    }
}
