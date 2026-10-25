//! PerfStorage Meter
//! Parity with `shell/modules/bar/components/performance/PerfStorage.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct PerfStorage;

impl PerfStorage {
    pub fn render(free_gb: u64, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(StyledTextExt::icon_size(div().child("hard_drive"), 12.0).text_color(rgba(c.secondary)))
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface))
                    .child(format!("{}G free", free_gb)),
            )
    }
}
