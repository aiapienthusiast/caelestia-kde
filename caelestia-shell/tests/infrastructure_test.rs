use caelestia_shell::infrastructure::*;

#[test]
fn test_solar_calculator_parsing_and_windows() {
    assert_eq!(SolarCalculator::parse_time("06:30"), Some(390));
    assert_eq!(SolarCalculator::parse_time("18:45"), Some(1125));
    assert_eq!(SolarCalculator::parse_time("25:00"), None);

    let coords = SolarCalculator::parse_coords("51.5074, -0.1278").unwrap();
    assert_eq!(coords.lat, 51.5074);
    assert_eq!(coords.lon, -0.1278);

    // Light window: 06:00 (360) to 18:00 (1080)
    assert!(SolarCalculator::is_light_at(700, 360, 1080));
    assert!(!SolarCalculator::is_light_at(200, 360, 1080));

    // Window wrapping midnight (e.g. 22:00 (1320) to 06:00 (360))
    // 23:00 (1380) is in the window
    assert!(SolarCalculator::is_light_at(1380, 1320, 360));
    // 03:00 (180) is in the window
    assert!(SolarCalculator::is_light_at(180, 1320, 360));
    // 12:00 (720) is outside the window
    assert!(!SolarCalculator::is_light_at(720, 1320, 360));
}

#[test]
fn test_geometry_utils_fit_and_centering() {
    let outer = Rect {
        x: 0.0,
        y: 0.0,
        width: 1920.0,
        height: 1080.0,
    };

    let centered = GeometryUtils::center_inside(800.0, 600.0, outer);
    assert_eq!(centered.x, 560.0);
    assert_eq!(centered.y, 240.0);

    // Aspect fit 16:9 into 4:3
    let (fit_w, fit_h) = GeometryUtils::fit_aspect(1920.0, 1080.0, 800.0, 600.0);
    assert_eq!(fit_w, 800.0);
    assert_eq!(fit_h, 450.0);

    // Aspect fill 16:9 into 4:3
    let (fill_w, fill_h) = GeometryUtils::fill_aspect(1920.0, 1080.0, 800.0, 600.0);
    assert_eq!(fill_h, 600.0);
    assert!((fill_w - (600.0 * 16.0 / 9.0)).abs() < 1e-6);

    // Clamp
    let inner = Rect {
        x: 2000.0,
        y: -50.0,
        width: 100.0,
        height: 100.0,
    };
    let clamped = GeometryUtils::clamp_rect(inner, outer);
    assert_eq!(clamped.x, 1820.0);
    assert_eq!(clamped.y, 0.0);
}

#[test]
fn test_string_utils() {
    assert_eq!(StringUtils::truncate("Hello World", 5), "Hell…");
    assert_eq!(StringUtils::truncate("Short", 10), "Short");

    assert_eq!(StringUtils::pluralize(1, "window", "windows"), "1 window");
    assert_eq!(StringUtils::pluralize(3, "window", "windows"), "3 windows");

    assert_eq!(StringUtils::format_duration(45), "00:45");
    assert_eq!(StringUtils::format_duration(125), "02:05");
    assert_eq!(StringUtils::format_duration(3665), "1:01:05");

    assert_eq!(StringUtils::capitalize("caelestia"), "Caelestia");
}

#[test]
fn test_icon_resolver() {
    assert_eq!(IconResolver::resolve("", "fallback-icon"), "fallback-icon");
    assert_eq!(IconResolver::resolve("firefox", "fallback"), "firefox");

    assert_eq!(IconResolver::map_class_to_icon("Alacritty"), "utilities-terminal");
    assert_eq!(IconResolver::map_class_to_icon("org.kde.dolphin"), "system-file-manager");
    assert_eq!(IconResolver::map_class_to_icon("unknown_binary"), "application-x-executable");

    assert_eq!(IconResolver::weather_icon(1000, false), "weather-clear");
    assert_eq!(IconResolver::weather_icon(1000, true), "weather-clear-night");
    assert_eq!(IconResolver::weather_icon(1066, false), "weather-snow");
}
