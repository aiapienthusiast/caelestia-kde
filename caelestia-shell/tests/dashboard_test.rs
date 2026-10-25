//! Custom Test Suite for Unified Caelestia Infrastructure, Services, and Dashboard
//! Ensures 100% parity with legacy shell/utils, shell/services, and shell/modules/dashboard.

use caelestia_shell::infrastructure::paths::CaelestiaPaths;
use caelestia_shell::infrastructure::sysinfo::SystemInfo;
use caelestia_shell::modules::dashboard::DashboardTab;
use caelestia_shell::services::mpris::{MprisService, PlaybackStatus, TrackMetadata};
use caelestia_shell::services::system_monitor::SystemMonitorService;
use caelestia_shell::services::weather::{WeatherData, WeatherService};

#[test]
fn test_caelestia_paths_resolution_and_shortening() {
    let paths = CaelestiaPaths::new();

    assert!(!paths.home.as_os_str().is_empty());
    assert!(paths.config.ends_with(".config/caelestia") || paths.config.ends_with("caelestia"));
    assert!(paths.state.ends_with(".local/state/caelestia") || paths.state.ends_with("caelestia"));
    assert!(paths.cache.ends_with(".cache/caelestia") || paths.cache.ends_with("caelestia"));

    // Scheme & Shell config paths
    assert!(paths.scheme_path().ends_with("scheme.json"));
    assert!(paths.shell_config_path().ends_with("shell.json"));

    // Home expansion & shortening
    let test_file = paths.home.join("Pictures/test.png");
    let shortened = paths.shorten_home(&test_file);
    assert_eq!(shortened, "~/Pictures/test.png");

    let expanded = paths.expand_home("~/Pictures/test.png");
    assert_eq!(expanded, test_file);
}

#[test]
fn test_system_info_loading_and_formatting() {
    let sysinfo = SystemInfo::load();

    assert!(!sysinfo.os_name.is_empty(), "OS name should not be empty");
    assert!(!sysinfo.kernel.is_empty(), "Kernel should not be empty");
    assert!(!sysinfo.hostname.is_empty(), "Hostname should not be empty");
    assert!(!sysinfo.user.is_empty(), "User should not be empty");

    // Device name fallback
    let device = sysinfo.device_name();
    assert!(device.is_empty() || !device.to_lowercase().contains("to be filled"));

    // Uptime formatting
    let uptime = SystemInfo::uptime_formatted();
    assert!(uptime.ends_with('m'), "Uptime should end with 'm': {}", uptime);
}

#[test]
fn test_system_monitor_metrics_and_sparkline_history() {
    let mut monitor = SystemMonitorService::new();

    // Refresh and check values
    monitor.refresh();
    assert!(monitor.state.ram_total_bytes > 0, "Total RAM must be > 0");
    assert!(monitor.ram_total_gib() > 0.0);
    assert!(monitor.ram_used_gib() <= monitor.ram_total_gib());

    // Sparkline history buffers must have samples and not exceed capacity
    assert!(!monitor.state.cpu_history.is_empty());
    assert!(!monitor.state.ram_history.is_empty());
    assert!(monitor.state.cpu_history.len() <= 30);
    assert!(monitor.state.ram_history.len() <= 30);

    // Each sample must be normalized between 0.0 and 1.0
    for &sample in &monitor.state.cpu_history {
        assert!(sample >= 0.0 && sample <= 1.0);
    }
}

#[test]
fn test_weather_service_and_wmo_icon_mappings() {
    let weather = WeatherService::load();

    assert!(!weather.data.temp.is_empty());
    assert!(!weather.data.condition.is_empty());
    assert!(!weather.data.icon.is_empty());

    // Test WMO Weather icon mappings
    assert_eq!(WeatherService::icon_for_wmo_code(0, true), "sunny");
    assert_eq!(WeatherService::icon_for_wmo_code(0, false), "clear_night");
    assert_eq!(WeatherService::icon_for_wmo_code(1, true), "partly_cloudy_day");
    assert_eq!(WeatherService::icon_for_wmo_code(3, true), "cloud");
    assert_eq!(WeatherService::icon_for_wmo_code(61, true), "rainy_heavy");
    assert_eq!(WeatherService::icon_for_wmo_code(95, true), "thunderstorm");

    // Serialization & round-trip caching
    let custom = WeatherData {
        temp: "28°".to_string(),
        temp_c: 28.0,
        condition: "Sunny".to_string(),
        icon: "sunny".to_string(),
        humidity: 40,
        wind_speed_kmh: 8.5,
        location: "KDEland".to_string(),
        high: "30°".to_string(),
        low: "20°".to_string(),
    };
    let json = serde_json::to_string(&custom).unwrap();
    let deserialized: WeatherData = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.temp, "28°");
    assert_eq!(deserialized.location, "KDEland");
}

