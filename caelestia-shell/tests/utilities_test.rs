//! Comprehensive Test Suite for Utilities Subsystem
//! Validates IdleInhibit (Keep Awake), Screen Capture & Recording lists,
//! QuickShare device selector and transfers, and Quick Toggles pagination & dispatch.

use caelestia_shell::modules::utilities::{
    CaptureCardModel, IdleInhibitor, NearbyDevice, QuickShareModel, QuickTogglesModel,
    UtilitiesConfig, UtilitiesState,
};
use caelestia_shell::services::audio::AudioService;
use caelestia_shell::services::bluetooth::BluetoothService;
use caelestia_shell::services::network::NetworkService;
use caelestia_shell::services::night_light::NightLightService;
use caelestia_shell::services::notifications::NotificationsService;
use std::path::PathBuf;

#[test]
fn test_idle_inhibit_toggle_and_active_timer() {
    let mut inhibitor = IdleInhibitor::new();
    assert!(!inhibitor.state.enabled);
    assert_eq!(inhibitor.state.enabled_since_secs, None);
    assert_eq!(inhibitor.active_time_string(), None);

    // Enable
    inhibitor.set_enabled(true);
    assert!(inhibitor.state.enabled);
    assert!(inhibitor.state.enabled_since_secs.is_some());
    assert!(inhibitor.active_time_string().is_some());

    // Toggle off
    inhibitor.toggle();
    assert!(!inhibitor.state.enabled);
    assert_eq!(inhibitor.state.enabled_since_secs, None);
    assert_eq!(inhibitor.active_time_string(), None);

    // Toggle on
    inhibitor.toggle();
    assert!(inhibitor.state.enabled);
    assert!(inhibitor.state.enabled_since_secs.is_some());
}

#[test]
fn test_capture_model_recording_and_screenshots() {
    let mut model = CaptureCardModel::new();
    assert!(!model.is_recording);
    assert_eq!(model.recordings.len(), 0);
    assert_eq!(model.screenshots.len(), 0);

    // Start recording
    model.start_recording();
    assert!(model.is_recording);
    assert!(model.record_start_secs.is_some());

    // Stop recording
    let rec_item = model.stop_recording().expect("Should return finished recording item");
    assert!(!model.is_recording);
    assert_eq!(model.recordings.len(), 1);
    assert_eq!(model.recordings[0].id, rec_item.id);

    // Take screenshot
    model.take_screenshot(true);
    assert_eq!(model.screenshots.len(), 1);

    // Deletion confirmation flow
    let shot_path = model.screenshots[0].filepath.clone();
    model.request_delete(&shot_path);
    assert_eq!(model.pending_delete_path, Some(shot_path.clone()));

    // Cancel deletion
    model.cancel_delete();
    assert_eq!(model.pending_delete_path, None);
    assert_eq!(model.screenshots.len(), 1);

    // Confirm deletion
    model.request_delete(&shot_path);
    let deleted = model.confirm_delete();
    assert_eq!(deleted, Some(shot_path));
    assert_eq!(model.screenshots.len(), 0);

    // Expand / collapse lists
    assert!(!model.recording_list_expanded);
    model.toggle_recordings_expanded();
    assert!(model.recording_list_expanded);

    assert!(!model.screenshot_list_expanded);
    model.toggle_screenshots_expanded();
    assert!(model.screenshot_list_expanded);
}

