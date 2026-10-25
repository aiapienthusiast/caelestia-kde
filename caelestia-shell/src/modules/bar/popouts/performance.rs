//! Performance Popout
//! Parity with `shell/modules/bar/popouts/Performance.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::slider_row::SliderRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct PerformancePopout;

impl PerformancePopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(SliderRow::render(
                "CPU Utilization",
                "12%",
                0.12,
                200.0,
                Some("memory"),
                false,
                false,
                false,
                c,
            ))
            .child(SliderRow::render(
                "RAM Used",
                "6.4 / 31.2 GB",
                0.21,
                200.0,
                Some("storage"),
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("System Performance", "speed", center_x, content, c)
    }
}
