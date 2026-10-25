//! What's New Window View
//! Parity with `shell/modules/whatsnew/WhatsNewWindow.qml`.

use super::entries::WhatsNewEntry;
use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct WhatsNewView;

impl WhatsNewView {
    pub fn render(entries: &[WhatsNewEntry], current_index: usize, c: &CaelestiaColours) -> Div {
        let entry = entries.get(current_index);

        div()
            .flex()
            .flex_col()
            .w(px(600.0))
            .h(px(450.0))
            .p_6()
            .rounded_xl()
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .shadow_lg()
            .child(
                // Header
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .pb_4()
                    .border_b_1()
                    .border_color(rgba(c.outline_variant))
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(rgba(c.on_surface))
                            .child("What's New in Caelestia"),
                    )
                    .child(
                        div()
                            .cursor(CursorStyle::PointingHand)
                            .child(StyledTextExt::icon_size(div().child("close"), 18.0).text_color(rgba(c.on_surface_variant))),
                    ),
            )
            .child(
                // Body
                if let Some(item) = entry {
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .pt_4()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .child(StyledTextExt::icon_size(div().child(item.icon.clone()), 24.0).text_color(rgba(c.primary)))
                                .child(
                                    div()
                                        .text_base()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .text_color(rgba(c.on_surface))
                                        .child(item.title.clone()),
                                ),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgba(c.on_surface_variant))
                                .child(item.description.clone()),
                        )
                } else {
                    div().child("No updates available.")
                },
            )
            .child(
                // Footer
                div()
                    .flex()
                    .flex_row()
                    .justify_end()
                    .pt_4()
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded_full()
                            .bg(rgba(c.primary))
                            .text_color(rgba(c.on_primary))
                            .text_xs()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .cursor(CursorStyle::PointingHand)
                            .child("Got it"),
                    ),
            )
    }
}
