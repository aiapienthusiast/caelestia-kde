//! ScreenCapture Popout
//! Parity with `shell/modules/bar/popouts/ScreenCapture.qml` and `CaptureCard.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::icon_button::IconButton;
use crate::components::controls::list_row::ListRow;
use crate::components::controls::split_button::{SplitButton, SplitButtonItem, SplitButtonType};
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CaptureMode {
    #[default]
    Shot,
    Record,
}

#[derive(Clone, Debug)]
pub struct ScreenCaptureState {
    pub mode: CaptureMode,
    pub is_recording: bool,
    pub is_paused: bool,
    pub elapsed_seconds: u64,
    pub recent_captures: Vec<String>,
}

impl Default for ScreenCaptureState {
    fn default() -> Self {
        Self {
            mode: CaptureMode::Shot,
            is_recording: false,
            is_paused: false,
            elapsed_seconds: 0,
            recent_captures: Vec::new(),
        }
    }
}

pub struct ScreenCapturePopout;

impl ScreenCapturePopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &ScreenCaptureState::default(), c)
    }

    pub fn render_with_state(center_x: f32, state: &ScreenCaptureState, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .w(px(360.0))
            .gap(px(SPACING_MEDIUM))
            // Header card
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p(px(PADDING_MEDIUM))
                    .rounded(px(ROUNDING_LARGE))
                    .bg(rgba(c.surface_container))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(SPACING_MEDIUM))
                            .child(
                                div()
                                    .size(px(40.0))
                                    .rounded_full()
                                    .bg(if state.is_recording {
                                        rgba(c.secondary)
                                    } else {
                                        rgba(c.secondary_container)
                                    })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        MaterialIcon::new(if state.mode == CaptureMode::Shot && !state.is_recording {
                                            "screenshot_region"
                                        } else {
                                            "screen_record"
                                        })
                                        .text_size(px(ICON_SIZE_MEDIUM))
                                        .text_color(if state.is_recording {
                                            rgba(c.on_secondary)
                                        } else {
                                            rgba(c.on_secondary_container)
                                        }),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        StyledText::new("Screen Capture")
                                            .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgba(c.on_surface)),
                                    )
                                    .child(
                                        StyledText::new(if state.mode == CaptureMode::Shot {
                                            "Capture, OCR and search"
                                        } else if state.is_paused {
                                            "Paused"
                                        } else if state.is_recording {
                                            "Recording..."
                                        } else {
                                            "Ready to record"
                                        })
                                        .text_size(px(FONT_SIZE_BODY_SMALL))
                                        .text_color(rgba(c.on_surface_variant)),
                                    ),
                            ),
                    )
                    .child(
                        if state.mode == CaptureMode::Shot {
                            SplitButton::new("screenshot_region", "Capture")
                                .button_type(SplitButtonType::Filled)
                                .disabled(state.is_recording)
                                .active_item(Some(SplitButtonItem {
                                    id: "region".into(),
                                    icon: Some("screenshot_region".into()),
                                    text: "Region".into(),
                                }))
                                .with_colours(c.clone())
                                .into_element()
                        } else {
                            SplitButton::new("fullscreen", "Start")
                                .button_type(SplitButtonType::Filled)
                                .disabled(state.is_recording)
                                .active_item(Some(SplitButtonItem {
                                    id: "fullscreen".into(),
                                    icon: Some("fullscreen".into()),
                                    text: "Record".into(),
                                }))
                                .with_colours(c.clone())
                                .into_element()
                        },
                    ),
            )
            // Mode Selector Row (Record / Capture)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(SPACING_SMALL))
                    .child(
                        div()
                            .id("btn_mode_record")
                            .flex_1()
                            .h(px(36.0))
                            .rounded(px(ROUNDING_FULL))
                            .bg(if state.mode == CaptureMode::Record {
                                rgba(c.secondary_container)
                            } else {
                                rgba(c.surface_container_high)
                            })
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .child(
                                StyledText::new("Record")
                                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if state.mode == CaptureMode::Record {
                                        rgba(c.on_secondary_container)
                                    } else {
                                        rgba(c.on_surface)
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .id("btn_mode_capture")
                            .flex_1()
                            .h(px(36.0))
                            .rounded(px(ROUNDING_FULL))
                            .bg(if state.mode == CaptureMode::Shot {
                                rgba(c.secondary_container)
                            } else {
                                rgba(c.surface_container_high)
                            })
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .child(
                                StyledText::new("Capture")
                                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if state.mode == CaptureMode::Shot {
                                        rgba(c.on_secondary_container)
                                    } else {
                                        rgba(c.on_surface)
                                    }),
                            ),
                    ),
            )
            // Active Recording Controls Banner or Action Options
            .child(
                if state.is_recording {
                    let mins = state.elapsed_seconds / 60;
                    let secs = state.elapsed_seconds % 60;
                    let duration_str = format!("Recording {:02}:{:02}", mins, secs);

                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .p(px(PADDING_MEDIUM))
                        .rounded(px(ROUNDING_MEDIUM))
                        .bg(rgba(c.surface_container_high))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(SPACING_SMALL))
                                .child(
                                    div()
                                        .px(px(PADDING_SMALL))
                                        .py(px(2.0))
                                        .rounded_full()
                                        .bg(if state.is_paused {
                                            rgba(c.secondary)
                                        } else {
                                            rgba(c.error)
                                        })
                                        .child(
                                            StyledText::new(if state.is_paused { "PAUSED" } else { "REC" })
                                                .text_size(px(FONT_SIZE_LABEL_SMALL))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(if state.is_paused {
                                                    rgba(c.on_secondary)
                                                } else {
                                                    rgba(c.on_error)
                                                }),
                                        ),
                                )
                                .child(
                                    StyledText::new(duration_str)
                                        .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                        .text_color(rgba(c.on_surface)),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(SPACING_EXTRA_SMALL))
                                .child(
                                    IconButton::new(if state.is_paused { "play_arrow" } else { "pause" })
                                        .with_colours(c.clone())
                                        .into_element(),
                                )
                                .child(
                                    IconButton::new("stop")
                                        .with_colours(c.clone())
                                        .into_element(),
                                ),
                        )
                } else if state.mode == CaptureMode::Shot {
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(SPACING_SMALL))
                        .child(ListRow::render(
                            "Capture Region",
                            Some("Select custom area or window"),
                            Some("crop"),
                            None,
                            false,
                            false,
                            false,
                            c,
                        ))
                        .child(ListRow::render(
                            "Capture Fullscreen",
                            Some("Save all active monitors"),
                            Some("fullscreen"),
                            None,
                            false,
                            false,
                            false,
                            c,
                        ))
                        .child(ListRow::render(
                            "Recognize Text (OCR)",
                            Some("Extract text directly from screen"),
                            Some("text_fields"),
                            None,
                            false,
                            false,
                            false,
                            c,
                        ))
                        .child(ListRow::render(
                            "Search Image",
                            Some("Visual search on selected snip"),
                            Some("image_search"),
                            None,
                            false,
                            false,
                            false,
                            c,
                        ))
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(SPACING_SMALL))
                        .child(ListRow::render(
                            "Record Screen & Audio",
                            Some("Default desktop audio output"),
                            Some("volume_up"),
                            None,
                            false,
                            false,
                            false,
                            c,
                        ))
                        .child(ListRow::render(
                            "Record Microphone",
                            Some("Include voice voiceover"),
                            Some("mic"),
                            None,
                            false,
                            false,
                            false,
                            c,
                        ))
                        .child(ListRow::render(
                            "Record GIF Animation",
                            Some("Lightweight animated capture"),
                            Some("animated_images"),
                            None,
                            false,
                            false,
                            false,
                            c,
                        ))
                },
            );

        PopoutWrapper::render("Screen Capture", "screenshot_monitor", center_x, content, c)
    }
}
