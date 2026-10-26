//! Notes Widget
//! Parity with `shell/modules/bar/components/Notes.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, CursorStyle, Div, Styled};

pub struct NotesWidget;

impl NotesWidget {
    /// Renders quick scratchpad notes trigger pill.
    pub fn render(c: &CaelestiaColours) -> Div {
        Self::render_with_state(false, false, c)
    }

    /// Renders with explicit hover and pressed states.
    pub fn render_with_state(is_hovered: bool, is_pressed: bool, c: &CaelestiaColours) -> Div {
        let dim = 32.0;

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
                c.primary,
            ))
            .child(
                MaterialIcon::with_size("sticky_note_2", IconSize::Medium, c.primary),
            )
    }
}

