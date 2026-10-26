//! PerfNetwork Meter
//! Parity with `shell/modules/bar/components/performance/PerfNetwork.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct PerfNetwork;

impl PerfNetwork {
    pub fn render(down_kb: u64, up_kb: u64, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(MaterialIcon::with_size("swap_vert", IconSize::Small, c.secondary))
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface))
                    .child(format!("↓{} ↑{}K", down_kb, up_kb)),
            )
    }
}
