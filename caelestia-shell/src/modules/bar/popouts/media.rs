//! Media Popout
//! Parity with `shell/modules/bar/popouts/Media.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::icon_button::IconButton;
use crate::components::controls::slider::StyledSlider;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug)]
pub struct MediaPopoutState {
    pub player_name: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub is_playing: bool,
    pub position_secs: u32,
    pub duration_secs: u32,
    pub volume: f32,
}

impl Default for MediaPopoutState {
    fn default() -> Self {
        Self {
            player_name: "Spotify".into(),
            title: "Starboy".into(),
            artist: "The Weeknd, Daft Punk".into(),
            album: "Starboy".into(),
            is_playing: true,
            position_secs: 105,
            duration_secs: 230,
            volume: 0.85,
        }
    }
}

pub struct MediaPopout;

impl MediaPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &MediaPopoutState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &MediaPopoutState,
        c: &CaelestiaColours,
    ) -> Div {
        let progress = if state.duration_secs > 0 {
            (state.position_secs as f32 / state.duration_secs as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let pos_str = format!(
            "{}:{:02} / {}:{:02}",
            state.position_secs / 60,
            state.position_secs % 60,
            state.duration_secs / 60,
            state.duration_secs % 60
        );

        let content = div()
            .flex()
            .flex_col()
            .w(px(276.0))
            .p(px(PADDING_MEDIUM))
            .rounded(px(ROUNDING_LARGE))
            .bg(rgba(c.surface_container))
            .gap(px(SPACING_SMALL))
            // Source Selector Line
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        StyledText::new(state.player_name.clone())
                            .text_size(px(FONT_SIZE_LABEL_MEDIUM))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(c.on_surface_variant)),
                    )
                    .child(
                        MaterialIcon::new("expand_more")
                            .text_size(px(ICON_SIZE_SMALL))
                            .text_color(rgba(c.on_surface_variant)),
                    ),
            )
            // Album Art & Metadata Hero
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(SPACING_MEDIUM))
                    .child(
                        div()
                            .size(px(64.0))
                            .rounded(px(ROUNDING_MEDIUM))
                            .bg(rgba(c.primary_container))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                MaterialIcon::new("music_note")
                                    .text_size(px(32.0))
                                    .text_color(rgba(c.on_primary_container)),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                StyledText::new(state.title.clone())
                                    .text_size(px(FONT_SIZE_BODY_LARGE))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgba(c.on_surface)),
                            )
                            .child(
                                StyledText::new(state.artist.clone())
                                    .text_size(px(FONT_SIZE_BODY_SMALL))
                                    .text_color(rgba(c.on_surface_variant)),
                            ),
                    ),
            )
            // Progress Slider & Timestamps
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        StyledSlider::render(progress, 340.0, false, false, false, c),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_end()
                            .child(
                                StyledText::new(pos_str)
                                    .text_size(px(FONT_SIZE_LABEL_SMALL))
                                    .text_color(rgba(c.outline)),
                            ),
                    ),
            )
            // Playback Controls Row
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .gap(px(SPACING_SMALL))
                    .child(
                        IconButton::new("shuffle")
                            .with_colours(c.clone()),
                    )
                    .child(
                        IconButton::new("skip_previous")
                            .with_colours(c.clone()),
                    )
                    .child(
                        div()
                            .size(px(44.0))
                            .rounded_full()
                            .bg(rgba(c.primary))
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.9))
                            .child(
                                MaterialIcon::new(if state.is_playing { "pause" } else { "play_arrow" })
                                    .text_size(px(ICON_SIZE_MEDIUM))
                                    .text_color(rgba(c.on_primary)),
                            ),
                    )
                    .child(
                        IconButton::new("skip_next")
                            .with_colours(c.clone()),
                    )
                    .child(
                        IconButton::new("repeat")
                            .with_colours(c.clone()),
                    ),
            );

        PopoutWrapper::render("Media", "music_note", center_x, content, c)
    }
}
