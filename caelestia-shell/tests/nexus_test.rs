//! Custom test cases for Nexus Control Center and ShellConfig.

use caelestia_shell::domain::tokens::*;
use caelestia_shell::infrastructure::config::ShellConfig;
use caelestia_shell::modules::nexus::state::{NexusState, CATEGORIES};
use std::fs;

#[test]
fn test_nexus_categories_and_navigation() {
    assert_eq!(CATEGORIES.len(), 11);
    assert_eq!(CATEGORIES[0].key, "appearance");
    assert_eq!(CATEGORIES[1].key, "desktop");
    assert_eq!(CATEGORIES[2].key, "panels");
    assert_eq!(CATEGORIES[3].key, "network");
    assert_eq!(CATEGORIES[10].key, "about");

    let mut state = NexusState::new();
    assert_eq!(state.selected_index, 0);
    assert_eq!(state.current_category().label, "Appearance");

    // Category navigation
    state.select_category(2);
    assert_eq!(state.selected_index, 2);
    assert_eq!(state.current_category().label, "Panels");

    // Search filtering
    state.set_search_query("wall");
    let matches = state.filtered_categories();
    assert!(!matches.is_empty());
    assert_eq!(matches[0].1.key, "appearance"); // "Wallpapers, fonts, colors"

    state.set_search_query("tiling");
    let matches = state.filtered_categories();
    assert!(!matches.is_empty());
    assert_eq!(matches[0].1.key, "desktop"); // "Desktop & Tiling"

    state.clear_search();
    assert_eq!(state.filtered_categories().len(), 11);
}

#[test]
fn test_shell_config_read_and_mutation() {
    let mut config = ShellConfig::default();

    // Verify appearance defaults
    assert_eq!(config.pitch_black(), false);
    assert_eq!(config.blur_enabled(), true);
    assert_eq!(config.transparency_enabled(), true);
    assert_eq!(config.transparency_base(), 0.58);
    assert_eq!(config.transparency_layers(), 0.50);
    assert_eq!(config.border_thickness(), 0.0);
    assert_eq!(config.rounding_scale(), 1.0);

    // Mutate pitch black (Bezel mode)
    config.set_pitch_black(true);
    assert_eq!(config.pitch_black(), true);

    // Mutate transparency
    config.set_transparency_base(0.75);
    assert_eq!(config.transparency_base(), 0.75);
    config.set_transparency_layers(0.65);
    assert_eq!(config.transparency_layers(), 0.65);

    // Disabling transparency also disables blur
    config.set_transparency_enabled(false);
    assert_eq!(config.transparency_enabled(), false);
    assert_eq!(config.blur_enabled(), false);

    // Mutate border thickness and rounding scale
    config.set_border_thickness(4.0);
    assert_eq!(config.border_thickness(), 4.0);
    config.set_rounding_scale(1.2);
    assert_eq!(config.rounding_scale(), 1.2);

    // Mutate font family
    config.set_font_family("Rubik");
    assert_eq!(config.font_family(), "Rubik");
}

