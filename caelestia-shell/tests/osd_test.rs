use caelestia_shell::modules::drawers::DrawerVisibilities;
use caelestia_shell::modules::osd::*;
use caelestia_shell::services::audio::AudioService;
use caelestia_shell::services::brightness::BrightnessService;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_osd_state_icons_and_helpers() {
    // 1. Volume Icon parity with Icons.getVolumeIcon
    assert_eq!(OsdState::volume_icon(0.8, true), "volume_off");
    assert_eq!(OsdState::volume_icon(0.0, false), "volume_mute");
    assert_eq!(OsdState::volume_icon(0.3, false), "volume_down");
    assert_eq!(OsdState::volume_icon(0.8, false), "volume_up");
    assert_eq!(OsdState::volume_icon(1.5, false), "volume_up");

    // 2. Microphone Icon parity with Icons.getMicVolumeIcon
    assert_eq!(OsdState::mic_icon(true), "mic_off");
    assert_eq!(OsdState::mic_icon(false), "mic");

    // 3. Brightness Icon parity with `brightness_${Math.round(value * 6) + 1}`
    assert_eq!(OsdState::brightness_icon(0.0), "brightness_1");
    assert_eq!(OsdState::brightness_icon(0.5), "brightness_4");
    assert_eq!(OsdState::brightness_icon(1.0), "brightness_7");

    // 4. Edge offset parity with Wrapper.qml
    let mut state = OsdState::default();
    state.offset_scale = 1.0; // closed
    state.sidebar_or_session_visible = false;
    assert_eq!(state.edge_offset(60.0), -65.0);

    state.sidebar_or_session_visible = true;
    assert_eq!(state.edge_offset(60.0), -77.0); // includes 12px sidebar offset
}

#[test]
fn test_osd_controller_reacts_to_audio_service() {
    let mut controller = OsdController::new(OsdConfig::default());
    assert!(!controller.state.is_open);

    let mut audio = AudioService::new();
    audio.state.sink_volume = 0.4;
    audio.state.sink_muted = false;

    // Synchronize initial state
    controller.sync_audio(&audio);
    assert!(controller.state.is_open);
    assert_eq!(controller.state.volume, 0.4);
    assert_eq!(controller.state.offset_scale, 0.0);

    // Close OSD
    controller.hide();
    assert!(!controller.state.is_open);

    // Volume change triggers OSD popup
    audio.state.sink_volume = 0.65;
    controller.sync_audio(&audio);
    assert!(controller.state.is_open);
    assert_eq!(controller.state.volume, 0.65);

    // Mute toggle triggers OSD popup
    controller.hide();
    audio.state.sink_muted = true;
    controller.sync_audio(&audio);
    assert!(controller.state.is_open);
    assert!(controller.state.muted);

    // Microphone change triggers OSD popup
    controller.hide();
    audio.state.source_volume = 0.8;
    controller.sync_audio(&audio);
    assert!(controller.state.is_open);
    assert_eq!(controller.state.source_volume, 0.8);
}

#[test]
fn test_osd_controller_reacts_to_brightness_service() {
    let mut controller = OsdController::new(OsdConfig::default());
    assert!(!controller.state.is_open);

    let mut brightness = BrightnessService::new();
    brightness.state.current_pct = 75;

    // Synchronize brightness
    controller.sync_brightness(&brightness);
    assert!(controller.state.is_open);
    assert!((controller.state.brightness - 0.75).abs() < 0.01);

    // Hide OSD
    controller.hide();
    assert!(!controller.state.is_open);

    // Brightness change triggers OSD
    brightness.state.current_pct = 40;
    controller.sync_brightness(&brightness);
    assert!(controller.state.is_open);
    assert!((controller.state.brightness - 0.40).abs() < 0.01);
}

#[test]
fn test_osd_auto_hide_and_hover_inhibition() {
    let config = OsdConfig {
        hide_delay_ms: 50,
        ..OsdConfig::default()
    };
    let mut controller = OsdController::new(config);
    controller.show();
    assert!(controller.state.is_open);

    // When hovered, tick should NOT hide OSD even if delay expired
    controller.set_hovered(true);
    sleep(Duration::from_millis(60));
    controller.tick();
    assert!(controller.state.is_open);

    // Once unhovered and delay expired, tick hides OSD
    controller.set_hovered(false);
    sleep(Duration::from_millis(60));
    controller.tick();
    assert!(!controller.state.is_open);
    assert_eq!(controller.state.offset_scale, 1.0);
}

#[test]
fn test_osd_drawer_visibilities_and_suppression() {
    let config = OsdConfig::default();
    let mut state = OsdState::default();
    let mut visibilities = DrawerVisibilities::default();

    state.is_open = true;
    visibilities.osd = true;

    // Normal state: OSD should be active
    assert!(state.should_be_active(&config, visibilities.utilities, visibilities.overview));

    // When utilities drawer opens, OSD is suppressed (matching Wrapper.qml)
    visibilities.utilities = true;
    assert!(!state.should_be_active(&config, visibilities.utilities, visibilities.overview));
    visibilities.utilities = false;

    // When overview is open, OSD is suppressed
    visibilities.overview = true;
    assert!(!state.should_be_active(&config, visibilities.utilities, visibilities.overview));
}
