//! LockStatus Popout
//! Parity with `shell/modules/bar/popouts/LockStatus.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct LockStatusPopout;

impl LockStatusPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Caps Lock",
                Some("Inactive"),
                Some("keyboard_capslock"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Num Lock",
                Some("Active"),
                Some("pin"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Lock Keys", "lock", center_x, content, c)
    }
}
