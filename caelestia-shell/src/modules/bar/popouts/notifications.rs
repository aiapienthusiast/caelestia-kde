//! Notifications Popout
//! Parity with `shell/modules/bar/popouts/Notifications.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::components::controls::toggle_row::ToggleRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct NotificationsPopout;

impl NotificationsPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ToggleRow::render(
                "Do Not Disturb",
                Some("Silence banner alerts"),
                Some("notifications_off"),
                false,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Terminal Build Succeeded",
                Some("Cargo finished dev target in 2.6s"),
                Some("check_circle"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Clear All Notifications",
                None::<&str>,
                Some("clear_all"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Notification Center", "notifications_none", center_x, content, c)
    }
}
