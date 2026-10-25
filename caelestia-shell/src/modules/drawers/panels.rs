//! Panels Subsystem
//! Complete parity with `shell/modules/drawers/Panels.qml`.
//!
//! Manages panel margins, edge anchoring, notification positioning, and popout intersection logic.

use super::exclusions::BarPosition;
use serde::{Deserialize, Serialize};

/// Notification position configuration
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotifPosition {
    Auto,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for NotifPosition {
    fn default() -> Self {
        Self::Auto
    }
}

/// Margin specification for panels container
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PanelMargins {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

/// 2D Bounding Box
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RectBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl RectBounds {
    pub fn intersects(&self, other: &RectBounds) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

/// Configuration for computing panel layout and margins
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PanelsConfig {
    pub bar_position: BarPosition,
    pub bar_width: f64,
    pub bar_height: f64,
    pub border_thickness: f64,
    pub islands: bool,
    pub island_spacing: f64,
    pub notif_position: NotifPosition,
    pub notifs_width: f64,
    pub notifs_height: f64,
    pub sidebar_width: f64,
    pub sidebar_height: f64,
}

impl Default for PanelsConfig {
    fn default() -> Self {
        Self {
            bar_position: BarPosition::Top,
            bar_width: 1920.0,
            bar_height: 48.0,
            border_thickness: 8.0,
            islands: true,
            island_spacing: 16.0,
            notif_position: NotifPosition::Auto,
            notifs_width: 360.0,
            notifs_height: 0.0,
            sidebar_width: 380.0,
            sidebar_height: 900.0,
        }
    }
}

/// Computed panel layout state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PanelsLayout {
    pub margins: PanelMargins,
    pub resolved_notif_position: NotifPosition,
    pub notif_at_top: bool,
    pub notif_at_bottom: bool,
    pub notif_reserved_height: f64,
}

impl PanelsLayout {
    pub fn compute(config: &PanelsConfig) -> Self {
        let island_extra = if config.islands {
            config.island_spacing
        } else {
            0.0
        };

        let left = if config.bar_position == BarPosition::Left {
            config.bar_width + (island_extra * 2.0)
        } else {
            config.border_thickness + island_extra
        };

        let right = if config.bar_position == BarPosition::Right {
            config.bar_width + (island_extra * 2.0)
        } else {
            config.border_thickness + island_extra
        };

        let top = if config.bar_position == BarPosition::Top {
            config.bar_height + (island_extra * 2.0)
        } else {
            config.border_thickness + island_extra
        };

        let bottom = if config.bar_position == BarPosition::Bottom {
            config.bar_height + (island_extra * 2.0)
        } else {
            config.border_thickness + island_extra
        };

        let resolved_notif = match config.notif_position {
            NotifPosition::Auto => match config.bar_position {
                BarPosition::Bottom => NotifPosition::BottomRight,
                BarPosition::Right => NotifPosition::TopLeft,
                _ => NotifPosition::TopRight,
            },
            other => other,
        };

        let notif_at_top = matches!(
            resolved_notif,
            NotifPosition::TopLeft | NotifPosition::TopRight
        );
        let notif_at_bottom = matches!(
            resolved_notif,
            NotifPosition::BottomLeft | NotifPosition::BottomRight
        );
        let notif_reserved_height = if config.notifs_height > 0.0 {
            config.notifs_height + config.island_spacing
        } else {
            0.0
        };

        Self {
            margins: PanelMargins {
                left,
                right,
                top,
                bottom,
            },
            resolved_notif_position: resolved_notif,
            notif_at_top,
            notif_at_bottom,
            notif_reserved_height,
        }
    }

    /// Check if a bar popout intersects the right-side notification area
    pub fn popout_intersects_right(
        bar_is_horizontal: bool,
        popout: &RectBounds,
        popout_visible: bool,
        popout_offset_scale: f64,
        notif_bounds: &RectBounds,
    ) -> bool {
        if !popout_visible || popout_offset_scale >= 1.0 {
            return false;
        }

        if bar_is_horizontal {
            let notif_left = notif_bounds.x;
            let notif_right = notif_left + notif_bounds.width;
            let pop_left = popout.x;
            let pop_right = pop_left + popout.width;
            pop_left < notif_right && pop_right > notif_left
        } else {
            let notif_top = notif_bounds.y;
            let notif_bottom = notif_top + notif_bounds.height;
            let pop_top = popout.y;
            let pop_bottom = pop_top + popout.height;
            pop_top < notif_bottom && pop_bottom > notif_top
        }
    }

    /// Check if a bar popout intersects the sidebar
    pub fn popout_intersects_sidebar(
        bar_is_horizontal: bool,
        popout: &RectBounds,
        popout_visible: bool,
        popout_offset_scale: f64,
        sidebar: &RectBounds,
        sidebar_visible: bool,
    ) -> bool {
        if !popout_visible || popout_offset_scale >= 1.0 || !sidebar_visible {
            return false;
        }

        if bar_is_horizontal {
            let side_left = sidebar.x;
            let side_right = side_left + sidebar.width;
            let pop_left = popout.x;
            let pop_right = pop_left + popout.width;
            pop_left < side_right && pop_right > side_left
        } else {
            let side_top = sidebar.y;
            let side_bottom = side_top + sidebar.height;
            let pop_top = popout.y;
            let pop_bottom = pop_top + popout.height;
            pop_top < side_bottom && pop_bottom > side_top
        }
    }
}
