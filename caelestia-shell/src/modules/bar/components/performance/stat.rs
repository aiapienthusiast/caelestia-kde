//! PerfStat Meter
//! Parity with `shell/modules/bar/components/performance/PerfStat.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct PerfStat;

impl PerfStat {
    pub fn render(label: &str, value: &str, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgba(c.on_surface_variant))
                    .child(label.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface))
                    .child(value.to_string()),
            )
    }
}
