//! Material 3 Theme & Palette Manager for Caelestia Shell.
//! Parity with shell/services/Colours.qml and shell/plugin/src/Caelestia/Services/palettemanager.cpp.

#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct CaelestiaColours {
    // Surface variants
    pub surface: u32,
    pub surface_container_lowest: u32,
    pub surface_container_low: u32,
    pub surface_container: u32,
    pub surface_container_high: u32,
    pub surface_container_highest: u32,
    pub on_surface: u32,
    pub on_surface_variant: u32,

    // Primary & Secondary
    pub primary: u32,
    pub on_primary: u32,
    pub primary_container: u32,
    pub on_primary_container: u32,
    pub secondary: u32,
    pub on_secondary: u32,
    pub secondary_container: u32,
    pub on_secondary_container: u32,

    // Outlines & Borders
    pub outline: u32,
    pub outline_variant: u32,

    // Inverse
    pub inverse_surface: u32,
    pub inverse_on_surface: u32,
    pub inverse_primary: u32,

    // Error states
    pub error: u32,
    pub on_error: u32,
    pub error_container: u32,
    pub on_error_container: u32,

    // Disabled states
    pub disabled_container: u32,
    pub disabled_content: u32,

    // Special
    pub mascot_bg: u32,
    pub is_light: bool,
}

impl gpui::Global for CaelestiaColours {}


pub fn parse_hex_color(s: &str, alpha: f32) -> u32 {
    let clean = s.trim().trim_start_matches('#');
    if clean.len() >= 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0);
        let a = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
    } else {
        0x000000ff
    }
}

pub fn alter_colour(hex: &str, alpha: f32, light: bool, transp_base: f32, layer: i32) -> u32 {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() < 6 {
        return 0x000000ff;
    }
    let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0) as f32 / 255.0;

    let luminance = (0.299 * r * r + 0.587 * g * g + 0.114 * b * b).sqrt();
    if luminance <= 0.0 {
        return parse_hex_color(hex, alpha);
    }

    let layer_sign = if !light || layer == 1 { 1.0 } else { -(layer as f32) / 2.0 };
    let light_mul = if light { 0.2 } else { 0.3 };
    let wall_factor = if light { if layer == 1 { 3.0 } else { 1.0 } } else { 2.5 };
    let offset = layer_sign * light_mul * (1.0 - transp_base) * (1.0 + 0.25 * wall_factor);
    let scale = (luminance + offset) / luminance;

    let r_out = (r * scale).clamp(0.0, 1.0);
    let g_out = (g * scale).clamp(0.0, 1.0);
    let b_out = (b * scale).clamp(0.0, 1.0);

    let r_byte = (r_out * 255.0).round() as u8;
    let g_byte = (g_out * 255.0).round() as u8;
    let b_byte = (b_out * 255.0).round() as u8;
    let a_byte = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;

    ((r_byte as u32) << 24) | ((g_byte as u32) << 16) | ((b_byte as u32) << 8) | (a_byte as u32)
}

impl Default for CaelestiaColours {
    fn default() -> Self {
        Self {
            surface: 0x1a111094,
            surface_container_lowest: 0x140c0b80,
            surface_container_low: 0x20151480,
            surface_container: 0x271d1c80,
            surface_container_high: 0x322826a0,
            surface_container_highest: 0x3d3231b0,
            on_surface: 0xf1dedcff,
            on_surface_variant: 0xd8c2bfff,

            primary: 0xffb4aaff,
            on_primary: 0x690005ff,
            primary_container: 0x871c19ff,
            on_primary_container: 0xffdad5ff,
            secondary: 0xe7bdb7ff,
            on_secondary: 0x442a26ff,
            secondary_container: 0x5d3f3be6,
            on_secondary_container: 0xffdad5ff,

            outline: 0xa08c8aff,
            outline_variant: 0x53434140,

            inverse_surface: 0xf1dedcff,
            inverse_on_surface: 0x392e2dff,
            inverse_primary: 0xb92420ff,

            error: 0xffb4abff,
            on_error: 0x690005ff,
            error_container: 0x93000aff,
            on_error_container: 0xffdad6ff,

            disabled_container: 0x271d1c40,
            disabled_content: 0xf1dedc60,

            mascot_bg: 0x140c0b80,
            is_light: false,
        }
    }
}

impl CaelestiaColours {
    pub fn load() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let scheme_path = format!("{}/.local/state/caelestia/scheme.json", home);
        let shell_path = format!("{}/.config/caelestia/shell.json", home);

        let mut transp_base = 0.58f32;
        let mut transp_layers = 0.50f32;

