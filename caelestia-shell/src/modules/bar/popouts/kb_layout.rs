//! Keyboard Layout Popout
//! Parity with `shell/modules/bar/popouts/kblayout/KbLayout.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString, Styled};

#[derive(Clone, Debug)]
pub struct KbLayoutItem {
    pub index: usize,
    pub code: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct KbLayoutState {
    pub layouts: Vec<KbLayoutItem>,
    pub active_index: usize,
}

impl Default for KbLayoutState {
    fn default() -> Self {
        Self {
            layouts: vec![
                KbLayoutItem {
                    index: 0,
                    code: "us".into(),
                    name: "English (US)".into(),
                },
                KbLayoutItem {
                    index: 1,
                    code: "de".into(),
                    name: "German (DE)".into(),
                },
                KbLayoutItem {
                    index: 2,
                    code: "fr".into(),
                    name: "French (FR)".into(),
                },
                KbLayoutItem {
                    index: 3,
                    code: "jp".into(),
                    name: "Japanese (Kana/Romaji)".into(),
                },
            ],
            active_index: 0,
        }
    }
}

pub struct KbLayoutPopout;

impl KbLayoutPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &KbLayoutState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &KbLayoutState,
        c: &CaelestiaColours,
    ) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .w(px(260.0))
            .p(px(PADDING_EXTRA_SMALL))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .gap(px(2.0))
            .children(state.layouts.iter().map(|layout| {
                let is_active = layout.index == state.active_index;
                let bg_col = if is_active {
                    rgba(c.secondary_container)
                } else {
                    rgba(0x00000000)
                };
                let fg_col = if is_active {
                    rgba(c.on_secondary_container)
                } else {
                    rgba(c.on_surface)
                };

                div()
                    .id(SharedString::from(format!("kb_layout_{}", layout.code)))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(PADDING_MEDIUM))
                    .py(px(PADDING_SMALL))
                    .rounded(px(ROUNDING_SMALL))
                    .bg(bg_col)
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(c.surface_container_highest)))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(SPACING_SMALL))
                            .child(
                                div()
                                    .w(px(28.0))
                                    .h(px(20.0))
                                    .rounded(px(4.0))
                                    .bg(rgba(c.surface_container_highest))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        StyledText::new(layout.code.to_uppercase())
                                            .text_size(px(FONT_SIZE_LABEL_SMALL))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgba(c.on_surface_variant)),
                                    ),
                            )
                            .child(
                                StyledText::new(layout.name.clone())
                                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                    .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                                    .text_color(fg_col),
                            ),
                    )
                    .child(if is_active {
                        div().child(
                            MaterialIcon::new("check")
                                .text_size(px(ICON_SIZE_SMALL))
                                .text_color(fg_col),
                        )
                    } else {
                        div()
                    })
            }));

        PopoutWrapper::render("Keyboard Layouts", "keyboard", center_x, content, c)
    }
}
