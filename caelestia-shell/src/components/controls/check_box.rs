//! CheckBox
//! Parity with `shell/components/controls/CheckBox.qml`.

use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, SharedString, Styled};

pub struct CheckBox;

impl CheckBox {
    /// Renders an M3 CheckBox with 24x24 geometry and animated check glyph.
    pub fn render(
        is_checked: bool,
        is_hovered: bool,
        is_pressed: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let size = CHECKBOX_SIZE;
        let radius = ROUNDING_SMALL;

        let border_color = if is_disabled {
            c.disabled_container
        } else if is_checked {
            c.primary
        } else {
            c.on_surface_variant
        };

        let check_color = if is_disabled {
            c.disabled_content
        } else {
            c.primary
        };

        let mut box_el = div()
            .relative()
            .w(px(size))
            .h(px(size))
            .rounded(px(radius))
            .border_2()
            .border_color(rgba(border_color))
            .flex()
            .items_center()
            .justify_center();

        if !is_disabled {
            box_el = box_el.cursor(CursorStyle::PointingHand);
        }

        box_el = box_el.child(StateLayer::overlay(
            is_hovered,
            is_pressed,
            is_disabled,
            radius,
            c.primary,
        ));

        if is_checked {
            box_el = box_el.child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(check_color))
                    .child("✓"),
            );
        }

        box_el
    }

    /// Renders a labeled CheckBox with spacing.
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
