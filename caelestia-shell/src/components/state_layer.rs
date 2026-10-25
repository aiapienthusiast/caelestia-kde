//! StateLayer
//! Parity with `shell/components/StateLayer.qml`.
//! Provides Material 3 interactive state feedback (8% hover, 12% pressed overlay).

use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct StateLayer;

impl StateLayer {
    /// Overlays an interactive state layer with M3 8% hover and 12% pressed feedback.
    pub fn overlay(
        is_hovered: bool,
        is_pressed: bool,
        disabled: bool,
        radius_px: f32,
        on_color: u32,
    ) -> Div {
        let opacity: f32 = if disabled {
            0.0
        } else if is_pressed {
            0.12
        } else if is_hovered {
            0.08
        } else {
            0.0
        };

        // Extract RGB components from on_color to blend at M3 state opacity
        let r = (on_color >> 24) & 0xff;
        let g = (on_color >> 16) & 0xff;
        let b = (on_color >> 8) & 0xff;
        let alpha_byte = ((opacity * 255.0).round() as u32) & 0xff;
        let overlay_color = (r << 24) | (g << 16) | (b << 8) | alpha_byte;

        div()
            .absolute()
            .inset_0()
            .rounded(px(radius_px))
            .bg(rgba(overlay_color))
            .when(!disabled, |el| el.cursor(CursorStyle::PointingHand))
    }
}