#[test]
fn test_quick_share_device_discovery_and_transfers() {
    let mut model = QuickShareModel::new();
    assert!(model.is_enabled);
    assert!(!model.is_selector_open);

    model.devices = vec![
        NearbyDevice {
            id: "dev-phone".to_string(),
            name: "Pixel 9".to_string(),
            device_type: "phone".to_string(),
        },
        NearbyDevice {
            id: "dev-laptop".to_string(),
            name: "Framework 16".to_string(),
            device_type: "laptop".to_string(),
        },
    ];

    // Modal open / close
    model.open_device_selector();
    assert!(model.is_selector_open);

    model.select_device("dev-phone");
    assert_eq!(model.selected_device_id.as_deref(), Some("dev-phone"));

    // Send file
    let tx = model
        .send_file("dev-phone", "document.pdf")
        .expect("Should initiate transfer");
    assert_eq!(tx.filename, "document.pdf");
    assert_eq!(tx.device_name, "Pixel 9");
    assert_eq!(model.transfers.len(), 1);
    assert!(!model.is_selector_open); // Closes on send

    // Delete transfer flow
    model.request_delete_transfer(&tx.id);
    assert_eq!(model.pending_delete_transfer_id.as_deref(), Some(tx.id.as_str()));

    model.confirm_delete_transfer();
    assert_eq!(model.transfers.len(), 0);
    assert_eq!(model.pending_delete_transfer_id, None);
}

#[test]
fn test_quick_toggles_pagination_and_service_reaction() {
    let config = UtilitiesConfig::default();
    let mut toggles = QuickTogglesModel::new(config.quick_toggles.clone(), 4);

    assert_eq!(toggles.per_page, 4);
    assert_eq!(toggles.current_page, 0);
    assert!(toggles.page_count() >= 2);

    let first_page = toggles.page_slice();
    assert_eq!(first_page.len(), 4);
    assert_eq!(first_page[0].id, "wifi");

    // Next page
    toggles.next_page();
    assert_eq!(toggles.current_page, 1);
    let second_page = toggles.page_slice();
    assert!(second_page.len() >= 2);

    // Cannot go past last page
    toggles.next_page();
    assert_eq!(toggles.current_page, toggles.page_count() - 1);

    // Prev page
    toggles.prev_page();
    assert_eq!(toggles.current_page, 0);

    // Cannot go before page 0
    toggles.prev_page();
    assert_eq!(toggles.current_page, 0);

    // Reaction to mock services
    let mut network = NetworkService::new();
    network.state.wifi_enabled = true;

    let mut bt = BluetoothService::new();
    bt.state.powered = true;

    let mut nl = NightLightService::new();
    nl.state.enabled = true;

    let mut notif = NotificationsService::new();
    notif.state.dnd = true;

    let mut audio = AudioService::new();
    audio.state.source_muted = true;

    assert!(toggles.is_active("wifi", Some(&network), None, None, None, None));
    assert!(toggles.is_active("bluetooth", None, Some(&bt), None, None, None));
    assert!(toggles.is_active("nightlight", None, None, Some(&nl), None, None));
    assert!(toggles.is_active("dnd", None, None, None, Some(&notif), None));
    assert!(toggles.is_active("mic_mute", None, None, None, None, Some(&audio)));

    // Toggle actions
    toggles.toggle_action("wifi", Some(&mut network), None, None, None, None);
    assert!(!network.state.wifi_enabled);

    toggles.toggle_action("pause_wallpaper", None, None, None, None, None);
    assert!(toggles.wallpaper_paused);
}

#[test]
fn test_utilities_state_lifecycle_and_cleanup() {
    let config = UtilitiesConfig::default();
    let mut state = UtilitiesState::new(&config);

    assert!(!state.is_open);
    state.open();
    assert!(state.is_open);

    // Set some modal state
    state.capture.pending_delete_path = Some(PathBuf::from("/tmp/test.png"));
    state.quick_share.is_selector_open = true;
    state.quick_share.pending_delete_transfer_id = Some("tx-1".to_string());

    // Closing resets all active modals
    state.close();
    assert!(!state.is_open);
    assert_eq!(state.capture.pending_delete_path, None);
    assert!(!state.quick_share.is_selector_open);
    assert_eq!(state.quick_share.pending_delete_transfer_id, None);

    // Toggle
    state.toggle();
    assert!(state.is_open);
    state.toggle();
    assert!(!state.is_open);
}
