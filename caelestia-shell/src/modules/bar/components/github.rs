//! GithubActivity Widget
//! Parity with `shell/modules/bar/components/GithubActivity.qml` and `GithubStore.qml`.

use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GithubDayActivity {
    pub count: usize,
    pub intensity: u8, // 0 to 4
    pub is_error: bool,
}

impl Default for GithubDayActivity {
    fn default() -> Self {
        Self {
            count: 0,
            intensity: 0,
            is_error: false,
        }
    }
}

pub struct GithubActivityWidget;

impl GithubActivityWidget {
    /// Renders the 7-day contribution activity heat strip.
    pub fn render(unread_count: usize, c: &CaelestiaColours) -> Div {
        // Backwards-compatible fallback with unread_count mapping to last day's intensity
        let days = vec![
            GithubDayActivity { count: 0, intensity: 0, is_error: false },
            GithubDayActivity { count: 1, intensity: 1, is_error: false },
            GithubDayActivity { count: 2, intensity: 2, is_error: false },
            GithubDayActivity { count: 0, intensity: 0, is_error: false },
            GithubDayActivity { count: 3, intensity: 3, is_error: false },
            GithubDayActivity { count: 4, intensity: 4, is_error: false },
            GithubDayActivity {
                count: unread_count,
                intensity: if unread_count > 0 { 4 } else { 0 },
                is_error: false,
            },
        ];
        Self::render_full(&days, true, false, false, c)
    }

    /// Renders 7-day contribution cells matching GithubActivity.qml:110-138.
    pub fn render_full(
        days: &[GithubDayActivity],
        is_horizontal: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let cell_size = 12.0;
        let cell_spacing = SPACING_SMALL;

        let padding_h = PADDING_MEDIUM;
        let padding_v = PADDING_EXTRA_SMALL;

        let container = div()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .px(px(padding_h))
            .py(px(padding_v))
            .rounded_full()
            .bg(rgba(c.surface_container))
            .cursor(CursorStyle::PointingHand)
            .child(StateLayer::overlay(
                is_hovered,
                is_pressed,
                false,
                ROUNDING_FULL,
                c.secondary,
            ));

        let cells_layout = if is_horizontal {
            div().flex().flex_row().items_center().gap(px(cell_spacing))
        } else {
            div().flex().flex_col().items_center().gap(px(cell_spacing))
        };

        let day_slice: Vec<GithubDayActivity> = if days.is_empty() {
            (0..7).map(|_| GithubDayActivity::default()).collect()
        } else {
            days.to_vec()
        };

        let cells = cells_layout.children(day_slice.into_iter().map(|day| {
            let color = if day.is_error {
                c.error
            } else if day.intensity == 0 {
                c.surface_container_lowest
            } else {
                // Primary with intensity scale (0.25 to 1.0)
                let alpha = (day.intensity as f32) * 0.25;
                let r = ((c.primary >> 24) & 0xff) as f32 / 255.0;
                let g = ((c.primary >> 16) & 0xff) as f32 / 255.0;
                let b = ((c.primary >> 8) & 0xff) as f32 / 255.0;
                let u_r = (r * 255.0) as u32;
                let u_g = (g * 255.0) as u32;
                let u_b = (b * 255.0) as u32;
                let u_a = (alpha * 255.0) as u32;
                (u_r << 24) | (u_g << 16) | (u_b << 8) | u_a
            };

            div()
                .w(px(cell_size))
                .h(px(cell_size))
                .rounded(px(2.0))
                .bg(rgba(color))
        }));

        container.child(cells)
    }
}

