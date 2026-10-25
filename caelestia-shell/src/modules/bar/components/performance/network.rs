//! PerfNetwork Meter
//! Parity with `shell/modules/bar/components/performance/PerfNetwork.qml`.

use crate::components::styled_text::StyledTextExt;
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
            .child(StyledTextExt::icon_size(div().child("swap_vert"), 12.0).text_color(rgba(c.secondary)))
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface))
                    .child(format!("↓{} ↑{}K", down_kb, up_kb)),
            )
    }
}
