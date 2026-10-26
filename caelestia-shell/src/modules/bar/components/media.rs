//! Media Widget
//! Parity with `shell/modules/bar/components/Media.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::components::styled_text::StyledText;
use crate::components::widgets::visualizer_bars::VisualizerBars;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct MediaWidget;

impl MediaWidget {
    /// Renders the compact media pill with audio visualizer spectrum bars.
    pub fn render(
        title: &str,
        is_playing: bool,
        anim_phase: f32,
        c: &CaelestiaColours,
    ) -> Div {
        Self::render_full(title, is_playing, anim_phase, true, true, true, false, false, c)
    }

    /// Renders with full layout, title, visualizer, and hover/pressed states matching Media.qml.
    pub fn render_full(
        title: &str,
        is_playing: bool,
        anim_phase: f32,
        show_title: bool,
        show_visualiser: bool,
        _is_horizontal: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let max_len = 28;
        let truncated_title = if title.len() > max_len {
            format!("{}…", &title[..max_len])
        } else if title.is_empty() {
            "Nothing playing".to_string()
        } else {
            title.to_string()
        };

        let show_text = show_title && is_playing && !title.is_empty();
        let show_bars = show_visualiser && is_playing;
        let has_content = show_text || show_bars;

        let padding_h = if has_content && show_text { PADDING_MEDIUM } else { PADDING_SMALL };
        let padding_v = PADDING_EXTRA_SMALL;

        let mut container = div()
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .rounded_full()
            .bg(rgba(c.surface_container))
            .cursor(CursorStyle::PointingHand)
            .child(StateLayer::overlay(
                is_hovered,
                is_pressed,
                false,
                ROUNDING_FULL,
                c.primary,
            ));

        if !has_content {
            // Mode icon fallback when paused / stopped / no track: graphic_eq (Media.qml:147-155)
            container = container
                .w(px(32.0))
                .h(px(32.0))
                .child(
                    MaterialIcon::with_size("graphic_eq", IconSize::Medium, c.on_surface),
                );
        } else {
            let mut content = div()
                .flex()
                .flex_row()
                .items_center()
                .px(px(padding_h))
                .py(px(padding_v))
                .gap(px(SPACING_SMALL));

            if show_text {
                content = content.child(
                    StyledText::new(truncated_title)
                        .text_size(px(FONT_SIZE_BODY_MEDIUM))
                        .text_color(rgba(c.on_surface)),
                );
            }

            if show_bars {
                content = content.child(VisualizerBars::render(
                    5, // 5 bars in Media.qml:113
                    anim_phase,
                    is_playing,
                    c.primary,
                    14.0,
                ));
            }

            container = container.child(content);
        }

        container
    }
}

