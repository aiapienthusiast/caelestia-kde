//! Screenshot Subsystem
//! Complete parity with `shell/modules/screenshot/`.
//!
//! Provides the interactive Wayland screen capture overlay:
//! - Rectangular and circular region selection
//! - Window boundary snapping
//! - Post-selection action dispatch (Copy, Edit in Swappy, Search with Lens, OCR, Record)
//! - Action toolbar and crosshair cursor guides

pub mod actions;
pub mod circle_selection;
pub mod config;
pub mod cursor_guide;
pub mod region;
pub mod state;
pub mod view;

pub use actions::{build_action_script, escape_shell_str, SnipAction};
pub use circle_selection::CircleSelectionDetails;
pub use config::ScreenshotConfig;
pub use cursor_guide::{CaptureMode, CursorGuideModel, OptionsToolbarModel};
pub use region::{SelectionMode, SelectionRect, SelectionRegion};
pub use state::{ScreenshotState, SelectionPhase};
pub use view::ScreenshotOverlayView;

