//! BatteryStatus Indicator
//! Parity with `shell/modules/bar/components/status/BatteryStatus.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct BatteryStatusWidget;

impl BatteryStatusWidget {
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

        let icon_color = if pct > 20 || is_charging {
            c.on_surface
        } else {
            c.error
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(MaterialIcon::with_size(icon_name, IconSize::Small, icon_color))
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(icon_color))
                    .child(format!("{}%", pct)),
            )
    }
}
