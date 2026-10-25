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

#[test]
fn test_m3_controls_render_contract() {
    use caelestia_shell::components::containers::{SectionHeader, StyledFlickable, VerticalFadeListView};
    use caelestia_shell::components::controls::{
        ListRow, Menu, MenuItem, SearchBar, SelectRow, SliderRow, StepperRow, ToggleRow,
    };

    let c = CaelestiaColours::default();

    // Verify render functions execute without panics and produce Divs
    let _list_row = ListRow::render(
        "Wi-Fi",
        Some("Connected to Home-5G"),
        Some("wifi"),
        None,
        false,
        false,
        false,
        &c,
    );

    let _toggle_row = ToggleRow::render(
        "Bluetooth",
        Some("Visible to nearby devices"),
        Some("bluetooth"),
        true,
        false,
        false,
        false,
        &c,
    );

    let _slider_row = SliderRow::render(
        "Display Brightness",
        "75%",
        0.75,
        200.0,
        Some("brightness_6"),
        false,
        false,
        false,
        &c,
    );

    let _stepper_row = StepperRow::render(
        "Volume Boost",
        Some("Max output amplification"),
        "120%",
        Some("volume_up"),
        false,
        false,
        false,
        &c,
    );

    let _select_row = SelectRow::render(
        "Power Profile",
        Some("Performance strategy"),
        "Balanced",
        Some("speed"),
        false,
        false,
        false,
        &c,
    );

    let _search_bar = SearchBar::render(
        "terminal",
        "Search applications...",
        true,
        false,
        false,
        &c,
    );

    let _menu_item = MenuItem::render(
        "Open in Terminal",
        Some("terminal"),
        Some("Ctrl+Shift+T"),
        false,
        false,
        false,
        false,
        &c,
    );

    let _menu = Menu::render(200.0, vec![_menu_item], &c);

    let _section = SectionHeader::render("Networking", Some("Manage interfaces"), &c);

    let _flickable = StyledFlickable::render("test_flick", Some(300.0), vec![_list_row]);

    let _fade_list = VerticalFadeListView::render("test_fade", Some(400.0), 8.0, vec![_toggle_row], &c);
}

#[test]
fn test_elevation_and_effects() {
    use caelestia_shell::components::effects::{Elevation, ElevationLevel, InnerBorder};
    use gpui::div;

    let c = CaelestiaColours::default();

    let _l0 = Elevation::apply(div(), ElevationLevel::Level0);
    let _l1 = Elevation::apply(div(), ElevationLevel::Level1);
    let _l2 = Elevation::apply(div(), ElevationLevel::Level2);
    let _l3 = Elevation::apply(div(), ElevationLevel::Level3);
    let _l4 = Elevation::apply(div(), ElevationLevel::Level4);
    let _l5 = Elevation::apply(div(), ElevationLevel::Level5);

    let _inner_border = InnerBorder::overlay(12.0, &c);
}

#[test]
fn test_toggle_button_behavior() {
    use caelestia_shell::components::controls::toggle_button::ToggleButton;

    let c = CaelestiaColours::default();

    let mut btn = ToggleButton::new(false)
        .with_icon("mic")
        .with_label("Microphone")
        .with_accent("Primary");

    assert_eq!(btn.corner_radius(), ROUNDING_FULL);
    let (bg_untoggled, _fg_untoggled) = btn.resolve_colors(&c);
    assert_eq!(bg_untoggled, gpui::rgba(c.primary_container));

    btn.toggled = true;
    assert_eq!(btn.corner_radius(), ROUNDING_MEDIUM);
    let (bg_toggled, _fg_toggled) = btn.resolve_colors(&c);
    assert_eq!(bg_toggled, gpui::rgba(c.primary));
}

#[test]
fn test_filled_slider_behavior() {
    use caelestia_shell::components::controls::filled_slider::{FilledSlider, SliderOrientation};

    let slider_v = FilledSlider::new(0.75, "volume_up");
    assert_eq!(slider_v.progress_percent(), 75);
    assert_eq!(slider_v.orientation, SliderOrientation::Vertical);

    let slider_h = FilledSlider::new(1.5, "brightness_medium")
        .with_orientation(SliderOrientation::Horizontal)
        .with_dimensions(32.0, 200.0);
    assert_eq!(slider_h.progress_percent(), 100); // clamped to 1.0
    assert_eq!(slider_h.orientation, SliderOrientation::Horizontal);
}

#[test]
fn test_circular_controls() {
    use caelestia_shell::components::controls::circular_indicator::{CircularIndicator, IndicatorAnimState};
    use caelestia_shell::components::controls::circular_progress::CircularProgress;

    let mut indicator = CircularIndicator::new(true).with_size(56.0);
    assert_eq!(indicator.anim_state, IndicatorAnimState::Running);
    assert_eq!(indicator.rotation_deg, 0.0);
    indicator.step(0.5);
    assert_eq!(indicator.rotation_deg, 180.0);

    let progress = CircularProgress::new(0.42)
        .with_size(80.0)
        .with_stroke_width(6.0);
    assert!((progress.clamped_value() - 0.42).abs() < 1e-4);
    assert_eq!(progress.arc_radius(), 37.0);
}

#[test]
fn test_setting_reset_button() {
    use caelestia_shell::components::controls::setting_reset_button::{SettingResetButton, SettingResetOptions};

    let numeric_opt = SettingResetOptions::new_numeric(1.0, 1.0);
    assert!(!numeric_opt.is_dirty());

    let dirty_opt = SettingResetOptions::new_numeric(1.05, 1.0);
    assert!(dirty_opt.is_dirty());

    let str_opt = SettingResetOptions::new_string("dark", "light");
    assert!(str_opt.is_dirty());

    let mut btn = SettingResetButton::new().with_options(Some(&dirty_opt));
    assert!(btn.dirty);
    assert!(btn.latched);
    assert!(!btn.is_disabled());

    btn.reset();
    assert!(!btn.latched);
}

