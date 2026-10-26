//! Updates Popout
//! Parity with `shell/modules/bar/popouts/Updates.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug)]
pub struct UpdatesPopoutState {
    pub checking: bool,
    pub has_update: bool,
    pub branch: String,
    pub pending_count: usize,
    pub last_check_text: String,
    pub next_check_text: String,
}

impl Default for UpdatesPopoutState {
    fn default() -> Self {
        Self {
            checking: false,
            has_update: false,
            branch: "main".into(),
            pending_count: 0,
            last_check_text: "5m ago".into(),
            next_check_text: "25m".into(),
        }
    }
}

pub struct UpdatesPopout;

impl UpdatesPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &UpdatesPopoutState::default(), c)
    }

    pub fn render_with_state(center_x: f32, state: &UpdatesPopoutState, c: &CaelestiaColours) -> Div {
        let status_icon = if state.checking {
            "sync"
        } else if state.has_update {
            "update"
        } else {
            "check_circle"
        };

        let status_text = if state.checking {
            "Checking for updates…".to_string()
        } else if state.has_update {
            format!("{} new commits on {} branch", state.pending_count, state.branch)
        } else {
            "System is up to date".to_string()
        };

        let content = div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .gap(px(SPACING_SMALL))
            // Status Card
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(PADDING_MEDIUM))
                    .rounded(px(ROUNDING_MEDIUM))
                    .bg(rgba(c.surface_container))
                    .gap(px(SPACING_SMALL))
                    // Header Row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(SPACING_SMALL))
                            .child(
                                MaterialIcon::new(status_icon)
                                    .text_size(px(ICON_SIZE_MEDIUM))
                                    .text_color(if state.has_update {
                                        rgba(c.primary)
                                    } else {
                                        rgba(c.on_surface_variant)
                                    }),
                            )
                            .child(
                                StyledText::new(status_text)
                                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgba(c.on_surface)),
                            ),
                    )
                    // Last check row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(SPACING_SMALL))
                            .child(
                                MaterialIcon::new("history")
                                    .text_size(px(ICON_SIZE_SMALL))
                                    .text_color(rgba(c.on_surface_variant)),
                            )
                            .child(
                                StyledText::new(format!("Last check: {}", state.last_check_text))
                                    .text_size(px(FONT_SIZE_BODY_SMALL))
                                    .text_color(rgba(c.on_surface_variant)),
                            ),
                    )
                    // Next check row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(SPACING_SMALL))
                            .child(
                                MaterialIcon::new("schedule")
                                    .text_size(px(ICON_SIZE_SMALL))
                                    .text_color(rgba(c.on_surface_variant)),
                            )
                            .child(
                                StyledText::new(format!("Next check in: {}", state.next_check_text))
                                    .text_size(px(FONT_SIZE_BODY_SMALL))
                                    .text_color(rgba(c.on_surface_variant)),
                            ),
                    ),
            )
            // Actions Card
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(PADDING_SMALL))
                    .rounded(px(ROUNDING_MEDIUM))
                    .bg(rgba(c.surface_container))
                    .gap(px(SPACING_EXTRA_SMALL))
                    .child(ListRow::render(
                        "Check for updates",
                        None::<&str>,
                        Some("sync"),
                        None,
                        false,
                        false,
                        false,
                        c,
                    ))
                    .child(if state.has_update {
                        ListRow::render(
                            "View changes & update",
                            Some("Open package manager in Nexus"),
                            Some("open_in_new"),
                            None,
                            false,
                            false,
                            false,
                            c,
                        )
                    } else {
                        div()
                    })
                    .child(ListRow::render(
                        "Dismiss indicator",
                        Some("Hide bar icon until next check"),
                        Some("visibility_off"),
                        None,
                        false,
                        false,
                        false,
                        c,
                    )),
            );

        PopoutWrapper::render("System Updates", "system_update", center_x, content, c)
    }
}
