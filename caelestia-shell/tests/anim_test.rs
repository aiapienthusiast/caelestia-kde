use std::time::Duration;
use gpui::Rgba;
use caelestia_shell::components::anim::*;
use caelestia_shell::domain::tokens::*;

#[test]
fn test_anim_durations_and_tokens() {
    assert_eq!(AnimType::FastSpatial.duration_ms(), ANIM_DURATION_EXPRESSIVE_FAST_SPATIAL);
    assert_eq!(AnimType::DefaultSpatial.duration_ms(), ANIM_DURATION_EXPRESSIVE_DEFAULT_SPATIAL);
    assert_eq!(AnimType::SlowSpatial.duration_ms(), ANIM_DURATION_EXPRESSIVE_SLOW_SPATIAL);
    assert_eq!(AnimType::FastEffects.duration_ms(), ANIM_DURATION_EXPRESSIVE_FAST_EFFECTS);
    assert_eq!(AnimType::DefaultEffects.duration_ms(), ANIM_DURATION_EXPRESSIVE_DEFAULT_EFFECTS);
    assert_eq!(AnimType::SlowEffects.duration_ms(), ANIM_DURATION_EXPRESSIVE_SLOW_EFFECTS);

    assert_eq!(AnimType::StandardSmall.duration_ms(), ANIM_DURATION_SMALL);
    assert_eq!(AnimType::Standard.duration_ms(), ANIM_DURATION_NORMAL);
    assert_eq!(AnimType::StandardLarge.duration_ms(), ANIM_DURATION_LARGE);
    assert_eq!(AnimType::StandardExtraLarge.duration_ms(), ANIM_DURATION_EXTRA_LARGE);

    assert_eq!(AnimType::EmphasizedSmall.duration_ms(), ANIM_DURATION_SMALL);
    assert_eq!(AnimType::Emphasized.duration_ms(), ANIM_DURATION_NORMAL);
}

#[test]
fn test_cubic_bezier_sampling() {
    let curve = EASING_STANDARD;
    assert!((curve.sample(0.0) - 0.0).abs() < 1e-4);
    assert!((curve.sample(1.0) - 1.0).abs() < 1e-4);

    let mid = curve.sample(0.5);
    assert!(mid > 0.0 && mid < 1.0);
}

#[test]
fn test_anim_value_interpolation() {
    let anim = Anim::new(AnimType::DefaultSpatial).with_range(10.0, 50.0);
    assert_eq!(anim.value_at(Duration::ZERO), 10.0);

    let half_time = anim.duration / 2;
    let mid_val = anim.value_at(half_time);
    assert!(mid_val > 10.0 && mid_val < 50.0);

    let full_val = anim.value_at(anim.duration);
    assert!((full_val - 50.0).abs() < 1e-3);
}

#[test]
fn test_canim_theme_switch_suppression_and_interpolation() {
    let mut canim = CAnim::new(false);
    assert_eq!(canim.duration, Duration::from_millis(ANIM_DURATION_EXPRESSIVE_SLOW_EFFECTS));

    canim.set_switching_theme(true);
    assert_eq!(canim.duration, Duration::ZERO);
    assert_eq!(canim.progress(Duration::ZERO), 1.0);

    canim.set_switching_theme(false);
    let from = Rgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    let to = Rgba { r: 1.0, g: 0.5, b: 0.2, a: 1.0 };
    let canim = canim.with_range(from, to);

    let c_start = canim.color_at(Duration::ZERO);
    assert_eq!(c_start, from);

    let c_end = canim.color_at(canim.duration);
    assert!((c_end.r - 1.0).abs() < 1e-3);
    assert!((c_end.g - 0.5).abs() < 1e-3);
    assert!((c_end.b - 0.2).abs() < 1e-3);
}

#[test]
fn test_anchor_anim() {
    let anchor = AnchorAnim::new(AnchorAnimType::FastSpatial).with_range(0.0, 100.0);
    assert_eq!(anchor.anim_type.duration_ms(), ANIM_DURATION_EXPRESSIVE_FAST_SPATIAL);
    assert_eq!(anchor.offset_at(Duration::ZERO), 0.0);
    assert!((anchor.offset_at(anchor.duration) - 100.0).abs() < 1e-3);
}

#[test]
fn test_anim_loader_state_machine() {
    let mut loader = AnimLoaderState::new();
    assert_eq!(loader.phase, LoaderPhase::Idle);

    loader.init_source("comp_a");
    assert_eq!(loader.current_id.as_deref(), Some("comp_a"));
    assert_eq!(loader.opacity, 1.0);

    // Transition to comp_b
    loader.set_source("comp_b");
    assert_eq!(loader.phase, LoaderPhase::FadingOut);

    // Advance halfway through out_anim
    let half_out = loader.out_anim.duration / 2;
    loader.tick(half_out);
    assert_eq!(loader.phase, LoaderPhase::FadingOut);
    assert!(loader.opacity < 1.0 && loader.opacity > 0.0);

    // Finish fade out
    loader.tick(half_out);
    // Should transition through Swapping and enter FadingIn
    assert_eq!(loader.phase, LoaderPhase::FadingIn);
    assert_eq!(loader.current_id.as_deref(), Some("comp_b"));

    // Finish fade in
    loader.tick(loader.in_anim.duration);
    assert_eq!(loader.phase, LoaderPhase::Idle);
    assert_eq!(loader.opacity, 1.0);
}
