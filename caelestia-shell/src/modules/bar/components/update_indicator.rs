//! UpdateIndicator Widget
//! Parity with `shell/modules/bar/components/UpdateIndicator.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, CursorStyle, Div, Styled};

pub struct UpdateIndicatorWidget;

impl UpdateIndicatorWidget {
    /// Renders pending software updates badge pill.
    pub fn render(update_count: usize, c: &CaelestiaColours) -> Div {
        Self::render_with_state(update_count > 0, false, false, false, c)
    }

    /// Renders with explicit has_update, checking, and hover/pressed states.
    pub fn render_with_state(
        has_update: bool,
        checking: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let icon_name = if checking {
            "sync"
        } else if has_update {
            "update"
        } else {
            "check_circle"
        };

        let icon_color = if has_update {
            c.primary
        } else {
            c.secondary
        };

        let dim = ICON_SIZE_SMALL + PADDING_SMALL * 2.0;

        div()
            .relative()
            .w(px(dim))
            .h(px(dim))
            .rounded_full()
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand)
            .child(StateLayer::overlay(
                is_hovered,
                is_pressed,
                false,
                ROUNDING_FULL,
                icon_color,
            ))
            .child(
                MaterialIcon::with_size(icon_name, IconSize::Small, icon_color),
            )
    }
}

