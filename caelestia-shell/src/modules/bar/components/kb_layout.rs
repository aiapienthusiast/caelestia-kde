//! KbLayoutIndicator Widget
//! Parity with `shell/modules/bar/components/KbLayoutIndicator.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, CursorStyle, Div, Styled};

pub struct KbLayoutIndicatorWidget;

impl KbLayoutIndicatorWidget {
    /// Renders active keyboard layout code pill (e.g. "US", "DE").
    pub fn render(layout_code: &str, c: &CaelestiaColours) -> Div {
        div()
            .px_2()
            .py_1()
            .rounded_full()
            .bg(rgba(c.surface_container_low))
            .text_color(rgba(c.on_surface))
            .cursor(CursorStyle::PointingHand)
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(layout_code.to_uppercase()),
            )
    }
}
