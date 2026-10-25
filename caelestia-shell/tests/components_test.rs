//! Unit tests for Caelestia Shell UI components.

use caelestia_shell::components::controls::button_base::{ButtonBase, ButtonStyle, ButtonType};
use caelestia_shell::components::icon::IconSize;
use caelestia_shell::components::styled_text::TypographyScale;
use caelestia_shell::domain::tokens::*;
use caelestia_shell::infrastructure::theme::CaelestiaColours;

#[test]
fn test_typography_scales() {
    assert_eq!(TypographyScale::DisplayLarge.font_size(), 57.0);
    assert_eq!(TypographyScale::HeadlineLarge.font_size(), 32.0);
    assert_eq!(TypographyScale::TitleMedium.font_size(), 16.0);
    assert_eq!(TypographyScale::BodySmall.font_size(), FONT_SIZE_SMALLER);
    assert_eq!(TypographyScale::LabelSmall.font_size(), FONT_SIZE_SMALL);
}

#[test]
fn test_icon_sizes() {
    assert_eq!(IconSize::Small.pixels(), ICON_SIZE_SMALL);
    assert_eq!(IconSize::Medium.pixels(), ICON_SIZE_MEDIUM);
    assert_eq!(IconSize::Large.pixels(), ICON_SIZE_LARGE);
    assert_eq!(IconSize::Custom(42.0).pixels(), 42.0);
}

#[test]
fn test_button_base_radius_morphing() {
    let mut style = ButtonStyle::default();
    assert_eq!(ButtonBase::calculate_radius(&style, 40.0), ROUNDING_LARGE);

    // Press morphing takes highest precedence
    style.is_pressed = true;
    assert_eq!(ButtonBase::calculate_radius(&style, 40.0), ROUNDING_SMALL);

    // Checked morphing when not pressed
    style.is_pressed = false;
    style.is_checked = true;
    assert_eq!(ButtonBase::calculate_radius(&style, 40.0), ROUNDING_MEDIUM);

    // Circular button
    style.is_checked = false;
    style.is_round = true;
    assert_eq!(ButtonBase::calculate_radius(&style, 40.0), 20.0);
}

#[test]
fn test_button_base_colors() {
    let colours = CaelestiaColours::default();
    let mut style = ButtonStyle::default();

    let active_bg = 0xff0000ff;
    let inactive_bg = 0x00ff00ff;
    let active_on = 0x0000ffff;
    let inactive_on = 0xffffffff;

    // Normal state
    assert_eq!(
        ButtonBase::calculate_bg(&style, &colours, active_bg, inactive_bg),
        inactive_bg
    );
    assert_eq!(
        ButtonBase::calculate_on_colour(&style, &colours, active_on, inactive_on),
        inactive_on
    );

    // Checked state
    style.is_checked = true;
    assert_eq!(
        ButtonBase::calculate_bg(&style, &colours, active_bg, inactive_bg),
        active_bg
    );
    assert_eq!(
        ButtonBase::calculate_on_colour(&style, &colours, active_on, inactive_on),
        active_on
    );

    // Disabled state
    style.is_disabled = true;
    assert_eq!(
        ButtonBase::calculate_bg(&style, &colours, active_bg, inactive_bg),
        colours.disabled_container
    );
    assert_eq!(
        ButtonBase::calculate_on_colour(&style, &colours, active_on, inactive_on),
        colours.disabled_content
    );

    // Text button has transparent bg
    style.is_disabled = false;
    style.button_type = ButtonType::Text;
    assert_eq!(
        ButtonBase::calculate_bg(&style, &colours, active_bg, inactive_bg) & 0xff,
        0
    );
}

#[test]
fn test_design_tokens_parity() {
    assert_eq!(ROUNDING_EXTRA_SMALL, 4.0);
    assert_eq!(ROUNDING_SMALL, 8.0);
    assert_eq!(ROUNDING_MEDIUM, 12.0);
    assert_eq!(ROUNDING_LARGE, 16.0);
    assert_eq!(ROUNDING_NORMAL, 24.0);
    assert_eq!(ROUNDING_EXTRA_LARGE, 28.0);
    assert_eq!(ROUNDING_FULL, 9999.0);

    assert_eq!(SPACING_SMALL, 8.0);
    assert_eq!(SPACING_MEDIUM, 12.0);
    assert_eq!(SPACING_LARGE, 16.0);

    assert_eq!(PADDING_SMALL, 8.0);
    assert_eq!(PADDING_MEDIUM, 12.0);
    assert_eq!(PADDING_LARGE, 16.0);

    assert_eq!(CHECKBOX_SIZE, 24.0);
    assert_eq!(RADIO_SIZE, 20.0);
    assert_eq!(RADIO_INNER_SIZE, 8.0);
    assert_eq!(SWITCH_TRACK_WIDTH, 52.0);
    assert_eq!(SWITCH_TRACK_HEIGHT, 32.0);
}
