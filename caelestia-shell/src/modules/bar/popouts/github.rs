//! Github Popout
//! Parity with `shell/modules/bar/popouts/Github.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::icon_text_button::IconTextButton;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug)]
pub struct GithubPopoutState {
    pub username: String,
    pub total_commits: u32,
    pub last_error: Option<String>,
}

impl Default for GithubPopoutState {
    fn default() -> Self {
        Self {
            username: "ladybug-me".into(),
            total_commits: 42,
            last_error: None,
        }
    }
}

pub struct GithubPopout;

impl GithubPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &GithubPopoutState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &GithubPopoutState,
        c: &CaelestiaColours,
    ) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .w(px(300.0))
            .gap(px(SPACING_SMALL))
            // Profile & Stats Card
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(PADDING_MEDIUM))
                    .rounded(px(ROUNDING_MEDIUM))
                    .bg(rgba(c.surface_container))
                    .gap(px(SPACING_SMALL))
                    // Username Row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(SPACING_SMALL))
                            .child(
                                MaterialIcon::new("person")
                                    .text_size(px(ICON_SIZE_MEDIUM))
                                    .text_color(rgba(c.on_surface_variant)),
                            )
                            .child(
                                StyledText::new(if !state.username.is_empty() {
                                    format!("@{}", state.username)
                                } else {
                                    "Not authenticated".to_string()
                                })
                                .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(c.on_surface)),
                            ),
                    )
                    // Stats / Error Row
                    .child(
                        if let Some(ref err) = state.last_error {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(SPACING_SMALL))
                                .child(
                                    MaterialIcon::new("error")
                                        .text_size(px(ICON_SIZE_MEDIUM))
                                        .text_color(rgba(c.error)),
                                )
                                .child(
                                    StyledText::new(err.clone())
                                        .text_size(px(FONT_SIZE_BODY_SMALL))
                                        .text_color(rgba(c.error)),
                                )
                        } else {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(SPACING_SMALL))
                                        .child(
                                            MaterialIcon::new("history")
                                                .text_size(px(ICON_SIZE_MEDIUM))
                                                .text_color(rgba(c.on_surface_variant)),
                                        )
                                        .child(
                                            StyledText::new("Last 7 days")
                                                .text_size(px(FONT_SIZE_BODY_SMALL))
                                                .text_color(rgba(c.on_surface_variant)),
                                        ),
                                )
                                .child(
                                    StyledText::new(format!("{} commits", state.total_commits))
                                        .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgba(c.on_surface)),
                                )
                        },
                    ),
            )
            // Open Profile Action Button
            .child(
                IconTextButton::new("open_in_new", "Open profile")
                    .with_colours(c.clone())
                    .into_element(),
            );

        PopoutWrapper::render("GitHub", "commit", center_x, content, c)
    }
}
