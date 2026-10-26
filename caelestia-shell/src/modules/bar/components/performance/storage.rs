//! PerfStorage Meter
//! Parity with `shell/modules/bar/components/performance/PerfStorage.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
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
            .child(MaterialIcon::with_size("hard_drive", IconSize::Small, c.secondary))
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface))
                    .child(format!("{}G free", free_gb)),
            )
    }
}
