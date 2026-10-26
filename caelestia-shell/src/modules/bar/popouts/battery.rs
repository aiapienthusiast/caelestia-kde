//! Battery Popout
//! 100% Parity with `shell/modules/bar/popouts/Battery.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, relative, rgba, Div, FontWeight, Styled};

/// Battery device state and power profile diagnostics (port of UPower & PowerProfiles).
#[derive(Clone, Debug, PartialEq)]
pub struct BatteryPopoutState {
    pub percentage: f32, // 0.0 to 100.0
    pub is_charging: bool,
    pub is_plugged_in: bool,
    pub time_remaining: Option<String>,
    pub power_draw_w: Option<f32>,
    pub health_percent: Option<f32>,
    pub degradation_reason: Option<String>,
}

impl Default for BatteryPopoutState {
    fn default() -> Self {
        Self {
            percentage: 85.0,
            is_charging: false,
            is_plugged_in: false,
            time_remaining: Some("4h 15m remaining".into()),
            power_draw_w: Some(14.2),
            health_percent: Some(96.0),
            degradation_reason: None,
        }
    }
}

pub struct BatteryPopout;

impl BatteryPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &BatteryPopoutState::default(), c)
    }

    /// Renders the battery popout with full parity to Battery.qml.
    pub fn render_with_state(center_x: f32, state: &BatteryPopoutState, c: &CaelestiaColours) -> Div {
        let width_px = 300.0;

        let mut content = div()
            .flex()
            .flex_col()
            .w(px(width_px))
            .gap(px(SPACING_MEDIUM));

        // 1. Header: "Battery"
        content = content.child(
            StyledText::new("Battery")
                .text_size(px(FONT_SIZE_NORMAL))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.on_surface)),
        );

        // 2. Battery Graphic & Metrics Hero Card
        let fill_pct = (state.percentage / 100.0).clamp(0.0, 1.0);
        let liquid_color = if fill_pct <= 0.2 && !state.is_charging {
            c.error
        } else {
            c.primary
        };

        // Vertical battery silhouette
        let battery_icon_col = div()
            .w(px(56.0))
            .h(px(90.0))
            .flex()
            .flex_col()
            .items_center()
            // Top terminal nub
            .child(
                div()
                    .w(px(22.0))
                    .h(px(8.0))
                    .rounded_t(px(ROUNDING_EXTRA_SMALL))
                    .bg(rgba(liquid_color)),
            )
            // Outer casing
            .child(
                div()
                    .w(px(52.0))
                    .h(px(82.0))
                    .rounded(px(ROUNDING_SMALL))
                    .border_2()
                    .border_color(rgba(c.outline))
                    .p(px(2.0))
                    .flex()
                    .flex_col()
                    .justify_end()
                    // Liquid level fill
                    .child(
                        div()
                            .w_full()
                            .h(relative(fill_pct))
                            .rounded(px(ROUNDING_EXTRA_SMALL))
                            .bg(rgba(liquid_color)),
                    ),
            );

        // Right details text column
        let status_str = if state.is_charging {
            "Charging"
        } else if state.is_plugged_in {
            "Plugged in (Full)"
        } else {
            "Discharging"
        };

        let mut details_col = div()
            .flex()
            .flex_col()
            .justify_center()
            .gap(px(SPACING_EXTRA_SMALL))
            // Large percentage readout
            .child(
                StyledText::new(format!("{}%", state.percentage.round() as i32))
                    .text_size(px(FONT_SIZE_EXTRA_LARGE))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(c.on_surface)),
            )
            // Charging status
            .child(
                StyledText::new(status_str)
                    .text_size(px(FONT_SIZE_NORMAL))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(c.on_surface_variant)),
            );

        if let Some(ref time_str) = state.time_remaining {
            details_col = details_col.child(
                StyledText::new(time_str.clone())
                    .text_size(px(FONT_SIZE_SMALL))
                    .text_color(rgba(c.on_surface_variant)),
            );
        }

        if let Some(w) = state.power_draw_w {
            details_col = details_col.child(
                StyledText::new(format!("{:.1} W power draw", w))
                    .text_size(px(FONT_SIZE_SMALL))
                    .text_color(rgba(c.on_surface_variant)),
            );
        }

        let hero_card = div()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(SPACING_LARGE))
            .child(battery_icon_col)
            .child(details_col);

        content = content.child(hero_card);

        // 3. Degradation warning if any
        if let Some(ref reason) = state.degradation_reason {
            content = content.child(
                div()
                    .w_full()
                    .p(px(PADDING_SMALL))
                    .rounded(px(ROUNDING_SMALL))
                    .bg(rgba(c.error_container))
                    .child(
                        StyledText::new(format!("Performance reduced: {}", reason))
                            .text_size(px(FONT_SIZE_SMALL))
                            .text_color(rgba(c.on_error_container)),
                    ),
            );
        }

        PopoutWrapper::render("Battery", "battery_full", center_x, content, c)
    }
}
