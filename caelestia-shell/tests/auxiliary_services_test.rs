//! Integration Tests for Ported Auxiliary Services and Infrastructure Subsystems
//! Covers:
//! - AutostartService (XDG desktop entries, parsing, field mapping)
//! - CalendarEventsService (RFC 5545 unfolding, VEVENT extraction, recurrence detection)
//! - OrionSearchService (HTML parsing, tag stripping, DDG unquoting)
//! - PanelCapabilitiesDetector (EDID CTA-861 block parsing, HDR static metadata)
//! - PluginManager (community manifest parsing, bar & background widgets, settings)
//! - KWinEffectsManager (effect metadata extraction, directory resolution)

use caelestia_shell::infrastructure::display_capabilities::PanelCapabilitiesDetector;
use caelestia_shell::modules::launcher::orion::OrionSearchService;
use caelestia_shell::modules::plugins::{
    PluginManager, PluginManifest, PluginSettingType,
};
use caelestia_shell::services::autostart::{AutostartEntry, AutostartService};
use caelestia_shell::services::calendar_events::CalendarEventsService;
use caelestia_shell::services::kwin_effects::KWinEffectsManager;
use std::fs;

#[test]
fn test_autostart_desktop_entry_parsing() {
    let temp_dir = std::env::temp_dir().join(format!("caelestia_test_autostart_{}", std::process::id()));
    fs::create_dir_all(&temp_dir).unwrap();

    let entry_file = temp_dir.join("test-app.desktop");
    let content = r#"[Desktop Entry]
Type=Application
Name=Test Application
Comment=A test autostart application
Exec=/usr/bin/test-app --arg1
Icon=test-icon
Hidden=false
NoDisplay=false
OnlyShowIn=KDE;GNOME;
"#;
    fs::write(&entry_file, content).unwrap();

    let parsed = AutostartService::parse_desktop_file(&entry_file);
    assert!(parsed.is_some(), "Failed to parse desktop file");

    let fields = parsed.unwrap();
    assert_eq!(fields.get("Name").map(|s| s.as_str()), Some("Test Application"));
    assert_eq!(fields.get("Comment").map(|s| s.as_str()), Some("A test autostart application"));
    assert_eq!(fields.get("Exec").map(|s| s.as_str()), Some("/usr/bin/test-app --arg1"));
    assert_eq!(fields.get("Icon").map(|s| s.as_str()), Some("test-icon"));
    assert_eq!(fields.get("Hidden").map(|s| s.as_str()), Some("false"));

    let entry = AutostartEntry {
        id: "test-app".to_string(),
        name: "Test Application".to_string(),
        exec: "/usr/bin/test-app --arg1".to_string(),
        icon: Some("test-icon".to_string()),
        comment: Some("A test autostart application".to_string()),
        enabled: true,
        is_user_entry: true,
        path: entry_file.clone(),
    };
    assert_eq!(entry.id, "test-app");
    assert!(entry.enabled);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_calendar_events_rfc5545_parsing() {
    let ics_data = "BEGIN:VCALENDAR\r\n\
VERSION:2.0\r\n\
PRODID:-//Caelestia Test//EN\r\n\
BEGIN:VEVENT\r\n\
SUMMARY:Team Sync Meeting\r\n\
DTSTART:20261026T100000Z\r\n\
DTEND:20261026T110000Z\r\n\
LOCATION:Virtual Room A\r\n\
DESCRIPTION:Weekly engineering sync and archite\r\n \
cture discussion.\r\n\
RRULE:FREQ=WEEKLY;BYDAY=MO\r\n\
END:VEVENT\r\n\
BEGIN:VEVENT\r\n\
SUMMARY:All-Day Conference\r\n\
DTSTART;VALUE=DATE:20261101\r\n\
DTEND;VALUE=DATE:20261102\r\n\
END:VEVENT\r\n\
END:VCALENDAR\r\n";

    let events = CalendarEventsService::parse_ics_content(ics_data, "work_calendar");
    assert_eq!(events.len(), 2);

    let event1 = &events[0];
    assert_eq!(event1.summary, "Team Sync Meeting");
    assert_eq!(event1.location.as_deref(), Some("Virtual Room A"));
    assert_eq!(
        event1.description.as_deref(),
        Some("Weekly engineering sync and architecture discussion.")
    );
    assert_eq!(event1.calendar_name, "work_calendar");
    assert!(event1.is_recurring);
    assert!(!event1.all_day);

    let event2 = &events[1];
    assert_eq!(event2.summary, "All-Day Conference");
    assert!(event2.all_day);
}

#[test]
fn test_orion_search_parsing_and_sanitization() {
    let raw_html = r#"
    <div class="result__title">
        <a class="result__snippet" href="//duckduckgo.com/l/?uddg=https%3A%2F%2Farchlinux.org%2Fpackages">
            Arch <b>Linux</b> - Package Database &amp; Repositories
        </a>
    </div>
    <div class="result__title">
        <a href="https://example.com/test">
            Direct Link Title
        </a>
    </div>
    "#;

    let results = OrionSearchService::parse_html_results(raw_html, 1, 10);
    assert_eq!(results.len(), 2);

    assert_eq!(
        results[0].title,
        "Arch Linux - Package Database & Repositories"
    );
    assert_eq!(results[0].url, "https://archlinux.org/packages");

    assert_eq!(results[1].title, "Direct Link Title");
    assert_eq!(results[1].url, "https://example.com/test");

    // Verify HTML tag stripping
    let cleaned = OrionSearchService::strip_html_tags("<p>Hello <b>World</b> &amp; Caelestia</p>");
    assert_eq!(cleaned, "Hello World & Caelestia");
}

#[test]
fn test_panel_edid_hdr_capabilities_parser() {
    // Construct a synthetic 256-byte EDID (128-byte base + 128-byte CTA-861 Extension)
    let mut edid = vec![0u8; 256];
    // Base block header: 00 FF FF FF FF FF FF 00
    edid[0] = 0x00;
    for i in 1..7 {
        edid[i] = 0xFF;
    }
    edid[7] = 0x00;

    // Extension flag at offset 126 = 1 (1 extension block follows)
    edid[126] = 1;

    // Extension block starts at 128
    // Tag 0x02 = CTA-861 Extension Block
    edid[128] = 0x02;
    // Revision 3
    edid[129] = 0x03;
    // Offset to DTDs (e.g. 10)
    edid[130] = 10;

    // Data Block Collection starts at offset 132 (128 + 4)
    // Extended Tag Data Block: Tag 7 (Extended), Length 3 -> Header = (7 << 5) | 3 = 0xE3
    edid[132] = 0xE3;
    // Extended Tag Code 0x06 = HDR Static Metadata Data Block
    edid[133] = 0x06;
    // Electro-Optical Transfer Functions: bit 2 = SMPTE ST 2084 (HDR10), bit 3 = HLG
    edid[134] = (1 << 2) | (1 << 3);
    // Static Metadata Descriptor Type 1
    edid[135] = 0x01;

    let has_hdr = PanelCapabilitiesDetector::parse_edid(&edid);
    assert!(has_hdr, "Expected HDR support detected from CTA-861 EDID");

    // Plain EDID without extension should return false
    let plain_edid = vec![0u8; 128];
    assert!(!PanelCapabilitiesDetector::parse_edid(&plain_edid));
}

#[test]
fn test_plugin_manifest_parsing_and_manager() {
    let manifest_json = r#"{
        "id": "weather",
        "name": "Weather Widget",
        "author": "Caelestia Team",
        "description": "Displays current weather and hourly forecasts",
        "version": "1.2.0",
        "barWidgets": [
            {
                "id": "weather-icon",
                "name": "Weather Icon",
                "component": "BarWeather.qml",
                "defaultZone": "right"
            }
        ],
        "backgroundWidgets": [
            {
                "id": "weather-radar",
                "component": "RadarCanvas.qml",
                "z": 1.5
            }
        ],
        "settings": [
            {
                "id": "city",
                "type": "text",
                "label": "City Name",
                "default": "Tokyo"
            },
            {
                "id": "units",
                "type": "enum",
                "label": "Units",
                "default": "metric",
                "options": ["metric", "imperial"]
            }
        ]
    }"#;

    let manifest: PluginManifest = serde_json::from_str(manifest_json).expect("Failed to deserialize PluginManifest");
    assert_eq!(manifest.id, "weather");
    assert_eq!(manifest.name, "Weather Widget");
    assert_eq!(manifest.author, "Caelestia Team");
    assert_eq!(manifest.version, "1.2.0");

    assert_eq!(manifest.bar_widgets.len(), 1);
    assert_eq!(manifest.bar_widgets[0].id, "weather-icon");
    assert_eq!(manifest.bar_widgets[0].default_zone, "right");

    assert_eq!(manifest.background_widgets.len(), 1);
    assert_eq!(manifest.background_widgets[0].id, "weather-radar");
    assert_eq!(manifest.background_widgets[0].z, 1.5);

    assert_eq!(manifest.settings.len(), 2);
    assert_eq!(manifest.settings[0].id, "city");
    assert_eq!(manifest.settings[0].setting_type, PluginSettingType::Text);
    assert_eq!(manifest.settings[1].id, "units");
    assert_eq!(manifest.settings[1].setting_type, PluginSettingType::Enum);

    // Register with PluginManager and verify queries
    let mut manager = PluginManager::new();
    manager.register_plugin(manifest);

    let right_widgets = manager.active_bar_widgets("right");
    assert_eq!(right_widgets.len(), 1);
    assert_eq!(right_widgets[0].1.id, "weather-icon");

    let default_city = manager.get_setting("weather", "city");
    assert_eq!(default_city, Some(serde_json::Value::String("Tokyo".to_string())));

    // Mutate setting
    manager.set_setting("weather", "city", serde_json::Value::String("Kyoto".to_string()));
    let updated_city = manager.get_setting("weather", "city");
    assert_eq!(updated_city, Some(serde_json::Value::String("Kyoto".to_string())));
}