#[test]
fn test_split_button_and_row() {
    use caelestia_shell::components::controls::split_button::{SplitButton, SplitButtonItem, SplitButtonType};
    use caelestia_shell::components::controls::split_button_row::SplitButtonRow;

    let mut split_btn = SplitButton::new("terminal", "Launch Terminal")
        .button_type(SplitButtonType::Tonal)
        .disabled(false);

    assert_eq!(split_btn.current_icon(), "terminal");
    assert_eq!(split_btn.current_text(), "Launch Terminal");

    split_btn.active_item = Some(SplitButtonItem {
        id: "bash".into(),
        icon: Some("code".into()),
        text: "Bash Console".into(),
    });
    assert_eq!(split_btn.current_icon(), "code");
    assert_eq!(split_btn.current_text(), "Bash Console");

    let row = SplitButtonRow::new("Default Terminal", "terminal", "Terminal")
        .enabled(true);
    assert_eq!(row.label, "Default Terminal");
    assert!(row.enabled);
}

#[test]
fn test_animated_password_mask() {
    use caelestia_shell::components::controls::password_mask::AnimatedPasswordMask;

    let mask = AnimatedPasswordMask::new("1234")
        .dot_size(12.0)
        .spacing(6.0);

    assert_eq!(mask.char_count(), 4);
    assert_eq!(mask.full_width(), 4.0 * 12.0 + 3.0 * 6.0); // 48 + 18 = 66
    assert!(!mask.show_password);
}

#[test]
fn test_custom_mouse_area() {
    use caelestia_shell::components::controls::mouse_area::CustomMouseArea;

    let mut mouse = CustomMouseArea::new();
    assert_eq!(mouse.threshold, 120);

    // Partial accumulation (no trigger)
    assert_eq!(mouse.handle_wheel_delta(60), None);
    assert_eq!(mouse.scroll_accumulated_y, 60);

    // Direction reversal resets accumulation
    assert_eq!(mouse.handle_wheel_delta(-30), None);
    assert_eq!(mouse.scroll_accumulated_y, -30);

    // Threshold crossing triggers notch count
    assert_eq!(mouse.handle_wheel_delta(-100), Some(-1));
    assert_eq!(mouse.scroll_accumulated_y, 0);
}

#[test]
fn test_styled_scroll_bar() {
    use caelestia_shell::components::controls::scroll_bar::{ScrollBarOrientation, ScrollBarPolicy, StyledScrollBar};

    let mut bar = StyledScrollBar::vertical()
        .policy(ScrollBarPolicy::AsNeeded)
        .position(0.5)
        .size(0.3);

    assert_eq!(bar.orientation, ScrollBarOrientation::Vertical);
    assert_eq!(bar.opacity(), 0.0); // idle with AsNeeded

    bar.active = true;
    assert_eq!(bar.opacity(), 0.6);

    bar.hovered = true;
    assert_eq!(bar.opacity(), 0.8);

    bar.pressed = true;
    assert_eq!(bar.opacity(), 1.0);

    // When content fits (size = 1.0), bar stays hidden
    bar.size = 1.0;
    assert_eq!(bar.opacity(), 0.0);
}

#[test]
fn test_styled_input_field_and_text_base() {
    use caelestia_shell::components::controls::input_field::{InputTextAlignment, StyledInputField};
    use caelestia_shell::components::controls::text_field_base::{TextEchoMode, TextFieldBase};

    let input = StyledInputField::new()
        .text("hello")
        .placeholder("Type here...")
        .alignment(InputTextAlignment::Left);
    assert_eq!(input.text, "hello");
    assert_eq!(input.placeholder_text, "Type here...");

    let mut base = TextFieldBase::new()
        .text("secret123")
        .echo_mode(TextEchoMode::Password);
    assert_eq!(base.display_text(), "•••••••••");

    base.echo_mode = TextEchoMode::Normal;
    assert_eq!(base.display_text(), "secret123");
}

#[test]
fn test_appstream_row() {
    use caelestia_shell::components::controls::appstream_row::AppStreamRow;

    let mut row = AppStreamRow::new(42, "Spotify")
        .volume(0.85);

    assert_eq!(row.volume_icon(), "volume_up");
    assert_eq!(row.value_label(), "85%");

    row.volume = 0.5;
    assert_eq!(row.volume_icon(), "volume_down");

    row.volume = 0.2;
    assert_eq!(row.volume_icon(), "volume_mute");

    row.muted = true;
    assert_eq!(row.volume_icon(), "volume_off");
    assert_eq!(row.value_label(), "Muted");
}

#[test]
fn test_containers_parity() {
    use caelestia_shell::components::containers::fade_flickable::VerticalFadeFlickable;
    use caelestia_shell::components::containers::list_view::StyledListView;
    use caelestia_shell::components::containers::window::{StyledWindow, WindowLayerNamespace};

    let mut flick = VerticalFadeFlickable::new("flick");
    flick.update_scroll(50.0, 500.0, 200.0);
    assert!(flick.is_top_fade_active());
    assert!(flick.is_bottom_fade_active());

    let list = StyledListView::new("list", 25);
    assert_eq!(list.item_count, 25);
    assert_eq!(list.max_velocity, 3000.0);

    let win = StyledWindow::new("caelestia-bar")
        .desktop_widget(true)
        .screen_name("DP-1");
    assert_eq!(win.namespace, WindowLayerNamespace::Desktop);
    assert_eq!(win.screen_name, "DP-1");
}


