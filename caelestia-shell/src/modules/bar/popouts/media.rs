//! Media Popout
//! Parity with `shell/modules/bar/popouts/Media.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::components::controls::slider_row::SliderRow;
use crate::components::widgets::cover_art::{CoverArtState, CoverArtView};
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct MediaPopout;

impl MediaPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let cover_state = CoverArtState::new()
            .with_art("https://example.com/starboy.jpg")
            .with_playing(true)
            .with_size(96.0);

        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(SPACING_MEDIUM))
                    .child(CoverArtView::new(cover_state))
                    .child(
                        div()
                            .flex_1()
                            .child(ListRow::render(
                                "Starboy",
                                Some("The Weeknd • Daft Punk"),
                                Some("album"),
                                None,
                                false,
                                false,
                                false,
                                c,
                            )),
                    ),
            )
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
