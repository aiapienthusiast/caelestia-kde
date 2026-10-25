//! Shortcuts Popout
//! Parity with `shell/modules/bar/popouts/Shortcuts.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct ShortcutsPopout;

impl ShortcutsPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Application Launcher",
                Some("Super + Space"),
                Some("search"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Overview Workspace",
                Some("Super + Tab"),
                Some("view_carousel"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Terminal Window",
                Some("Super + Return"),
                Some("terminal"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Keyboard Shortcuts", "keyboard", center_x, content, c)
    }
}
