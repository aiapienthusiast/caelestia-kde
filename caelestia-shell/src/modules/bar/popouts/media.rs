//! Media Popout
//! Parity with `shell/modules/bar/popouts/Media.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::components::controls::slider_row::SliderRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct MediaPopout;

impl MediaPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Starboy",
                Some("The Weeknd • Daft Punk"),
                Some("album"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(SliderRow::render(
                "Playback Progress",
                "1:45 / 3:50",
                0.45,
                200.0,
                Some("play_arrow"),
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Media Player", "music_note", center_x, content, c)
    }
}
