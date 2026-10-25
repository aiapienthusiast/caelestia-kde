//! Performance Widget
//! Parity with `shell/modules/bar/components/PerformanceWidget.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, CursorStyle, Div, Styled};

pub struct PerformanceWidget;

impl PerformanceWidget {
    /// Renders the compact performance monitor pill.
    pub fn render(cpu_pct: u8, ram_pct: u8, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .rounded_full()
            .bg(rgba(c.surface_container_low))
            .cursor(CursorStyle::PointingHand)
            .child(
                div()
                    .text_color(rgba(c.primary))
                    .child(StyledTextExt::icon_size(div().child("speed"), 14.0)),
            )
            .child(
                div()
                    .text_color(rgba(c.on_surface))
                    .text_xs()
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(format!("CPU {}% • RAM {}%", cpu_pct, ram_pct)),
            )
    }
}
