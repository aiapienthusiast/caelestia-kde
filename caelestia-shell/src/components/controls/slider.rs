//! StyledSlider & FilledSlider
//! Parity with `shell/components/controls/StyledSlider.qml` and `FilledSlider.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct StyledSlider;

impl StyledSlider {
    /// Renders an M3 Slider with filled track, remaining track, and handle.
    pub fn render(
        value: f32, // 0.0 to 1.0
        width_px: f32,
        is_hovered: bool,
        is_dragging: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let clamped_val = value.clamp(0.0, 1.0);
        let track_height = SLIDER_HEIGHT;
        let handle_width = if is_dragging { 6.0 } else if is_hovered { 5.0 } else { 4.0 };
        let handle_height = if is_dragging { track_height + 8.0 } else { track_height + 4.0 };

        let fg_color = if is_disabled {
            c.disabled_content
        } else {
            c.primary
        };

        let bg_color = if is_disabled {
            c.disabled_container
        } else {
            c.secondary_container
        };

        let filled_width = clamped_val * width_px;
        let handle_pos = (filled_width - handle_width / 2.0).clamp(0.0, width_px - handle_width);

        let mut container = div()
            .relative()
            .w(px(width_px))
            .h(px(handle_height))
            .flex()
            .items_center();

        if !is_disabled {
            container = container.cursor(CursorStyle::PointingHand);
        }

        // Background track (remaining)
        let bg_track = div()
            .absolute()
            .left_0()
            .right_0()
            .h(px(track_height))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(bg_color));

        // Active track (filled)
        let active_track = div()
            .absolute()
            .left_0()
            .w(px(filled_width))
            .h(px(track_height))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(fg_color));

        // Handle pill
        let handle = div()
            .absolute()
            .left(px(handle_pos))
            .w(px(handle_width))
            .h(px(handle_height))
            .rounded(px(ROUNDING_FULL))
            .bg(rgba(fg_color));

        container
            .child(bg_track)
            .child(active_track)
            .child(handle)
    }
}
