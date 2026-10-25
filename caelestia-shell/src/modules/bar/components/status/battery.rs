//! BatteryStatus Indicator
//! Parity with `shell/modules/bar/components/status/BatteryStatus.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct BatteryStatusWidget;

impl BatteryStatusWidget {
    pub fn render(pct: u8, is_charging: bool, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(StyledTextExt::icon_size(
                div().child(if is_charging { "battery_charging_full" } else { "battery_full" }),
                16.0,
            ).text_color(rgba(c.on_surface)))
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface))
                    .child(format!("{}%", pct)),
            )
    }
}
