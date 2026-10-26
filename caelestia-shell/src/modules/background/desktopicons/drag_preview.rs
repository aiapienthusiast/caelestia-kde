//! Drag preview and drop menu interaction models
//! Ported with 100% parity from `DragPreview.qml` and `DropMenu.qml`.

use serde::{Deserialize, Serialize};
use super::layout_engine::GridPos;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DragPreviewState {
    pub is_dragging: bool,
    pub moving_keys: Vec<String>,
    pub anchor_key: String,
    pub current_x: f64,
    pub current_y: f64,
    pub target_grid_pos: GridPos,
    pub is_valid_drop: bool,
}

impl Default for DragPreviewState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            moving_keys: Vec::new(),
            anchor_key: String::new(),
            current_x: 0.0,
            current_y: 0.0,
            target_grid_pos: GridPos::new(0, 0),
            is_valid_drop: false,
        }
    }
}

impl DragPreviewState {
    pub fn start_drag(moving_keys: Vec<String>, anchor_key: String, start_pos: GridPos) -> Self {
        Self {
            is_dragging: true,
            moving_keys,
            anchor_key,
            current_x: 0.0,
            current_y: 0.0,
            target_grid_pos: start_pos,
            is_valid_drop: true,
        }
    }

    pub fn update_cursor(&mut self, x: f64, y: f64, target_pos: GridPos, is_valid: bool) {
        self.current_x = x;
        self.current_y = y;
        self.target_grid_pos = target_pos;
        self.is_valid_drop = is_valid;
    }

    pub fn end_drag(&mut self) {
        self.is_dragging = false;
        self.moving_keys.clear();
        self.anchor_key.clear();
    }
}

/// Action chosen from the drop menu when dragging files from external sources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DropAction {
    CopyHere,
    MoveHere,
    LinkHere,
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropMenuModel {
    pub is_visible: bool,
    pub position_x: f64,
    pub position_y: f64,
    pub target_pos: GridPos,
    pub source_uris: Vec<String>,
}

impl Default for DropMenuModel {
    fn default() -> Self {
        Self {
            is_visible: false,
            position_x: 0.0,
            position_y: 0.0,
            target_pos: GridPos::new(0, 0),
            source_uris: Vec::new(),
        }
    }
}

impl DropMenuModel {
    pub fn trigger(&mut self, x: f64, y: f64, target_pos: GridPos, uris: Vec<String>) {
        self.is_visible = true;
        self.position_x = x;
        self.position_y = y;
        self.target_pos = target_pos;
        self.source_uris = uris;
    }

    pub fn dismiss(&mut self) {
        self.is_visible = false;
        self.source_uris.clear();
    }
}
