//! Github Popout
//! Parity with `shell/modules/bar/popouts/Github.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct GithubPopout;

impl GithubPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "PR #42: Rust GPUI Shell Port",
                Some("ladybug-me • 2 hours ago"),
                Some("commit"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(ListRow::render(
                "Issue #19: Audio visualizer bars",
                Some("Mentioned by user • Today"),
                Some("chat_bubble_outline"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("GitHub Notifications", "notifications", center_x, content, c)
    }
}
