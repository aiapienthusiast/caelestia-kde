//! StyledSwitch
//! Parity with `shell/components/controls/StyledSwitch.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct StyledSwitch;

impl StyledSwitch {
    /// Renders an M3 toggle switch with full visual parity to StyledSwitch.qml.
    pub fn render(
        is_checked: bool,
        is_hovered: bool,
        is_pressed: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let track_width = SWITCH_TRACK_WIDTH;
        let track_height = SWITCH_TRACK_HEIGHT;
        let thumb_height = SWITCH_THUMB_SIZE;
        let thumb_width = if is_pressed { thumb_height * 1.2 } else { thumb_height };

        let padding = (track_height - thumb_height) / 2.0;

        let track_color = if is_disabled {
            c.disabled_container
        } else if is_checked {
            c.primary
        } else {
            c.surface_container_highest
        };

        let thumb_color = if is_disabled {
            c.disabled_content
        } else if is_checked {
            c.on_primary
        } else {
            c.outline
        };

        let thumb_left = if is_checked {
            track_width - thumb_width - padding
        } else {
            padding
        };

        let mut track = div()
            .relative()
            .w(px(track_width))
            .h(px(track_height))
            .rounded(px(ROUNDING_FULL))
            .bg(rgba(track_color));

        if !is_disabled {
            track = track.cursor(CursorStyle::PointingHand);
        }

        // Thumb element with M3 state layer
        let thumb = div()
            .relative()
            .absolute()
            .top(px(padding))
            .left(px(thumb_left))
            .w(px(thumb_width))
            .h(px(thumb_height))
            .rounded(px(ROUNDING_FULL))
            .bg(rgba(thumb_color))
            .flex()
            .items_center()
            .justify_center()
            .child(crate::components::state_layer::StateLayer::overlay(
                is_hovered,
                is_pressed,
                is_disabled,
                thumb_height / 2.0,
                if is_checked { c.primary } else { c.on_surface },
            ));

        track.child(thumb)
    }
}
