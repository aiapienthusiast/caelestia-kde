//! ScreenCapture Popout
//! Parity with `shell/modules/bar/popouts/ScreenCapture.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct ScreenCapturePopout;

impl ScreenCapturePopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Capture Fullscreen",
                Some("Save all active monitors"),
                Some("fullscreen"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Capture Active Window",
                Some("Snip focused application"),
                Some("crop_square"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Capture Region",
                Some("Select custom bounding area"),
                Some("crop"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Screen Capture", "screenshot_monitor", center_x, content, c)
    }
}
