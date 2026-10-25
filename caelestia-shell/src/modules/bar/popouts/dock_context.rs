//! DockContext Popout
//! Parity with `shell/modules/bar/popouts/DockContext.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct DockContextPopout;

impl DockContextPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Minimize Window",
                None::<&str>,
                Some("minimize"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Maximize / Restore",
                None::<&str>,
                Some("crop_square"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Close Window",
                None::<&str>,
                Some("close"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Window Options", "window", center_x, content, c)
    }
}
