use caelestia_shell::modules::drawers::blur::*;
use caelestia_shell::modules::drawers::focus_return::*;
use caelestia_shell::modules::screenshot::circle_selection::*;
use caelestia_shell::modules::screenshot::cursor_guide::*;
use caelestia_shell::modules::utilities::gallery::*;
use caelestia_shell::modules::utilities::modals::*;
use std::path::PathBuf;

#[test]
fn test_drawers_blur_bodies_bounding_box() {
    let mut blur_model = BlurBodiesModel::default();
    assert_eq!(blur_model.bounding_box(), None);

    blur_model.add_region(BlurRegionRect {
        x: 0.0,
        y: 0.0,
        width: 400.0,
        height: 1080.0,
        corner_radius: 16.0,
    });
    blur_model.add_region(BlurRegionRect {
        x: 350.0,
        y: 100.0,
        width: 300.0,
        height: 400.0,
        corner_radius: 12.0,
    });

    let bb = blur_model.bounding_box().unwrap();
    assert_eq!(bb.x, 0.0);
    assert_eq!(bb.y, 0.0);
    assert_eq!(bb.width, 650.0);
    assert_eq!(bb.height, 1080.0);
}

#[test]
fn test_focus_return_coordinator() {
    let mut coordinator = FocusReturnCoordinator::default();
    assert!(!coordinator.any_drawer_open);

    let active_win = PreviousWindowFocus {
        window_id: 42,
        app_id: "firefox".to_string(),
        title: "Rust Documentation".to_string(),
        surface_id: Some(101),
    };

    coordinator.on_drawer_opening(Some(active_win.clone()));
    assert!(coordinator.any_drawer_open);
    assert_eq!(coordinator.saved_focus, Some(active_win.clone()));

    // Opening second drawer does not overwrite original focus
    let second_win = PreviousWindowFocus {
        window_id: 99,
        app_id: "terminal".to_string(),
        title: "Shell".to_string(),
        surface_id: None,
    };
    coordinator.on_drawer_opening(Some(second_win));
    assert_eq!(coordinator.saved_focus, Some(active_win.clone()));

    let returned = coordinator.on_drawer_closing();
    assert_eq!(returned, Some(active_win));
    assert!(!coordinator.any_drawer_open);
    assert_eq!(coordinator.saved_focus, None);
}

#[test]
fn test_screenshot_circle_selection_and_guides() {
    let circle = CircleSelectionDetails::from_points(100.0, 100.0, 300.0, 100.0);
    assert_eq!(circle.center_x, 200.0);
    assert_eq!(circle.center_y, 100.0);
    assert_eq!(circle.radius, 100.0);
    assert_eq!(circle.diameter(), 200.0);

    let (x, y, w, h) = circle.bounding_rect();
    assert_eq!(x, 100.0);
    assert_eq!(y, 0.0);
    assert_eq!(w, 200.0);
    assert_eq!(h, 200.0);

    let mut guide = CursorGuideModel::default();
    guide.update_pos(512.0, 768.0);
    assert_eq!(guide.formatted_coords(), "512, 768 px");

    let mut toolbar = OptionsToolbarModel::default();
    toolbar.show_at(200.0, 300.0);
    assert!(toolbar.is_visible);
    assert_eq!(toolbar.position_x, 200.0);
    toolbar.set_mode(CaptureMode::Recording);
    assert_eq!(toolbar.capture_mode, CaptureMode::Recording);
    toolbar.hide();
    assert!(!toolbar.is_visible);
}

#[test]
fn test_utilities_modals_and_galleries() {
    let mut modal = ModalDialogModel::open_destructive(
        "Delete Recording",
        "Are you sure you want to permanently delete this video?",
        "Delete",
        Some("rec_123".to_string()),
    );
    assert!(modal.is_open);
    assert!(modal.is_destructive);
    assert_eq!(modal.target_id.as_deref(), Some("rec_123"));

    modal.close();
    assert!(!modal.is_open);
    assert_eq!(modal.target_id, None);

    let mut gallery = CaptureGalleryModel::default();
    gallery.add_screenshot(GalleryItemDef {
        id: "snip_1".to_string(),
        path: PathBuf::from("/tmp/snip_1.png"),
        timestamp_epoch_secs: 1000,
        size_bytes: 45000,
        duration_secs: None,
        dimensions: Some((1920, 1080)),
        is_selected: false,
    });
    assert_eq!(gallery.screenshots.len(), 1);
    assert!(gallery.delete_screenshot("snip_1"));
    assert_eq!(gallery.screenshots.len(), 0);
}
