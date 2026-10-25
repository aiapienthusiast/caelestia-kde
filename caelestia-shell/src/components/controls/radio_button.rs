//! StyledRadioButton
//! Parity with `shell/components/controls/StyledRadioButton.qml`.

use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, SharedString, Styled};

pub struct StyledRadioButton;

impl StyledRadioButton {
    /// Renders an M3 RadioButton indicator.
    pub fn render(
        is_checked: bool,
        is_hovered: bool,
        is_pressed: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let outer_size = RADIO_SIZE;
        let inner_size = RADIO_INNER_SIZE;

        let border_color = if is_disabled {
            c.disabled_container
        } else if is_checked {
            c.primary
        } else {
            c.on_surface_variant
        };

        let inner_color = if is_disabled {
            c.disabled_content
        } else if is_checked {
            c.primary
        } else {
            0x00000000
        };

        let mut circle = div()
            .relative()
            .w(px(outer_size))
            .h(px(outer_size))
            .rounded(px(ROUNDING_FULL))
            .border_2()
            .border_color(rgba(border_color))
            .flex()
            .items_center()
            .justify_center();

        if !is_disabled {
            circle = circle.cursor(CursorStyle::PointingHand);
        }

        circle = circle.child(StateLayer::overlay(
            is_hovered,
            is_pressed,
            is_disabled,
            outer_size / 2.0,
            c.primary,
        ));

        if is_checked {
            circle = circle.child(
                div()
                    .w(px(inner_size))
                    .h(px(inner_size))
                    .rounded(px(ROUNDING_FULL))
                    .bg(rgba(inner_color)),
            );
        }

        circle
    }

    /// Renders a labeled RadioButton.
    pub fn labeled(
        label: impl Into<SharedString>,
        is_checked: bool,
        is_hovered: bool,
        is_pressed: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let label_color = if is_disabled {
            c.disabled_content
        } else {
            c.on_surface
        };

        div()
            .flex()
            .items_center()
            .gap(px(SPACING_MEDIUM))
            .child(Self::render(is_checked, is_hovered, is_pressed, is_disabled, c))
            .child(
                div()
                    .text_size(px(FONT_SIZE_NORMAL))
                    .text_color(rgba(label_color))
                    .child(label.into()),
            )
    }
}
