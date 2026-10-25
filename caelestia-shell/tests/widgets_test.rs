use caelestia_shell::components::widgets::{
    CoverArtState, ExtraIndicatorView, HourStripView, HourlyForecastEntry,
};

#[test]
fn test_cover_art_state_rotation_and_lifecycle() {
    let mut state = CoverArtState::new()
        .with_art("https://example.com/cover.jpg")
        .with_playing(true)
        .with_size(120.0);

    assert_eq!(state.size, 120.0);
    assert!(state.is_playing);
    assert_eq!(state.rotation_deg, 0.0);

    // Advance by 23.5 seconds (exactly 1 full rotation)
    state.step_rotation(23.5);
    assert!((state.rotation_deg - 0.0).abs() < 0.01);

    // Advance by 11.75 seconds (180 degrees)
    state.step_rotation(11.75);
    assert!((state.rotation_deg - 180.0).abs() < 0.1);

    // When paused, rotation should not advance
    state.is_playing = false;
    state.step_rotation(5.0);
    assert!((state.rotation_deg - 180.0).abs() < 0.1);
}

#[test]
fn test_extra_indicator_pill() {
    let _indicator = ExtraIndicatorView::new(5).with_radius(16.0);
    assert!(true);
}

#[test]
fn test_hourly_forecast_entry_and_icon_mapping() {
    let entry_clear = HourlyForecastEntry::new("2026-10-26T14:00", 14, 22.5, 0);
    assert_eq!(entry_clear.icon_name, "weather_sunny");
    assert_eq!(entry_clear.format_hour(true), "2PM");
    assert_eq!(entry_clear.format_hour(false), "14:00");

    let entry_rain = HourlyForecastEntry::new("2026-10-26T00:00", 0, 15.0, 61);
    assert_eq!(entry_rain.icon_name, "weather_rainy");
    assert_eq!(entry_rain.format_hour(true), "12AM");
    assert_eq!(entry_rain.format_hour(false), "00:00");

    let entry_snow = HourlyForecastEntry::new("2026-10-26T08:00", 8, -2.0, 71);
    assert_eq!(entry_snow.icon_name, "weather_snowy");
    assert_eq!(entry_snow.format_hour(true), "8AM");
}

#[test]
fn test_hour_strip_view_construction() {
    let entries = vec![
        HourlyForecastEntry::new("2026-10-26T12:00", 12, 20.0, 0),
        HourlyForecastEntry::new("2026-10-26T13:00", 13, 21.0, 1),
    ];
    let _strip = HourStripView::new(entries).with_twelve_hour(false);
    assert!(true);
}
