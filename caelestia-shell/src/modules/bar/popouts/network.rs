//! Network Popout
//! Parity with `shell/modules/bar/popouts/Network.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::components::controls::toggle_row::ToggleRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct NetworkPopout;

impl NetworkPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ToggleRow::render(
                "Wi-Fi",
                Some("Connected to Home-5G"),
                Some("wifi"),
                true,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Home-5G",
                Some("Connected • Excellent Signal"),
                Some("wifi_lock"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Guest-Network",
                Some("Secured with WPA3"),
                Some("wifi_lock"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Wi-Fi & Networks", "wifi", center_x, content, c)
    }
}