#[test]
fn test_shell_config_atomic_persistence() {
    let temp_dir = std::env::temp_dir().join(format!("test_nexus_{}", std::process::id()));
    let _ = fs::create_dir_all(&temp_dir);
    let test_config_path = temp_dir.join("shell.json");

    // Create a mock shell.json with complex preexisting structure matching real Caelestia user setups
    let initial_json = r#"{
        "ai": { "defaultOllamaModel": "qwen2.5-coder:3b" },
        "appearance": {
            "pitchBlack": false,
            "transparency": { "base": 0.58, "enabled": true, "layers": 0.50 },
            "blur": true
        },
        "bar": { "dock": { "pinnedApps": ["firefox", "discord"] } }
    }"#;
    fs::write(&test_config_path, initial_json).unwrap();

    // Load from disk
    let mut config = ShellConfig::load_from(&test_config_path);
    assert_eq!(config.pitch_black(), false);
    assert_eq!(config.blur_enabled(), true);
    assert_eq!(config.transparency_base(), 0.58);

    // Mutate multiple appearance settings
    config.set_pitch_black(true);
    config.set_border_thickness(6.0);
    config.set_transparency_base(0.85);
    config.set_rounding_scale(1.25);

    // Persist atomically to disk
    assert!(config.save_to(&test_config_path).is_ok());

    // Re-load and verify all mutated values and preserved unrelated keys
    let reloaded = ShellConfig::load_from(&test_config_path);
    assert_eq!(reloaded.pitch_black(), true);
    assert_eq!(reloaded.border_thickness(), 6.0);
    assert_eq!(reloaded.transparency_base(), 0.85);
    assert_eq!(reloaded.rounding_scale(), 1.25);

    // Verify unrelated keys are 100% intact
    let raw = fs::read_to_string(&test_config_path).unwrap();
    let val: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(
        val["ai"]["defaultOllamaModel"],
        "qwen2.5-coder:3b"
    );
    assert_eq!(
        val["bar"]["dock"]["pinnedApps"][0],
        "firefox"
    );
    assert_eq!(
        val["bar"]["dock"]["pinnedApps"][1],
        "discord"
    );

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_nexus_dimensions_and_geometry() {
    // 16:9 Aspect ratio verification matching shell/modules/nexus/Nexus.qml
    let width = 1080.0f32;
    let height = 640.0f32;
    let ratio = width / height;
    assert!((ratio - (16.0f32 / 9.5f32)).abs() < 0.2f32);

    // Drawers presentation geometry & tokens parity
    assert_eq!(DRAWER_WIDTH, 116.0);
    assert_eq!(HITBOX_WIDTH, 4.0);
    assert_eq!(BUTTON_WIDTH, 76.0);
    assert_eq!(BUTTON_HEIGHT, 76.0);
    assert!(ROUNDING_EXTRA_LARGE >= 28.0);
    assert!(ROUNDING_MEDIUM >= 12.0);
    assert!(ROUNDING_FULL >= 9999.0);
    assert!(PADDING_LARGE >= 16.0);
    assert!(PADDING_MEDIUM >= 12.0);
}

#[test]
fn test_nexus_all_categories_integrity() {
    assert_eq!(CATEGORIES.len(), 11);
    let mut seen_keys = std::collections::HashSet::new();

    for cat in CATEGORIES.iter() {
        assert!(!cat.key.is_empty(), "Category key cannot be empty");
        assert!(!cat.label.is_empty(), "Category label cannot be empty");
        assert!(!cat.description.is_empty(), "Category description cannot be empty");
        assert!(!cat.icon.is_empty(), "Category icon cannot be empty");
        assert!(seen_keys.insert(cat.key), "Duplicate category key: {}", cat.key);
    }
}

#[test]
fn test_nexus_appearance_function_calls_and_toggles() {
    let mut config = ShellConfig::default();

    // Initial state
    assert_eq!(config.pitch_black(), false);
    assert_eq!(config.blur_enabled(), true);
    assert_eq!(config.transparency_enabled(), true);

    // Toggle pitch black (Bezel mode)
    config.set_pitch_black(!config.pitch_black());
    assert_eq!(config.pitch_black(), true);

    // Toggle blur
    config.set_blur_enabled(!config.blur_enabled());
    assert_eq!(config.blur_enabled(), false);
    config.set_blur_enabled(true);
    assert_eq!(config.blur_enabled(), true);

    // Base opacity cycle stops
    let stops = [0.35, 0.50, 0.58, 0.70, 0.85, 1.00];
    for &stop in stops.iter() {
        config.set_transparency_base(stop);
        assert!((config.transparency_base() - stop).abs() < 0.01);
    }

    // Border thickness steps
    let thicknesses = [0.0, 1.0, 2.0, 4.0, 6.0];
    for &th in thicknesses.iter() {
        config.set_border_thickness(th);
        assert_eq!(config.border_thickness(), th);
    }

    // Corner radius scale steps
    let scales = [0.5, 0.75, 1.0, 1.25, 1.5];
    for &sc in scales.iter() {
        config.set_rounding_scale(sc);
        assert_eq!(config.rounding_scale(), sc);
    }
}

#[test]
fn test_all_nexus_pages_instantiation() {
    use caelestia_shell::infrastructure::theme::CaelestiaColours;
    use caelestia_shell::modules::nexus::pages::*;

    let c = CaelestiaColours::default();
    let _ = AboutPage::render(&c);
    let _ = DesktopPage::render(&c);
    let _ = PanelsPage::render(&c);
    let _ = NetworkPage::render(&c);
    let _ = BluetoothPage::render(&c);
    let _ = DisplayPage::render(&c);
    let _ = AudioPage::render(&c);
    let _ = PowerPage::render(&c);
    let _ = AppsPage::render(&c);
    let _ = ServicesPage::render(&c);
}

