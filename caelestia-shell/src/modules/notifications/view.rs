//! Notifications Stack View Component
//! Parity with `shell/modules/notifications/Content.qml` and `Wrapper.qml`.
//!
//! Renders the vertical floating toast stack anchored to the screen corner.

use super::config::NotificationConfig;
use super::manager::NotificationManager;
use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::notifications::{NotificationsService, Urgency};
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, IntoElement, Render, Styled, Window,
};

pub struct NotificationsView {
    pub manager: NotificationManager,
    pub colours: CaelestiaColours,
    pub has_fullscreen: bool,
    pub overview_open: bool,
}

impl NotificationsView {
    pub fn new(colours: CaelestiaColours) -> Self {
        Self {
            manager: NotificationManager::new(NotificationConfig::default()),
            colours,
            has_fullscreen: false,
            overview_open: false,
        }
    }

    /// Synchronize with backend service
    pub fn sync(&mut self, service: &NotificationsService) {
        self.manager.sync_with_service(service);
    }

    /// Periodic timer tick for auto-dismiss
    pub fn tick(&mut self) {
        self.manager.tick();
    }
}

impl Render for NotificationsView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let popups = self.manager.visible_popups(self.has_fullscreen, self.overview_open);
        if popups.is_empty() {
            return div().into_any_element();
        }

        let c = &self.colours;

        div()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .children(
                popups.into_iter().map(|toast| {
                    let id = toast.item.id;
                    let urgency = toast.item.urgency;
                    let app_name = toast.item.app_name.clone();
                    let summary = toast.item.summary.clone();
                    let body = toast.item.body.clone();

                    let (bg_color, border_color, title_color) = match urgency {
                        Urgency::Critical => (
                            rgba(c.secondary_container),
                            rgba(c.error),
                            rgba(c.on_secondary_container),
                        ),
                        Urgency::Low => (
                            rgba(c.surface_container_low),
                            rgba(c.outline_variant),
                            rgba(c.on_surface_variant),
                        ),
                        Urgency::Normal => (
                            rgba(c.surface_container),
                            rgba(c.outline_variant),
                            rgba(c.on_surface),
                        ),
                    };

                    div()
                        .w(px(360.0))
                        .rounded_2xl()
                        .bg(bg_color)
                        .border_1()
                        .border_color(border_color)
                        .shadow_lg()
                        .p_4()
                        .flex()
                        .flex_col()
                        .gap_2()
                        // Header
                        .child(
                            div()
                                .w_full()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .w(px(24.0))
                                                .h(px(24.0))
                                                .rounded_full()
                                                .bg(rgba(c.primary_container))
                                                .text_color(rgba(c.on_primary_container))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(StyledTextExt::icon_size(div().child("notifications"), 14.0)),
                                        )
                                        .child(
                                            div()
                                                .text_color(rgba(c.on_surface_variant))
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::MEDIUM)
                                                .child(app_name),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_color(rgba(c.on_surface_variant))
                                        .cursor(CursorStyle::PointingHand)
                                        .child(StyledTextExt::icon_size(div().child("close"), 16.0))
                                        .on_mouse_down(
                                            gpui::MouseButton::Left,
                                            cx.listener(move |this, _, _, _cx| {
                                                if let Some(t) = this.manager.toasts.iter_mut().find(|t| t.item.id == id) {
                                                    t.close();
                                                }
                                            }),
                                        ),
                                ),
                        )
                        // Summary
                        .child(
                            div()
                                .text_color(title_color)
                                .text_sm()
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .child(summary),
                        )
                        // Body
                        .child(
                            div()
                                .text_color(rgba(c.on_surface_variant))
                                .text_xs()
                                .child(body),
                        )
                }),
            )
            .into_any_element()
    }
}
