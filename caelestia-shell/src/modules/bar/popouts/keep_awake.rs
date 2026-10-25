//! KeepAwake Popout
//! Parity with `shell/modules/bar/popouts/KeepAwake.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::components::controls::toggle_row::ToggleRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct KeepAwakePopout;

impl KeepAwakePopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ToggleRow::render(
                "Inhibit Sleep",
                Some("Prevent automatic display dim & lock"),
                Some("coffee"),
                true,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Duration: Indefinite",
                Some("Click to change timer preset"),
                Some("timer"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Keep Awake", "coffee", center_x, content, c)
    }
}
