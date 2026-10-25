//! TextButton
//! Parity with `shell/components/controls/TextButton.qml`.

use super::button_base::{ButtonBase, ButtonStyle, ButtonType};
use crate::components::state_layer::StateLayer;
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, SharedString};

pub struct TextButton;

impl TextButton {
    /// Renders an M3 TextButton with full visual and behavioral parity.
    pub fn render(
        text: impl Into<SharedString>,
        style: &ButtonStyle,
        c: &CaelestiaColours,
    ) -> Div {
        let height_px = 36.0;
        let radius = ButtonBase::calculate_radius(style, height_px);

        // Colors directly matching TextButton.qml:15-32
        let active_color = if style.button_type == ButtonType::Filled {
            c.primary
        } else {
            c.secondary
        };

        let inactive_color = if !style.is_toggle && style.button_type == ButtonType::Filled {
            c.primary
        } else if style.button_type == ButtonType::Filled {
            c.surface_container
        } else {
            c.secondary_container
        };

        let active_on_color = if style.button_type == ButtonType::Text {
            c.primary
        } else if style.button_type == ButtonType::Filled {
            c.on_primary
        } else {
            c.on_secondary
        };

        let inactive_on_color = if !style.is_toggle && style.button_type == ButtonType::Filled {
            c.on_primary
        } else if style.button_type == ButtonType::Text {
            c.primary
        } else if style.button_type == ButtonType::Filled {
            c.on_surface
        } else {
            c.on_secondary_container
        };

        let bg_color = ButtonBase::calculate_bg(style, c, active_color, inactive_color);
        let on_color = ButtonBase::calculate_on_colour(style, c, active_on_color, inactive_on_color);

        let border_color = if style.is_focused {
            Some(c.primary)
        } else {
            None
        };

        let container = ButtonBase::container(style, bg_color, radius, border_color)
            .h(px(height_px))
            .px(px(PADDING_MEDIUM))
            .py(px(PADDING_SMALL));

        container
            .child(StateLayer::overlay(
                style.is_hovered,
                style.is_pressed,
                style.is_disabled,
                radius,
                on_color,
            ))
            .child(
                div()
                    .text_size(px(TypographyScale::LabelLarge.font_size()))
                    .font_weight(TypographyScale::LabelLarge.default_weight())
                    .text_color(rgba(on_color))
                    .child(text.into()),
            )
    }
}
