//! Updates Popout
//! Parity with `shell/modules/bar/popouts/Updates.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct UpdatesPopout;

impl UpdatesPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "System Up to Date",
                Some("Last checked 30 minutes ago"),
                Some("verified"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Check for Updates",
                Some("Query pacman & flatpak repositories"),
                Some("refresh"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("System Updates", "system_update", center_x, content, c)
    }
}
