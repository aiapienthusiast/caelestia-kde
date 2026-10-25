//! TrayMenu Popout
//! Parity with `shell/modules/bar/popouts/TrayMenu.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct TrayMenuPopout;

impl TrayMenuPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Open Main Window",
                None::<&str>,
                Some("launch"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Quit Application",
                None::<&str>,
                Some("exit_to_app"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Application Menu", "menu", center_x, content, c)
    }
}
