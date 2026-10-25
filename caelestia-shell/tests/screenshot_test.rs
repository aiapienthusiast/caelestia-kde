//! Comprehensive Test Suite for Screenshot Subsystem
//! Validates selection geometry normalization, window boundary snapping,
//! snip action script generation, and overlay lifecycle.

use caelestia_shell::modules::screenshot::{
    build_action_script, escape_shell_str, ScreenshotConfig, ScreenshotState, SelectionMode,
    SelectionPhase, SelectionRect, SelectionRegion, SnipAction,
};
use caelestia_shell::services::kwin::WindowItem;
use std::path::Path;

#[test]
fn test_selection_region_geometry_and_normalization() {
    // Normal drag: top-left (100, 100) to bottom-right (400, 300)
    let mut reg1 = SelectionRegion::new(100.0, 100.0, SelectionMode::RectCorners);
    reg1.update(400.0, 300.0);
    let b1 = reg1.bounds();
    assert_eq!(b1, SelectionRect::new(100.0, 100.0, 300.0, 200.0));
    assert!(b1.is_valid());
    assert_eq!(b1.area(), 60000.0);

    // Inverted drag: bottom-right (500, 400) to top-left (200, 150)
    let mut reg2 = SelectionRegion::new(500.0, 400.0, SelectionMode::RectCorners);
    reg2.update(200.0, 150.0);
    let b2 = reg2.bounds();
    assert_eq!(b2, SelectionRect::new(200.0, 150.0, 300.0, 250.0));
    assert!(b2.is_valid());

    // Too small drag (< 4px) is invalid
    let mut reg3 = SelectionRegion::new(100.0, 100.0, SelectionMode::RectCorners);
    reg3.update(102.0, 101.0);
    assert!(!reg3.bounds().is_valid());
}

#[test]
fn test_window_snapping() {
    let win = WindowItem {
        address: "win-code".to_string(),
        title: "VS Code".to_string(),
        app_id: "code".to_string(),
        x: 120,
        y: 80,
        width: 1400,
        height: 900,
        ..Default::default()
    };

    let mut state = ScreenshotState::new();
    state.activate(SnipAction::Copy);
    assert_eq!(state.phase, SelectionPhase::Select);

    // Snap to window
    state.snap_to_window(&win);
    assert_eq!(state.phase, SelectionPhase::Post);

    let bounds = state.current_bounds().expect("Should have snapped bounds");
    assert_eq!(bounds.x, 120.0);
    assert_eq!(bounds.y, 80.0);
    assert_eq!(bounds.width, 1400.0);
    assert_eq!(bounds.height, 900.0);
}

#[test]
fn test_snip_action_script_generation() {
    let config = ScreenshotConfig::default();
    let rect = SelectionRect::new(50.0, 60.0, 800.0, 600.0);
    let temp_shot = Path::new("/tmp/screen-frozen.png");

    // Copy script contains crop, wl-copy, and notify
    let copy_script = build_action_script(&rect, temp_shot, SnipAction::Copy, &config);
    assert!(copy_script.contains("magick '/tmp/screen-frozen.png' -crop 800x600+50+60 +repage"));
    assert!(copy_script.contains("wl-copy -t image/png"));
    assert!(copy_script.contains("notify-send"));

    // Edit script invokes swappy
    let edit_script = build_action_script(&rect, temp_shot, SnipAction::Edit, &config);
    assert!(edit_script.contains("swappy -f"));

    // Search script invokes curl upload and lens URL
    let search_script = build_action_script(&rect, temp_shot, SnipAction::Search, &config);
    assert!(search_script.contains("lens.google.com"));
    assert!(search_script.contains("curl -sF"));

    // OCR script invokes tesseract
    let ocr_script = build_action_script(&rect, temp_shot, SnipAction::CharRecognition, &config);
    assert!(ocr_script.contains("tesseract"));
    assert!(ocr_script.contains("wl-copy"));

    // Record script starts gpu-screen-recorder
    let record_script = build_action_script(&rect, temp_shot, SnipAction::Record, &config);
    assert!(record_script.contains("gpu-screen-recorder"));

    // Shell string escaping
    assert_eq!(escape_shell_str("my'file'name"), "my'\\''file'\\''name");
}

#[test]
fn test_screenshot_state_lifecycle_and_drag() {
    let mut state = ScreenshotState::new();
    assert!(!state.is_active);

    state.activate(SnipAction::Copy);
    assert!(state.is_active);
    assert_eq!(state.phase, SelectionPhase::Select);

    // Drag start
    state.start_drag(10.0, 20.0);
    assert!(state.is_dragging);
    assert_eq!(state.cursor_pos, (10.0, 20.0));

    // Drag update
    state.update_drag(500.0, 400.0);
    assert_eq!(state.cursor_pos, (500.0, 400.0));

    // Finish drag transitions to Post phase
    state.finish_drag();
    assert!(!state.is_dragging);
    assert_eq!(state.phase, SelectionPhase::Post);

    // Dismiss resets state
    state.dismiss();
    assert!(!state.is_active);
    assert_eq!(state.phase, SelectionPhase::Select);
    assert_eq!(state.selection, None);
}
