//! Bluetooth Popout
//! Parity with `shell/modules/bar/popouts/Bluetooth.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::components::controls::toggle_row::ToggleRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct BluetoothPopout;

impl BluetoothPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ToggleRow::render(
                "Bluetooth",
                Some("Visible as 'Caelestia-Desktop'"),
                Some("bluetooth"),
                true,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Sony WH-1000XM5",
                Some("Connected • Battery 80%"),
                Some("headphones"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "MX Master 3S",
                Some("Connected • Battery 95%"),
                Some("mouse"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Bluetooth Devices", "bluetooth", center_x, content, c)
    }
}
