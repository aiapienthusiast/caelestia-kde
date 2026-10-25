//! Window Info Popup Dialog
//! Parity with `shell/modules/windowinfo/WindowInfo.qml`.

use super::buttons::WindowInfoButtons;
use super::details::WindowInfoDetails;
use super::preview::WindowInfoPreview;
use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::kwin::WindowItem;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct WindowInfoView;

impl WindowInfoView {
    pub fn render(window: Option<&WindowItem>, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_col()
            .w(px(520.0))
            .p_6()
            .rounded_xl()
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .shadow_lg()
            .gap_4()
            .child(
                // Header with title and close icon
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(rgba(c.on_surface))
                            .child("Window Properties"),
                    )
                    .child(
                        div()
                            .cursor(CursorStyle::PointingHand)
                            .child(StyledTextExt::icon_size(div().child("close"), 18.0).text_color(rgba(c.on_surface_variant))),
                    ),
            )
            .child(
                // Body: Preview on left, details on right
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .child(WindowInfoPreview::render(window, c))
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .justify_between()
                            .child(WindowInfoDetails::render(window, c))
                            .child(WindowInfoButtons::render(window, c)),
                    ),
            )
    }
}
