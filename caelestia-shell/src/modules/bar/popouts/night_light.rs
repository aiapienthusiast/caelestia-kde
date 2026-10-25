//! NightLight Popout
//! Parity with `shell/modules/bar/popouts/NightLight.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::slider_row::SliderRow;
use crate::components::controls::toggle_row::ToggleRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct NightLightPopout;

impl NightLightPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ToggleRow::render(
                "Night Light",
                Some("Reduce blue light during evening"),
                Some("nights_stay"),
                true,
                false,
                false,
                false,
                c,
            ))
            .child(SliderRow::render(
                "Color Temperature",
                "4500K",
                0.60,
                200.0,
                Some("thermostat"),
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Night Light", "nights_stay", center_x, content, c)
    }
}
