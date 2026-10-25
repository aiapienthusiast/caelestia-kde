//! PeripheralBattery Popout
//! Parity with `shell/modules/bar/popouts/PeripheralBattery.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct PeripheralBatteryPopout;

impl PeripheralBatteryPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Wireless Mouse",
                Some("85% Remaining"),
                Some("mouse"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Mechanical Keyboard",
                Some("92% Remaining"),
                Some("keyboard"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Connected Peripherals", "devices", center_x, content, c)
    }
}
