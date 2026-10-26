//! NightLight Popout
//! 100% Parity with `shell/modules/bar/popouts/NightLight.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::icon_text_button::IconTextButton;
use crate::components::controls::slider::StyledSlider;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

/// State of night light temperature and solar scheduling (port of `HyprSunset` service).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NightLightPopoutState {
    pub auto_mode: bool,
    pub day_temp_k: u32,
    pub night_temp_k: u32,
}

impl Default for NightLightPopoutState {
    fn default() -> Self {
        Self {
            auto_mode: true,
            day_temp_k: 6000,
            night_temp_k: 3800,
        }
    }
}

impl NightLightPopoutState {
    pub const MIN_TEMP_K: u32 = 2000;
    pub const MAX_TEMP_K: u32 = 6500;
    pub const TEMP_RANGE: f32 = 4500.0;

    pub fn day_slider_value(&self) -> f32 {
        ((self.day_temp_k.saturating_sub(Self::MIN_TEMP_K)) as f32 / Self::TEMP_RANGE).clamp(0.0, 1.0)
    }

    pub fn night_slider_value(&self) -> f32 {
        ((self.night_temp_k.saturating_sub(Self::MIN_TEMP_K)) as f32 / Self::TEMP_RANGE).clamp(0.0, 1.0)
    }
}

pub struct NightLightPopout;

impl NightLightPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &NightLightPopoutState::default(), c)
    }

    /// Renders the night light popout with full parity to NightLight.qml.
    pub fn render_with_state(center_x: f32, state: &NightLightPopoutState, c: &CaelestiaColours) -> Div {
        let width_px = 300.0;

        let mut content = div()
            .flex()
            .flex_col()
            .w(px(width_px))
            .gap(px(SPACING_SMALL));

        // 1. Header: "Night Light"
        content = content.child(
            StyledText::new("Night Light")
                .text_size(px(FONT_SIZE_NORMAL))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.on_surface)),
        );

        // 2. Auto / Manual Switch Button
        let mut btn_style = ButtonStyle::default();
        btn_style.button_type = if state.auto_mode {
            ButtonType::Filled
        } else {
            ButtonType::Tonal
        };

        let mode_text = if state.auto_mode { "Auto" } else { "Manual" };
        content = content.child(
            div()
                .mt(px(SPACING_EXTRA_SMALL))
                .child(IconTextButton::render("routine", mode_text, &btn_style, c)),
        );

        // 3. Daylight Temperature (only visible in Auto mode)
        if state.auto_mode {
            let day_label = format!("Daylight Temperature ({}K)", state.day_temp_k);

            content = content.child(
                div()
                    .mt(px(SPACING_SMALL))
                    .child(
                        StyledText::new(day_label)
                            .text_size(px(FONT_SIZE_NORMAL))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(c.on_surface)),
                    ),
            );

            content = content.child(
                div()
                    .w_full()
                    .h(px(PADDING_MEDIUM * 3.0))
                    .flex()
                    .items_center()
                    .child(StyledSlider::render(
                        state.day_slider_value(),
                        width_px,
                        false,
                        false,
                        false,
                        c,
                    )),
            );
        }

        // 4. Nightlight / Current Temperature
        let night_label = if state.auto_mode {
            format!("Nightlight Temperature ({}K)", state.night_temp_k)
        } else {
            format!("Temperature ({}K)", state.night_temp_k)
        };

        content = content.child(
            div()
                .mt(px(SPACING_SMALL))
                .child(
                    StyledText::new(night_label)
                        .text_size(px(FONT_SIZE_NORMAL))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(c.on_surface)),
                ),
        );

        content = content.child(
            div()
                .w_full()
                .h(px(PADDING_MEDIUM * 3.0))
                .flex()
                .items_center()
                .child(StyledSlider::render(
                    state.night_slider_value(),
                    width_px,
                    false,
                    false,
                    false,
                    c,
                )),
        );

        PopoutWrapper::render("Night Light", "nights_stay", center_x, content, c)
    }
}
