//! PerfBattery Meter
//! Parity with `shell/modules/bar/components/performance/PerfBattery.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct PerfBattery;

impl PerfBattery {
    pub fn render(pct: u8, is_charging: bool, c: &CaelestiaColours) -> Div {
        let icon_name = if is_charging {
            "battery_charging_full"
        } else if pct > 80 {
            "battery_full"
        } else if pct > 50 {
            "battery_5_bar"
        } else if pct > 20 {
            "battery_3_bar"
        } else {
            "battery_alert"
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(MaterialIcon::with_size(icon_name, IconSize::Small, c.primary))
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface))
                    .child(format!("{}%", pct)),
            )
    }
}
