//! LockStatus Popout
//! 100% Parity with `shell/modules/bar/popouts/LockStatus.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

/// State of keyboard locks from KWin/XKB (port of `Kwin.capsLock` and `Kwin.numLock`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LockStatusState {
    pub caps_lock: bool,
    pub num_lock: bool,
}

pub struct LockStatusPopout;

impl LockStatusPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &LockStatusState::default(), c)
    }

    /// Renders the keyboard lock keys popout with full parity to LockStatus.qml.
    pub fn render_with_state(center_x: f32, state: &LockStatusState, c: &CaelestiaColours) -> Div {
        let width_px = 300.0;

        let mut content = div()
            .flex()
            .flex_col()
            .w(px(width_px))
            .gap(px(SPACING_SMALL));

        // 1. Header: "Keyboard Locks"
        content = content.child(
            StyledText::new("Keyboard Locks")
                .text_size(px(FONT_SIZE_NORMAL))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.on_surface)),
        );

        // 2. Card with Lock Statuses
        let card = div()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .flex()
            .flex_col()
            .gap(px(SPACING_MEDIUM))
            // Caps Lock Line
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(SPACING_SMALL))
                    .child(
                        MaterialIcon::new("keyboard_capslock")
                            .text_size(px(ICON_SIZE_SMALL))
                            .text_color(rgba(if state.caps_lock { c.primary } else { c.on_surface_variant })),
                    )
                    .child(
                        StyledText::new(if state.caps_lock {
                            "Caps Lock enabled"
                        } else {
                            "Caps Lock disabled"
                        })
                        .text_size(px(FONT_SIZE_NORMAL))
                        .font_weight(if state.caps_lock { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                        .text_color(rgba(if state.caps_lock { c.on_surface } else { c.on_surface_variant })),
                    ),
            )
            // Num Lock Line
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(SPACING_SMALL))
                    .child(
                        MaterialIcon::new("pin")
                            .text_size(px(ICON_SIZE_SMALL))
                            .text_color(rgba(if state.num_lock { c.primary } else { c.on_surface_variant })),
                    )
                    .child(
                        StyledText::new(if state.num_lock {
                            "Num Lock enabled"
                        } else {
                            "Num Lock disabled"
                        })
                        .text_size(px(FONT_SIZE_NORMAL))
                        .font_weight(if state.num_lock { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                        .text_color(rgba(if state.num_lock { c.on_surface } else { c.on_surface_variant })),
                    ),
            );

        content = content.child(card);

        PopoutWrapper::render("Keyboard Locks", "lock", center_x, content, c)
    }
}
