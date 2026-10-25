//! Screenshot State Coordinator
//! Complete parity with `shell/modules/screenshot/regionSelector/RegionSelection.qml`.

use super::actions::{build_action_script, SnipAction};
use super::config::ScreenshotConfig;
use super::region::{SelectionMode, SelectionRect, SelectionRegion};
use crate::services::kwin::WindowItem;
use std::path::PathBuf;
use tracing::info;

/// Screenshot selection phase
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionPhase {
    Select,
    Post,
}

/// Master state coordinator for the interactive screenshot overlay
pub struct ScreenshotState {
    pub is_active: bool,
    pub phase: SelectionPhase,
    pub selection: Option<SelectionRegion>,
    pub action: SnipAction,
    pub mode: SelectionMode,
    pub is_dragging: bool,
    pub cursor_pos: (f32, f32),
    pub temp_screenshot_path: PathBuf,
}

impl Default for ScreenshotState {
    fn default() -> Self {
        Self {
            is_active: false,
            phase: SelectionPhase::Select,
            selection: None,
            action: SnipAction::Copy,
            mode: SelectionMode::RectCorners,
            is_dragging: false,
            cursor_pos: (0.0, 0.0),
            temp_screenshot_path: PathBuf::from("/tmp/caelestia-frozen-screen.png"),
        }
    }
}

impl ScreenshotState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Trigger overlay activation
    pub fn activate(&mut self, action: SnipAction) {
        self.is_active = true;
        self.phase = SelectionPhase::Select;
        self.action = action;
        self.selection = None;
        self.is_dragging = false;
        info!("Activated interactive screenshot overlay for action: {:?}", action);
    }

    /// Start mouse drag selection
    pub fn start_drag(&mut self, x: f32, y: f32) {
        self.is_dragging = true;
        self.phase = SelectionPhase::Select;
        self.selection = Some(SelectionRegion::new(x, y, self.mode));
        self.cursor_pos = (x, y);
    }

    /// Update mouse drag position
    pub fn update_drag(&mut self, x: f32, y: f32) {
        self.cursor_pos = (x, y);
        if self.is_dragging {
            if let Some(ref mut region) = self.selection {
                region.update(x, y);
            }
        }
    }

    /// Finish drag and show action options toolbar
    pub fn finish_drag(&mut self) {
        self.is_dragging = false;
        if let Some(ref region) = self.selection {
            if region.bounds().is_valid() {
                self.phase = SelectionPhase::Post;
            } else {
                self.selection = None;
            }
        }
    }

    /// Snap selection to window rectangle
    pub fn snap_to_window(&mut self, win: &WindowItem) {
        let mut region = SelectionRegion::new(win.x as f32, win.y as f32, SelectionMode::RectCorners);
        region.snap_to_window(win);
        self.selection = Some(region);
        self.phase = SelectionPhase::Post;
    }

    /// Set post-selection action
    pub fn set_action(&mut self, action: SnipAction) {
        self.action = action;
    }

    /// Get current selection bounds
    pub fn current_bounds(&self) -> Option<SelectionRect> {
        self.selection.as_ref().map(|s| s.bounds())
    }

    /// Execute the chosen action on the selected area
    pub fn execute_action(&mut self, config: &ScreenshotConfig) -> Option<String> {
        let bounds = self.current_bounds()?;
        let script = build_action_script(&bounds, &self.temp_screenshot_path, self.action, config);

        info!("Executing screenshot snip action: {:?}", self.action);
        self.dismiss();

        if !cfg!(test) && std::env::var("CAELESTIA_TEST").is_err() {
            let _ = std::process::Command::new("bash")
                .arg("-c")
                .arg(&script)
                .spawn();
        }

        Some(script)
    }

    /// Dismiss overlay
    pub fn dismiss(&mut self) {
        self.is_active = false;
        self.selection = None;
        self.is_dragging = false;
        self.phase = SelectionPhase::Select;
    }
}
