//! Audio Popout
//! Parity with `shell/modules/bar/popouts/Audio.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::components::controls::slider_row::SliderRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct AudioPopout;

impl AudioPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Built-in Audio Analog Stereo",
                Some("Speakers (Active)"),
                Some("speaker"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(SliderRow::render(
                "Output Volume",
                "75%",
                0.75,
                200.0,
                Some("volume_up"),
                false,
                false,
                false,
                c,
            ))
            .child(SliderRow::render(
                "Microphone Level",
                "60%",
                0.60,
                200.0,
                Some("mic"),
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Audio Settings", "volume_up", center_x, content, c)
    }
}
