//! Interactions Subsystem
//! Complete parity with `shell/modules/drawers/Interactions.qml`.
//!
//! Handles pointer gestures, edge-swipe detection, hover activation, and click-to-dismiss logic.

use super::exclusions::BarPosition;
use serde::{Deserialize, Serialize};

/// Target drawer that can be triggered or interacted with
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrawerTarget {
    Dashboard,
    Launcher,
    Sidebar,
    Session,
    Osd,
    Utilities,
}

/// Gesture drag tracking state
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GestureDrag {
    pub is_dragging: bool,
    pub start_x: f64,
    pub start_y: f64,
    pub current_x: f64,
    pub current_y: f64,
    pub target: Option<DrawerTarget>,
}

impl GestureDrag {
    pub fn delta_x(&self) -> f64 {
        self.current_x - self.start_x
    }

    pub fn delta_y(&self) -> f64 {
        self.current_y - self.start_y
    }
}

/// Interactions configuration matching Interactions.qml
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InteractionsConfig {
    pub screen_width: f64,
    pub screen_height: f64,
    pub bar_position: BarPosition,
    pub bar_thickness: f64,
    pub border_thickness: f64,
    pub hover_thickness: f64,
    pub swipe_threshold: f64,
}

impl Default for InteractionsConfig {
    fn default() -> Self {
        Self {
            screen_width: 1920.0,
            screen_height: 1080.0,
            bar_position: BarPosition::Top,
            bar_thickness: 48.0,
            border_thickness: 8.0,
            hover_thickness: 16.0,
            swipe_threshold: 40.0,
        }
    }
}

/// Gesture and edge interaction processor
pub struct InteractionsTracker {
    pub config: InteractionsConfig,
    pub drag: GestureDrag,
}

impl InteractionsTracker {
    pub fn new(config: InteractionsConfig) -> Self {
        Self {
            config,
            drag: GestureDrag::default(),
        }
    }

    /// Calculate span bounds matching Interactions.qml spanBounds(near, far, span)
    pub fn span_bounds(near: f64, far: f64, span: f64) -> (f64, f64) {
        if span >= 100.0 {
            return (near, far);
        }
        let half = (far - near) * span.max(0.0) / 200.0;
        let centre = (near + far) / 2.0;
        (centre - half, centre + half)
    }

    /// Check if point starts an edge swipe for Dashboard (top edge)
    pub fn is_top_edge_trigger(&self, x: f64, y: f64) -> bool {
        let ht = self.config.hover_thickness;
        let y_bound = if self.config.bar_position == BarPosition::Top {
            self.config.bar_thickness + ht
        } else {
            ht
        };
        y <= y_bound && x >= 0.0 && x <= self.config.screen_width
    }

    /// Check if point starts an edge swipe for Launcher (bottom edge)
    pub fn is_bottom_edge_trigger(&self, x: f64, y: f64) -> bool {
        let ht = self.config.hover_thickness;
        let bottom_bound = if self.config.bar_position == BarPosition::Bottom {
            self.config.screen_height - self.config.bar_thickness - ht
        } else {
            self.config.screen_height - ht
        };
        y >= bottom_bound && x >= 0.0 && x <= self.config.screen_width
    }

    /// Check if point starts an edge swipe for Sidebar (right edge by default, or left)
    pub fn is_sidebar_edge_trigger(&self, x: f64, y: f64) -> bool {
        let ht = self.config.hover_thickness;
        let x_valid = if self.config.bar_position == BarPosition::Right {
            x <= ht
        } else {
            x >= self.config.screen_width - ht
        };
        x_valid && y >= 0.0 && y <= self.config.screen_height
    }

    /// Start a drag gesture
    pub fn start_drag(&mut self, x: f64, y: f64) -> Option<DrawerTarget> {
        let target = if self.is_top_edge_trigger(x, y) {
            Some(DrawerTarget::Dashboard)
        } else if self.is_bottom_edge_trigger(x, y) {
            Some(DrawerTarget::Launcher)
        } else if self.is_sidebar_edge_trigger(x, y) {
            Some(DrawerTarget::Sidebar)
        } else {
            None
        };

        self.drag = GestureDrag {
            is_dragging: target.is_some(),
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            target,
        };

        target
    }

    /// Update drag motion
    pub fn update_drag(&mut self, x: f64, y: f64) {
        if self.drag.is_dragging {
            self.drag.current_x = x;
            self.drag.current_y = y;
        }
    }

    /// Conclude drag and determine if open threshold was achieved
    pub fn end_drag(&mut self) -> Option<(DrawerTarget, bool)> {
        if !self.drag.is_dragging {
            return None;
        }

        let target = self.drag.target?;
        let should_open = match target {
            DrawerTarget::Dashboard => self.drag.delta_y() >= self.config.swipe_threshold,
            DrawerTarget::Launcher => self.drag.delta_y() <= -self.config.swipe_threshold,
            DrawerTarget::Sidebar => {
                if self.config.bar_position == BarPosition::Right {
                    self.drag.delta_x() >= self.config.swipe_threshold
                } else {
                    self.drag.delta_x() <= -self.config.swipe_threshold
                }
            }
            _ => false,
        };

        self.drag.is_dragging = false;
        Some((target, should_open))
    }
}
