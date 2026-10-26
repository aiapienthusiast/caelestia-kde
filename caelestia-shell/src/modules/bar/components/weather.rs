//! Weather Widget
//! Parity with `shell/modules/bar/components/WeatherWidget.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct WeatherWidget;

impl WeatherWidget {
    /// Renders weather icon and temperature readout pill with default horizontal layout.
    pub fn render(temperature: i32, icon: &str, c: &CaelestiaColours) -> Div {
        Self::render_full(&format!("{}°C", temperature), icon, true, false, false, c)
    }

    /// Renders with explicit temperature string, layout orientation, and hover/pressed states.
    pub fn render_full(
        temp_str: &str,
        icon: &str,
        is_horizontal: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let padding_h = PADDING_MEDIUM;
        let padding_v = PADDING_SMALL;

        let mut row = div()
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(SPACING_EXTRA_SMALL))
            .px(px(padding_h))
            .py(px(padding_v))
            .rounded_full()
            .cursor(CursorStyle::PointingHand)
            .child(StateLayer::overlay(
                is_hovered,
                is_pressed,
                false,
                ROUNDING_FULL,
                c.secondary,
            ))
            .child(
                MaterialIcon::with_size(icon, IconSize::Medium, c.secondary),
            );

        if is_horizontal && !temp_str.is_empty() {
            row = row.child(
                StyledText::new(temp_str)
                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                    .text_color(rgba(c.on_surface)),
            );
        }

        row
    }
}

