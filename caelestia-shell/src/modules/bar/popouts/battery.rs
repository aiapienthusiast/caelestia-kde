//! Battery Popout
//! Parity with `shell/modules/bar/popouts/Battery.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct BatteryPopout;

impl BatteryPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "85% • Discharging",
                Some("Approx. 4 hours 15 mins remaining"),
                Some("battery_6_bar"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Power Draw",
                Some("14.2 W (Battery Health: 96%)"),
                Some("electric_bolt"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Power & Battery", "battery_full", center_x, content, c)
    }
}
