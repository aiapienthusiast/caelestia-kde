//! ScreenCapture Widget
//! Parity with `shell/modules/bar/components/ScreenCaptureWidget.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct ScreenCaptureWidget;

impl ScreenCaptureWidget {
    /// Renders the screenshot / recording trigger button pill.
    pub fn render(is_recording: bool, c: &CaelestiaColours) -> Div {
        Self::render_with_state(is_recording, false, false, false, c)
    }

    /// Renders with explicit popout open and hover/pressed states.
    pub fn render_with_state(
        is_recording: bool,
        is_open: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let icon_name = if is_recording {
            "screen_record"
        } else {
            "photo_camera"
        };

        let bg_color = if is_open {
            c.secondary_container
        } else if is_hovered {
            c.surface_container_high
        } else {
            0x00000000
        };

        let icon_color = if is_open {
            c.on_secondary_container
        } else {
            c.primary
        };

        let dim = 32.0;

        div()
            .relative()
            .w(px(dim))
            .h(px(dim))
            .rounded_full()
            .bg(rgba(bg_color))
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
                MaterialIcon::with_size(icon_name, IconSize::Medium, icon_color),
            )
    }
}

