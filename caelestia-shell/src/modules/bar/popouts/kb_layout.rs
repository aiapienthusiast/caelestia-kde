//! Keyboard Layout Popout
//! Parity with `shell/modules/bar/popouts/kblayout/KbLayout.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct KbLayoutPopout;

impl KbLayoutPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "English (US)",
                Some("Active Layout"),
                Some("check"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "German (DE)",
                Some("Standard German"),
                Some("language"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Keyboard Layout", "keyboard", center_x, content, c)
    }
}
