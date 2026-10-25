//! Window Information Action Buttons
//! Parity with `shell/modules/windowinfo/Buttons.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::kwin::WindowItem;
use gpui::{div, prelude::*, rgba, CursorStyle, Div, Styled};

pub struct WindowInfoButtons;

impl WindowInfoButtons {
    pub fn render(window: Option<&WindowItem>, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(rgba(c.on_surface_variant))
                    .child("Actions"),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(rgba(c.surface_container_high))
                            .cursor(CursorStyle::PointingHand)
                            .child(StyledTextExt::icon_size(div().child("close"), 14.0).text_color(rgba(c.error)))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgba(c.on_surface))
                                    .child("Close Window"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(rgba(c.surface_container_high))
                            .cursor(CursorStyle::PointingHand)
                            .child(StyledTextExt::icon_size(div().child("fullscreen"), 14.0).text_color(rgba(c.primary)))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgba(c.on_surface))
                                    .child(if window.map(|w| w.is_fullscreen).unwrap_or(false) {
                                        "Exit Fullscreen"
                                    } else {
                                        "Fullscreen"
                                    }),
                            ),
                    ),
            )
    }
}
