//! Power Popout
//! Parity with `shell/modules/bar/popouts/Power.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct PowerPopout;

impl PowerPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Lock Screen",
                Some("Lock current session"),
                Some("lock"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Suspend",
                Some("Sleep to RAM"),
                Some("bedtime"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Restart System",
                Some("Reboot workstation"),
                Some("restart_alt"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Shut Down",
                Some("Power off machine safely"),
                Some("power_settings_new"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Session & Power", "power_settings_new", center_x, content, c)
    }
}
