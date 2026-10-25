use caelestia_shell::components::images::{
    AnimatedImageController, CachedImageSource, FadeImageController, IconSource, ImageFillMode,
    VideoWallpaperController, WindowStreamState,
};
use std::path::PathBuf;

#[test]
fn test_cached_image_source_scaling() {
    let source = CachedImageSource::new("/tmp/test.png")
        .with_fill_mode(ImageFillMode::PreserveAspectFit)
        .with_size(100.0, 50.0)
        .with_dpr(2.0);

    assert_eq!(source.path, PathBuf::from("/tmp/test.png"));
    assert_eq!(source.fill_mode, ImageFillMode::PreserveAspectFit);
    assert_eq!(source.target_pixel_size(), (200, 100));
}

#[test]
fn test_animated_image_controller_lifecycle() {
    let mut controller = AnimatedImageController::new("/tmp/anim.gif").with_frames(3);
    assert!(controller.playing);
    assert_eq!(controller.current_frame, 0);

    // Step 1
    controller.step_frame();
    assert_eq!(controller.current_frame, 1);

    // Step 2
    controller.step_frame();
    assert_eq!(controller.current_frame, 2);

    // Step 3 (loops back to 0)
    controller.step_frame();
    assert_eq!(controller.current_frame, 0);
    assert_eq!(controller.loops_completed, 1);

    // Pause
    controller.pause();
    assert!(!controller.playing);
    controller.step_frame();
    assert_eq!(controller.current_frame, 0); // No advance while paused

    // Restart
    controller.play();
    controller.step_frame();
    assert_eq!(controller.current_frame, 1);
    controller.restart();
    assert_eq!(controller.current_frame, 0);
    assert!(controller.playing);
}

#[test]
fn test_icon_source_parsing() {
    assert_eq!(IconSource::parse("").is_empty(), true);

    let theme_icon = IconSource::parse("image://icon/firefox");
    assert_eq!(theme_icon, IconSource::Theme("firefox".to_string()));
    assert_eq!(theme_icon.display_name(), "firefox");

    let file_uri = IconSource::parse("file:///usr/share/pixmaps/app.png");
    assert_eq!(
        file_uri,
        IconSource::File(PathBuf::from("/usr/share/pixmaps/app.png"))
    );
    assert_eq!(file_uri.display_name(), "app.png");

    let direct_path = IconSource::parse("/opt/my-app/icon.svg");
    assert_eq!(
        direct_path,
        IconSource::File(PathBuf::from("/opt/my-app/icon.svg"))
    );

    let named = IconSource::parse("folder");
    assert_eq!(named, IconSource::Theme("folder".to_string()));
}

#[test]
fn test_fade_image_controller_transitions() {
    let mut controller = FadeImageController::new();
    assert_eq!(controller.opacity, 1.0);
    assert!(controller.current_source.is_none());

    // First source
    controller.set_source("/tmp/first.jpg");
    assert_eq!(controller.current_source, Some(PathBuf::from("/tmp/first.jpg")));
    assert!(controller.previous_source.is_none());
    assert!(!controller.had_previous);

    // Change to second source
    controller.set_source("/tmp/second.jpg");
    assert_eq!(controller.current_source, Some(PathBuf::from("/tmp/second.jpg")));
    assert_eq!(controller.previous_source, Some(PathBuf::from("/tmp/first.jpg")));
    assert!(controller.had_previous);
    assert!(controller.is_fading_out);
    assert_eq!(controller.opacity, 0.0);

    // Complete fade in
    controller.complete_fade_in();
    assert_eq!(controller.opacity, 1.0);
    assert!(!controller.is_fading_out);
    assert!(controller.previous_source.is_none());
}

#[test]
fn test_window_stream_preview_fitting() {
    let stream = WindowStreamState::new("0x12345")
        .with_fallback_icon("preferences-system-windows")
        .with_stream(42, 1001);

    assert!(stream.stream_available);
    assert_eq!(stream.node_id, Some(42));
    assert_eq!(stream.object_serial, Some(1001));

    // Standard 16:9 container test
    let (w, h) = stream.fitted_dimensions(1920.0, 1080.0);
    assert_eq!(w, 1920.0);
    assert_eq!(h, 1080.0);

    // Narrow container
    let (nw, nh) = stream.fitted_dimensions(800.0, 600.0);
    assert_eq!(nw, 800.0);
    assert_eq!(nh, 450.0); // 800 / (16/9)
}

#[test]
fn test_video_wallpaper_auto_pause_and_mute() {
    let mut controller = VideoWallpaperController::new("/tmp/bg.mp4", true);
    assert!(controller.playing);
    assert!(!controller.muted);

    // Auto-pause on fullscreen
    controller.update_pause_state(true, false, true, false);
    assert!(!controller.playing);

    // Resume when fullscreen exits
    controller.update_pause_state(false, false, true, false);
    assert!(controller.playing);

    // User explicitly paused -> stays paused regardless
    controller.paused_by_user = true;
    controller.update_pause_state(false, false, true, false);
    assert!(!controller.playing);
    controller.paused_by_user = false;

    // Auto-mute when media starts playing
    controller.update_mute_state(true, true, true);
    assert!(controller.muted);

    // Unmute when media stops
    controller.update_mute_state(true, true, false);
    assert!(!controller.muted);

    // Non-first instance is always muted
    let mut secondary = VideoWallpaperController::new("/tmp/bg.mp4", false);
    secondary.update_mute_state(true, false, false);
    assert!(secondary.muted);
}
