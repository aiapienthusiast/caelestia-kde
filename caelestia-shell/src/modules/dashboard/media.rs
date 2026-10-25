//! Media Player Tab
//! Parity with `shell/modules/dashboard/Media.qml` and `CoverVisualiser.qml`.
//! Displays track metadata, live visualizer bars, and media controls.

use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::icon_button::IconButton;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::{StyledText, StyledTextExt};
use crate::components::widgets::visualizer_bars::VisualizerBars;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::mpris::{MprisService, PlaybackStatus};
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, MouseButton, Styled};

pub struct MediaView;

impl MediaView {
    pub fn render<V: 'static>(
        mpris: &MprisService,
        anim_phase: f32,
        colours: &CaelestiaColours,
        _cx: &mut gpui::Context<V>,
    ) -> Div {
        let is_playing = mpris.current_track.status == PlaybackStatus::Playing;
        let play_icon = if is_playing { "pause" } else { "play_arrow" };

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(20.0))
            .w_full()
            .h_full()
            .p(px(24.0))
            .child(
                // Media Cover & Visualizer Card
                div()
                    .w(px(520.0))
                    .p(px(24.0))
                    .rounded(px(24.0))
                    .bg(rgba(colours.surface_container))
                    .border_1()
                    .border_color(rgba(colours.outline_variant))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(16.0))
                    .child(
                        // Music Icon or Album Art Box
                        div()
                            .w(px(120.0))
                            .h(px(120.0))
                            .rounded(px(20.0))
                            .bg(rgba(colours.primary_container))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                MaterialIcon::new("music_note")
                                    .color(colours.on_primary_container)
                                    .icon_size(56.0),
                            ),
                    )
                    .child(
                        // Track title & artist
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                StyledText::new(&mpris.current_track.title)
                                    .color(colours.on_surface)
                                    .font_size(20.0)
                                    .font_weight(gpui::FontWeight::BOLD),
                            )
                            .child(
                                StyledText::new(&mpris.current_track.artist)
                                    .color(colours.on_surface_variant)
                                    .font_size(14.0),
                            ),
                    )
                    .child(
                        // Visualizer Spectrum Bars
                        div()
                            .py(px(8.0))
                            .child(VisualizerBars::render(
                                24,
                                anim_phase,
                                is_playing,
                                colours.primary,
                                36.0,
                            )),
                    )
                    .child(
                        // Playback Controls Row
                        div()
                            .flex()
                            .items_center()
                            .gap(px(16.0))
                            .child(
                                IconButton::render(
                                    "skip_previous",
                                    &ButtonStyle {
                                        button_type: ButtonType::Tonal,
                                        is_round: true,
                                        ..Default::default()
                                    },
                                    colours,
                                    Some(44.0),
                                )
                                .cursor(CursorStyle::PointingHand)
                                .on_mouse_down(MouseButton::Left, |_, _, _| {
                                    MprisService::previous();
                                }),
                            )
                            .child(
                                IconButton::render(
                                    play_icon,
                                    &ButtonStyle {
                                        button_type: ButtonType::Filled,
                                        is_round: true,
                                        ..Default::default()
                                    },
                                    colours,
                                    Some(56.0),
                                )
                                .cursor(CursorStyle::PointingHand)
                                .on_mouse_down(MouseButton::Left, |_, _, _| {
                                    MprisService::play_pause();
                                }),
                            )
                            .child(
                                IconButton::render(
                                    "skip_next",
                                    &ButtonStyle {
                                        button_type: ButtonType::Tonal,
                                        is_round: true,
                                        ..Default::default()
                                    },
                                    colours,
                                    Some(44.0),
                                )
                                .cursor(CursorStyle::PointingHand)
                                .on_mouse_down(MouseButton::Left, |_, _, _| {
                                    MprisService::next();
                                }),
                            ),
                    ),
            )
    }
}
