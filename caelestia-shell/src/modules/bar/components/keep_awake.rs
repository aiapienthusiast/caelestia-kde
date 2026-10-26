//! KeepAwake Widget
//! Parity with `shell/modules/bar/components/KeepAwakeWidget.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct KeepAwakeWidget;

impl KeepAwakeWidget {
    /// Renders the caffeine / sleep inhibitor toggle pill.
    pub fn render(is_active: bool, c: &CaelestiaColours) -> Div {
        Self::render_with_state(is_active, false, false, c)
    }

    /// Renders with explicit hover and pressed states.
    pub fn render_with_state(
        is_active: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let (bg_col, fg_col) = if is_active {
            (c.secondary_container, c.on_secondary_container)
        } else {
            (c.surface_container_high, c.on_surface_variant)
        };

        let dim = 32.0;

        div()
            .relative()
            .w(px(dim))
            .h(px(dim))
            .rounded_full()
            .bg(rgba(bg_col))
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand)
            .child(StateLayer::overlay(
                is_hovered,
                is_pressed,
                false,
                ROUNDING_FULL,
                fg_col,
            ))
            .child(
                MaterialIcon::with_size("coffee", IconSize::Medium, fg_col),
            )
    }
}

