//! ScreenCapture Widget
//! Parity with `shell/modules/bar/components/ScreenCaptureWidget.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct ScreenCaptureWidget;

impl ScreenCaptureWidget {
    /// Renders the screenshot / recording trigger button pill.
    pub fn render(is_recording: bool, c: &CaelestiaColours) -> Div {
        div()
            .w(px(28.0))
            .h(px(28.0))
            .rounded_full()
            .bg(if is_recording {
                rgba(c.error)
            } else {
                rgba(c.surface_container_low)
            })
            .text_color(if is_recording {
                rgba(c.on_error)
            } else {
                rgba(c.on_surface)
            })
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand)
            .child(StyledTextExt::icon_size(
                div().child(if is_recording { "stop_circle" } else { "screenshot_monitor" }),
                14.0,
            ))
    }
}