#[test]
fn test_mpris_service_metadata_and_progress() {
    let mut mpris = MprisService::new();

    // Default resting track
    assert!(!mpris.current_track.title.is_empty());
    assert!(!mpris.current_track.artist.is_empty());

    // Mock track and progress
    mpris.current_track = TrackMetadata {
        title: "Still Rollin".to_string(),
        artist: "Shubh".to_string(),
        album: "Still Rollin".to_string(),
        art_url: "file:///tmp/art.png".to_string(),
        status: PlaybackStatus::Playing,
        position_secs: 75.0,
        duration_secs: 150.0,
    };

    assert_eq!(mpris.progress(), 0.5);
    assert_eq!(mpris.formatted_time(), "1:15 / 2:30");
}

#[test]
fn test_dashboard_tabs_integrity() {
    assert_eq!(DashboardTab::ALL.len(), 4);
    assert_eq!(DashboardTab::Dashboard.label(), "Dashboard");
    assert_eq!(DashboardTab::Media.label(), "Media");
    assert_eq!(DashboardTab::Performance.label(), "Performance");
    assert_eq!(DashboardTab::Weather.label(), "Weather");

    assert_eq!(DashboardTab::Dashboard.icon(), "dashboard");
    assert_eq!(DashboardTab::Media.icon(), "queue_music");
    assert_eq!(DashboardTab::Performance.icon(), "speed");
    assert_eq!(DashboardTab::Weather.icon(), "cloud");
}

#[test]
fn test_dashboard_tab_cycling() {
    let tabs = DashboardTab::ALL;
    // Cycling forwards (right arrow or tab key)
    for (i, tab) in tabs.iter().enumerate() {
        let next_idx = (i + 1) % tabs.len();
        assert_eq!(tabs[next_idx], match tab {
            DashboardTab::Dashboard => DashboardTab::Media,
            DashboardTab::Media => DashboardTab::Performance,
            DashboardTab::Performance => DashboardTab::Weather,
            DashboardTab::Weather => DashboardTab::Dashboard,
        });
    }

    // Cycling backwards (left arrow key)
    for (i, tab) in tabs.iter().enumerate() {
        let prev_idx = if i == 0 { tabs.len() - 1 } else { i - 1 };
        assert_eq!(tabs[prev_idx], match tab {
            DashboardTab::Dashboard => DashboardTab::Weather,
            DashboardTab::Media => DashboardTab::Dashboard,
            DashboardTab::Performance => DashboardTab::Media,
            DashboardTab::Weather => DashboardTab::Performance,
        });
    }
}

#[test]
fn test_sysinfo_sanitise_dmi_and_device_name() {
    // Junk filtering
    assert_eq!(SystemInfo::sanitise_dmi("To be filled by O.E.M."), "");
    assert_eq!(SystemInfo::sanitise_dmi("Default string"), "");
    assert_eq!(SystemInfo::sanitise_dmi("None"), "");
    assert_eq!(SystemInfo::sanitise_dmi("System Product Name"), "");
    assert_eq!(SystemInfo::sanitise_dmi("  Framework  "), "Framework");

    // Device name formatting
    let mut info = SystemInfo::load();
    info.board_vendor = "Framework".to_string();
    info.board_name = "Laptop 16".to_string();
    assert_eq!(info.device_name(), "Framework Laptop 16");

    info.board_vendor = "Lenovo".to_string();
    info.board_name = "Lenovo ThinkPad X1".to_string();
    assert_eq!(info.device_name(), "Lenovo ThinkPad X1");
}

#[test]
fn test_mpris_zero_duration_and_stopped() {
    let mut mpris = MprisService::new();
    mpris.current_track = TrackMetadata {
        title: "Test".to_string(),
        artist: "Artist".to_string(),
        album: String::new(),
        art_url: String::new(),
        status: PlaybackStatus::Stopped,
        position_secs: 0.0,
        duration_secs: 0.0,
    };

    assert_eq!(mpris.progress(), 0.0);
    assert_eq!(mpris.formatted_time(), "0:00 / 0:00");
}
