//! Active Workspace Indicator
//! Parity with `shell/modules/bar/components/workspaces/ActiveIndicator.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct ActiveIndicatorWidget;

impl ActiveIndicatorWidget {
    pub fn render(
        _active_ws_id: u32,
        current_offset: f32,
        is_horizontal: bool,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .absolute()
            .rounded_full()
            .bg(rgba(c.primary))
            .when(is_horizontal, |d| {
                d.h(px(4.0))
                    .w(px(24.0))
                    .bottom(px(0.0))
                    .left(px(current_offset))
            })
            .when(!is_horizontal, |d| {
                d.w(px(4.0))
                    .h(px(24.0))
                    .right(px(0.0))
                    .top(px(current_offset))
            })
    }
}
