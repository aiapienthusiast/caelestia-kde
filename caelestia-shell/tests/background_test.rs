use caelestia_shell::modules::background::{
    clock::DesktopClockModel,
    config::{BackgroundConfig, DesktopClockLayout, WallpaperFitMode},
    context_menu::{DesktopContextMenuModel, DesktopMenuAction},
    state::BackgroundState,
    wallpaper::WallpaperModel,
};
use chrono::TimeZone;
use std::fs::File;
use std::path::PathBuf;

#[test]
fn test_wallpaper_format_detection_and_cycling() {
    let temp_dir = std::env::temp_dir().join("caelestia_bg_test");
    let _ = std::fs::create_dir_all(&temp_dir);

    let img1 = temp_dir.join("wall1.png");
    let img2 = temp_dir.join("wall2.jpg");
    let vid1 = temp_dir.join("wall3.mp4");
    let txt = temp_dir.join("notes.txt");

    let _ = File::create(&img1);
    let _ = File::create(&img2);
    let _ = File::create(&vid1);
    let _ = File::create(&txt);

    assert!(WallpaperModel::is_supported_wallpaper(&img1));
    assert!(WallpaperModel::is_supported_wallpaper(&img2));
    assert!(WallpaperModel::is_supported_wallpaper(&vid1));
    assert!(!WallpaperModel::is_supported_wallpaper(&txt));

    assert!(!WallpaperModel::is_video_path(&img1));
    assert!(WallpaperModel::is_video_path(&vid1));

    let mut model = WallpaperModel::new(Some(img1.clone()));
    model.scan_directory(&temp_dir);

    assert_eq!(model.available.len(), 3);
    assert_eq!(model.current_path, Some(img1.clone()));
    assert_eq!(model.current_index, Some(0));

    // Cycle forward
    model.next_wallpaper(false);
    assert_eq!(model.current_path, Some(img2.clone()));
    assert_eq!(model.current_index, Some(1));

    model.next_wallpaper(false);
    assert_eq!(model.current_path, Some(vid1.clone()));
    assert!(model.is_current_video());

    // Wrap around
    model.next_wallpaper(false);
    assert_eq!(model.current_path, Some(img1.clone()));

    // Cycle backwards
    model.prev_wallpaper();
    assert_eq!(model.current_path, Some(vid1.clone()));

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_desktop_clock_layout_modes() {
    // Stacked mode: aspect < 1.25
    assert_eq!(
        DesktopClockModel::calculate_layout_mode(200.0, 200.0),
        DesktopClockLayout::Stacked
    );

    // Compact mode: width < 340 or aspect < 1.55
    assert_eq!(
        DesktopClockModel::calculate_layout_mode(300.0, 200.0),
        DesktopClockLayout::Compact
    );
    assert_eq!(
        DesktopClockModel::calculate_layout_mode(400.0, 280.0),
        DesktopClockLayout::Compact
    );

    // Wide mode: wider container
    assert_eq!(
        DesktopClockModel::calculate_layout_mode(600.0, 200.0),
        DesktopClockLayout::Wide
    );
}

#[test]
fn test_desktop_clock_formatting() {
    let fixed_time = chrono::Local
        .with_ymd_and_hms(2026, 10, 25, 18, 45, 30)
        .unwrap();

    // Wide Layout
    let mut model = DesktopClockModel::new(DesktopClockLayout::Wide);
    model.show_seconds = false;
    let wide = model.format_datetime(fixed_time);
    assert_eq!(wide.primary_time, "18:45");
    assert_eq!(wide.secondary_time, None);
    assert!(wide.day_of_week.contains("Sunday"));

    // Wide Layout with seconds
    model.show_seconds = true;
    let wide_sec = model.format_datetime(fixed_time);
    assert_eq!(wide_sec.primary_time, "18:45:30");

    // Stacked Layout: Hours on primary, Minutes on secondary
    let stacked_model = DesktopClockModel::new(DesktopClockLayout::Stacked);
    let stacked = stacked_model.format_datetime(fixed_time);
    assert_eq!(stacked.primary_time, "18");
    assert_eq!(stacked.secondary_time, Some("45".to_string()));
}

#[test]
fn test_desktop_context_menu_actions() {
    let mut menu = DesktopContextMenuModel::default();
    assert!(!menu.is_open);

    menu.open_at(250.0, 400.0, "DP-1");
    assert!(menu.is_open);
    assert_eq!(menu.position_x, 250.0);
    assert_eq!(menu.position_y, 400.0);
    assert_eq!(menu.screen_name, "DP-1");

    // Enter Edit Mode action
    menu.execute(DesktopMenuAction::EnterEditMode);
    assert!(!menu.is_open);
    assert!(menu.edit_mode);
}

#[test]
fn test_background_state_coordination() {
    let config = BackgroundConfig {
        enabled: true,
        wallpaper_enabled: true,
        wallpaper_path: Some(PathBuf::from("/tmp/test_bg.png")),
        slideshow_enabled: false,
        slideshow_interval_mins: 15,
        slideshow_random: false,
        fit_mode: WallpaperFitMode::Cover,
        desktop_icons_enabled: true,
        desktop_clock_enabled: true,
        desktop_clock_layout: DesktopClockLayout::Wide,
    };

    let mut state = BackgroundState::new(config);
    assert!(state.icons_visible);

    // Middle click toggles icons
    state.on_empty_desktop_middle_click();
    assert!(!state.icons_visible);

    state.on_empty_desktop_middle_click();
    assert!(state.icons_visible);

    // Right click opens context menu
    state.on_empty_desktop_right_click(500.0, 300.0, "eDP-1");
    assert!(state.context_menu.is_open);
    assert_eq!(state.context_menu.position_x, 500.0);

    // Executing toggle icons through context menu
    state.execute_context_menu_action(DesktopMenuAction::ToggleDesktopIcons);
    assert!(!state.context_menu.is_open);
    assert!(!state.icons_visible);
}
