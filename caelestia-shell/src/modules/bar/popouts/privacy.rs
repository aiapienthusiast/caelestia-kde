//! Privacy Popout
//! Parity with `shell/modules/bar/popouts/Privacy.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct PrivacyPopout;

impl PrivacyPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "Microphone: Idle",
                Some("No apps recording audio"),
                Some("mic_none"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Camera: Off",
                Some("No apps using webcam"),
                Some("videocam_off"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Privacy & Sensors", "security", center_x, content, c)
    }
}