#[test]
fn test_kwin_effects_metadata_parsing() {
    let temp_dir = std::env::temp_dir().join(format!("caelestia_test_kwin_effect_{}", std::process::id()));
    fs::create_dir_all(&temp_dir).unwrap();

    let metadata_path = temp_dir.join("metadata.json");
    let content = r#"{
        "KPlugin": {
            "Id": "caelestia-blur",
            "Name": "Caelestia Dynamic Blur",
            "Description": "Hardware-accelerated dual Kawase blur with window geometry tracking",
            "Authors": [
                {
                    "Name": "Caelestia Contributors"
                }
            ],
            "Category": "Appearance",
            "License": "GPL-3.0"
        }
    }"#;
    fs::write(&metadata_path, content).unwrap();

    let effect_info = KWinEffectsManager::inspect_effect_dir(&temp_dir);
    assert!(effect_info.is_some(), "Failed to parse KPlugin metadata");

    let effect = effect_info.unwrap();
    assert_eq!(effect.id, "caelestia-blur");
    assert_eq!(effect.name, "Caelestia Dynamic Blur");
    assert_eq!(
        effect.description,
        "Hardware-accelerated dual Kawase blur with window geometry tracking"
    );
    assert_eq!(effect.author, "Caelestia Contributors");
    assert_eq!(effect.path, temp_dir);

    let _ = fs::remove_dir_all(&temp_dir);
}
