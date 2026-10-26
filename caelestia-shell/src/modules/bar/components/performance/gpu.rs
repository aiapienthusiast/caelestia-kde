//! PerfGpu Meter
//! Parity with `shell/modules/bar/components/performance/PerfGpu.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct PerfGpu;

impl PerfGpu {
    pub fn render(usage_pct: u8, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(MaterialIcon::with_size("developer_board", IconSize::Small, c.secondary))
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface))
                    .child(format!("{}%", usage_pct)),
            )
    }
}
