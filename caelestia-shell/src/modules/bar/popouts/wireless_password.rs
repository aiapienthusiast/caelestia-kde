//! WirelessPassword Popout
//! Parity with `shell/modules/bar/popouts/WirelessPassword.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct WirelessPasswordPopout;

impl WirelessPasswordPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Network: Home-5G",
                Some("Enter WPA2/WPA3 Personal Passphrase"),
                Some("wifi_password"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Connect",
                Some("Attempt wireless authentication"),
                Some("check"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Connect to Wi-Fi", "wifi_password", center_x, content, c)
    }
}
