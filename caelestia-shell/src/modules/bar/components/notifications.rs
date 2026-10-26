//! NotificationsIndicator Widget
//! Parity with `shell/modules/bar/components/NotificationsIndicator.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct NotificationsIndicatorWidget;

impl NotificationsIndicatorWidget {
    /// Renders the notifications toggle pill button.
    pub fn render(count: usize, c: &CaelestiaColours) -> Div {
        Self::render_full(count, false, c)
    }

    /// Renders with count and DND flag.
    pub fn render_full(count: usize, dnd: bool, c: &CaelestiaColours) -> Div {
        Self::render_with_state(count, dnd, false, false, c)
    }

    /// Renders with explicit hover and pressed state layer overlay.
    pub fn render_with_state(
        count: usize,
        dnd: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let icon_name = if dnd {
            "notifications_off"
        } else if count > 0 {
            "notifications_unread"
        } else {
            "notifications"
        };

        let icon_size = ICON_SIZE_SMALL;
        let dim = icon_size + PADDING_MEDIUM * 2.0;

        div()
            .relative()
            .w(px(dim))
            .h(px(dim))
            .rounded_full()
            .bg(rgba(c.surface_container))
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand)
            .child(StateLayer::overlay(
                is_hovered,
                is_pressed,
                false,
                ROUNDING_FULL,
                c.secondary,
            ))
            .child(
                MaterialIcon::with_size(icon_name, IconSize::Small, c.secondary),
            )
    }
}

