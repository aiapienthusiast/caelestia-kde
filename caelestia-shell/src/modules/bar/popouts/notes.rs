//! Notes Popout
//! Parity with `shell/modules/bar/popouts/Notes.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct NotesPopout;

impl NotesPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(
                div()
                    .w_full()
                    .min_h(px(80.0))
                    .p(px(SPACING_SMALL))
                    .rounded(px(ROUNDING_SMALL))
                    .bg(rgba(c.surface_container_low))
                    .text_size(px(13.0))
                    .text_color(rgba(c.on_surface))
                    .child("Sprint 2 in progress: Bar Popouts & Tray integration complete."),
            )
            .child(ListRow::render(
                "Clear Note",
                Some("Discard scratchpad text"),
                Some("delete_outline"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Quick Notes", "edit_note", center_x, content, c)
    }
}
