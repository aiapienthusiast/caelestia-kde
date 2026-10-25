//! Greeter Popout
//! Parity with `shell/modules/bar/popouts/Greeter.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct GreeterPopout;

impl GreeterPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "ladybug-me",
                Some("Arch Linux • Plasma 6 Wayland"),
                Some("account_circle"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Lock Screen",
                Some("Lock session now"),
                Some("lock"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Log Out",
                Some("End user session"),
                Some("logout"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("User Profile", "account_circle", center_x, content, c)
    }
}
