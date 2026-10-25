//! Region Selection Geometry & Snapping
//! Complete parity with `shell/modules/screenshot/regionSelector/RegionSelection.qml`.

use crate::services::kwin::WindowItem;
use serde::{Deserialize, Serialize};

/// Shape selection mode for regional capture
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMode {
    RectCorners,
    Circle,
}

impl Default for SelectionMode {
    fn default() -> Self {
        Self::RectCorners
    }
}

/// Normalized bounding box coordinates
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectionRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl SelectionRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn is_valid(&self) -> bool {
        self.width >= 4.0 && self.height >= 4.0
    }
}

/// Active selection region tracking cursor coordinates and snapping
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectionRegion {
    pub mode: SelectionMode,
    pub start_x: f32,
    pub start_y: f32,
    pub current_x: f32,
    pub current_y: f32,
    pub snapped_window_address: Option<String>,
}

impl SelectionRegion {
    pub fn new(start_x: f32, start_y: f32, mode: SelectionMode) -> Self {
        Self {
            mode,
            start_x,
            start_y,
            current_x: start_x,
            current_y: start_y,
            snapped_window_address: None,
        }
    }

    /// Compute normalized rectangle
    pub fn bounds(&self) -> SelectionRect {
        let x = self.start_x.min(self.current_x);
        let y = self.start_y.min(self.current_y);
        let width = (self.current_x - self.start_x).abs();
        let height = (self.current_y - self.start_y).abs();
        SelectionRect { x, y, width, height }
    }

    /// Update drag position
    pub fn update(&mut self, x: f32, y: f32) {
        self.current_x = x;
        self.current_y = y;
        self.snapped_window_address = None;
    }

    /// Snap selection to window boundaries
    pub fn snap_to_window(&mut self, win: &WindowItem) {
        self.start_x = win.x as f32;
        self.start_y = win.y as f32;
        self.current_x = (win.x + win.width as i32) as f32;
        self.current_y = (win.y + win.height as i32) as f32;
        self.snapped_window_address = Some(win.address.clone());
    }
}
