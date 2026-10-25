use caelestia_shell::components::effects::{
    AmbientGlowState, ColouredIconView, ColouriserEffect, MaskEffect,
};
use gpui::rgb;

#[test]
fn test_mask_effect_parameters() {
    let mask = MaskEffect::new().with_inverted(true).with_threshold(0.75);
    assert!(mask.enabled);
    assert!(mask.inverted);
    assert_eq!(mask.threshold_min, 0.75);
    assert_eq!(mask.spread_at_min, 1.0);
}

#[test]
fn test_colouriser_luminance_and_brightness() {
    // Dark color -> high brightness adjustment (1.0 - low luminance)
    let dark_effect = ColouriserEffect::new(rgb(0x101010));
    assert!(dark_effect.brightness > 0.8);

    // Light color -> low brightness adjustment
    let light_effect = ColouriserEffect::new(rgb(0xf0f0f0));
    assert!(light_effect.brightness < 0.2);

    // Colorization clamping
    let clamped = dark_effect.with_colorization(1.5);
    assert_eq!(clamped.colorization, 1.0);
}

#[test]
fn test_coloured_icon_view_construction() {
    let _icon = ColouredIconView::new("settings", rgb(0xd0bcff)).with_size(32.0);
    assert!(true);
}

#[test]
fn test_ambient_glow_state_and_color_interpolation() {
    let mut glow = AmbientGlowState::new()
        .with_address("0x240000a")
        .with_fallback_color(rgb(0xd0bcff))
        .with_opacity(0.5);

    assert_eq!(glow.address, "0x240000a");
    assert_eq!(glow.active_address, "0x240000a");
    assert!(glow.active);

    let effective_color = glow.effective_glow_color();
    assert_eq!(effective_color.a, 0.5);

    // Switch to new address
    glow.trigger_switch("0x3800012");
    assert_eq!(glow.address, "0x3800012");
    assert_eq!(glow.active_address, "0x3800012");
    assert_eq!(glow.bloom_progress, 1.0);
}
