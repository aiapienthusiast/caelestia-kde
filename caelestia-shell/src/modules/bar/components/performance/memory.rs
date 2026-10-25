//! PerfMemory Meter
//! Parity with `shell/modules/bar/components/performance/PerfMemory.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct PerfMemory;

impl PerfMemory {
    pub fn render(used_gb: f32, total_gb: f32, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(StyledTextExt::icon_size(div().child("storage"), 12.0).text_color(rgba(c.secondary)))
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface))
                    .child(format!("{:.1}/{:.0}G", used_gb, total_gb)),
            )
    }
}
