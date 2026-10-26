//! Notifications Popout
//! Parity with `shell/modules/bar/popouts/Notifications.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::icon_text_button::IconTextButton;
use crate::components::controls::list_row::ListRow;
use crate::components::controls::toggle_row::ToggleRow;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

#[derive(Clone, Debug)]
pub struct NotificationEntry {
    pub title: String,
    pub body: String,
    pub icon: String,
}

#[derive(Clone, Debug)]
pub struct NotificationsPopoutState {
    pub dnd: bool,
    pub unread_count: usize,
    pub notifications: Vec<NotificationEntry>,
}

impl Default for NotificationsPopoutState {
    fn default() -> Self {
        Self {
            dnd: false,
            unread_count: 2,
            notifications: vec![
                NotificationEntry {
                    title: "Cargo Build Finished".into(),
                    body: "Compiling caelestia-shell v0.1.0 completed in 2.4s".into(),
                    icon: "check_circle".into(),
                },
                NotificationEntry {
                    title: "System Update".into(),
                    body: "12 packages ready to install".into(),
                    icon: "system_update".into(),
                },
            ],
        }
    }
}

pub struct NotificationsPopout;

impl NotificationsPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &NotificationsPopoutState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &NotificationsPopoutState,
        c: &CaelestiaColours,
    ) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .w(px(300.0))
            .gap(px(SPACING_SMALL))
            // DND & Unread Count Card
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(PADDING_MEDIUM))
                    .rounded(px(ROUNDING_MEDIUM))
                    .bg(rgba(c.surface_container))
                    .gap(px(SPACING_SMALL))
                    .child(ToggleRow::render(
                        "Do not disturb",
                        None::<&str>,
                        Some("notifications_off"),
                        state.dnd,
                        false,
                        false,
                        false,
                        c,
                    ))
                    .child(
                        StyledText::new(if state.dnd {
                            "Notifications off".to_string()
                        } else {
                            format!("{} unread", state.unread_count)
                        })
                        .text_size(px(FONT_SIZE_BODY_SMALL))
                        .text_color(rgba(c.on_surface_variant)),
                    ),
            )
            // Notification Items List
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(SPACING_EXTRA_SMALL))
                    .children(state.notifications.iter().map(|notif| {
                        ListRow::render(
                            &notif.title,
                            Some(notif.body.as_str()),
                            Some(notif.icon.as_str()),
                            None,
                            false,
                            false,
                            false,
                            c,
                        )
                    })),
            )
            // Clear All Action Button
            .child(
                IconTextButton::new("clear_all", "Clear all")
                    .with_colours(c.clone())
                    .into_element(),
            );

        PopoutWrapper::render("Notifications", "notifications", center_x, content, c)
    }
}
