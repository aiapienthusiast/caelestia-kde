//! Window Preview Container
//! Parity with `shell/modules/windowinfo/Preview.qml`.

use crate::components::effects::ambient_glow::{AmbientGlowState, AmbientGlowView};
use crate::components::images::window_preview::{WindowPreviewView, WindowStreamState};
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::kwin::WindowItem;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct WindowInfoPreview;

impl WindowInfoPreview {
    pub fn render(window: Option<&WindowItem>, c: &CaelestiaColours) -> Div {
        let addr = window.map(|w| w.address.as_str()).unwrap_or("0x0");
        let icon = window.map(|w| w.app_id.as_str()).unwrap_or("window");

        let glow_state = AmbientGlowState::new()
            .with_address(addr)
            .with_fallback_color(rgba(c.primary));
        let stream_state = WindowStreamState::new(addr).with_fallback_icon(icon);

        div()
            .relative()
            .w(px(260.0))
            .h(px(160.0))
            .rounded_lg()
            .overflow_hidden()
            .bg(rgba(c.surface_container_highest))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(AmbientGlowView::new(glow_state)),
            )
            .child(
                div()
                    .relative()
                    .size_full()
                    .child(WindowPreviewView::new(stream_state)),
            )
    }
}