        if let Ok(shell_str) = std::fs::read_to_string(&shell_path) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&shell_str) {
                if let Some(appearance) = val.get("appearance") {
                    if let Some(tr) = appearance.get("transparency") {
                        if let Some(b) = tr.get("base").and_then(|v| v.as_f64()) {
                            transp_base = b as f32;
                        }
                        if let Some(l) = tr.get("layers").and_then(|v| v.as_f64()) {
                            transp_layers = l as f32;
                        }
                    }
                }
            }
        }

        let scheme_str = match std::fs::read_to_string(&scheme_path) {
            Ok(s) => s,
            Err(e) => {
                crate::infrastructure::loader_diagnostics::global_record_fallback(
                    crate::infrastructure::loader_diagnostics::DiagnosticCategory::Theme,
                    "scheme.json",
                    Some(scheme_path.as_str()),
                    "Using hardcoded default Material 3 Dark theme",
                    format!("Failed to read theme file: {}", e),
                );
                return Self::default();
            }
        };

        let scheme: serde_json::Value = match serde_json::from_str(&scheme_str) {
            Ok(v) => v,
            Err(e) => {
                crate::infrastructure::loader_diagnostics::global_record_fallback(
                    crate::infrastructure::loader_diagnostics::DiagnosticCategory::Theme,
                    "scheme.json",
                    Some(scheme_path.as_str()),
                    "Using hardcoded default Material 3 Dark theme",
                    format!("JSON syntax error: {}", e),
                );
                return Self::default();
            }
        };

        let is_light = scheme.get("mode").and_then(|m| m.as_str()) == Some("light");
        let colours = match scheme.get("colours") {
            Some(c) => c,
            None => {
                crate::infrastructure::loader_diagnostics::global_record_fallback(
                    crate::infrastructure::loader_diagnostics::DiagnosticCategory::Theme,
                    "scheme.json",
                    Some(scheme_path.as_str()),
                    "Using default Material 3 Dark theme",
                    "Missing 'colours' object in scheme JSON",
                );
                return Self::default();
            }
        };

        crate::infrastructure::loader_diagnostics::global_record_loaded(
            crate::infrastructure::loader_diagnostics::DiagnosticCategory::Theme,
            "scheme.json",
            Some(scheme_path.as_str()),
            format!("Material 3 {} palette loaded successfully", if is_light { "light" } else { "dark" }),
        );

        let get_hex = |key: &str, fallback: &str| -> String {
            colours
                .get(key)
                .and_then(|v| v.as_str())
                .unwrap_or(fallback)
                .to_string()
        };

        let surface_hex = get_hex("surface", "1a1110");
        let surface_container_lowest_hex = get_hex("surfaceContainerLowest", "140c0b");
        let surface_container_low_hex = get_hex("surfaceContainerLow", "201514");
        let surface_container_hex = get_hex("surfaceContainer", "271d1c");
        let surface_container_high_hex = get_hex("surfaceContainerHigh", "322826");
        let surface_container_highest_hex = get_hex("surfaceContainerHighest", "3d3231");
        let on_surface_hex = get_hex("onSurface", "f1dedc");
        let on_surface_variant_hex = get_hex("onSurfaceVariant", "d8c2bf");

        let primary_hex = get_hex("primary", "ffb4aa");
        let on_primary_hex = get_hex("onPrimary", "690005");
        let primary_container_hex = get_hex("primaryContainer", "871c19");
        let on_primary_container_hex = get_hex("onPrimaryContainer", "ffdad5");
        let secondary_hex = get_hex("secondary", "e7bdb7");
        let on_secondary_hex = get_hex("onSecondary", "442a26");
        let secondary_container_hex = get_hex("secondaryContainer", "5d3f3b");
        let on_secondary_container_hex = get_hex("onSecondaryContainer", "ffdad5");

        let outline_hex = get_hex("outline", "a08c8a");
        let outline_variant_hex = get_hex("outlineVariant", "534341");

        let inverse_surface_hex = get_hex("inverseSurface", "f1dedc");
        let inverse_on_surface_hex = get_hex("inverseOnSurface", "392e2d");
        let inverse_primary_hex = get_hex("inversePrimary", "b92420");

        let error_hex = get_hex("error", "ffb4ab");
        let on_error_hex = get_hex("onError", "690005");
        let error_container_hex = get_hex("errorContainer", "93000a");
        let on_error_container_hex = get_hex("onErrorContainer", "ffdad6");

        Self {
            surface: parse_hex_color(&surface_hex, transp_base),
            surface_container_lowest: parse_hex_color(&surface_container_lowest_hex, transp_layers),
            surface_container_low: parse_hex_color(&surface_container_low_hex, transp_layers),
            surface_container: alter_colour(&surface_container_hex, transp_layers, is_light, transp_base, 1),
            surface_container_high: alter_colour(&surface_container_high_hex, (transp_layers + 0.15).min(1.0), is_light, transp_base, 1),
            surface_container_highest: alter_colour(&surface_container_highest_hex, (transp_layers + 0.25).min(1.0), is_light, transp_base, 2),
            on_surface: parse_hex_color(&on_surface_hex, 1.0),
            on_surface_variant: parse_hex_color(&on_surface_variant_hex, 0.85),

            primary: parse_hex_color(&primary_hex, 1.0),
            on_primary: parse_hex_color(&on_primary_hex, 1.0),
            primary_container: parse_hex_color(&primary_container_hex, 1.0),
            on_primary_container: parse_hex_color(&on_primary_container_hex, 1.0),
            secondary: parse_hex_color(&secondary_hex, 1.0),
            on_secondary: parse_hex_color(&on_secondary_hex, 1.0),
            secondary_container: parse_hex_color(&secondary_container_hex, 0.90),
            on_secondary_container: parse_hex_color(&on_secondary_container_hex, 1.0),

            outline: parse_hex_color(&outline_hex, 0.6),
            outline_variant: parse_hex_color(&outline_variant_hex, 0.25),

            inverse_surface: parse_hex_color(&inverse_surface_hex, 0.95),
            inverse_on_surface: parse_hex_color(&inverse_on_surface_hex, 1.0),
            inverse_primary: parse_hex_color(&inverse_primary_hex, 1.0),

            error: parse_hex_color(&error_hex, 1.0),
            on_error: parse_hex_color(&on_error_hex, 1.0),
            error_container: parse_hex_color(&error_container_hex, 1.0),
            on_error_container: parse_hex_color(&on_error_container_hex, 1.0),

            disabled_container: parse_hex_color(&surface_container_hex, 0.25),
            disabled_content: parse_hex_color(&on_surface_hex, 0.38),

            mascot_bg: parse_hex_color(&surface_container_lowest_hex, transp_layers),
            is_light,
        }
    }
}
