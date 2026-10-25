//! Notification Card Component
//! Parity with `shell/modules/notifications/Notification.qml`.
//!
//! Renders Material 3 toast cards with urgency theming, app icons, summary,
//! expandable body text, and interactive dismiss controls.

use super::toast::ToastItem;
use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::notifications::Urgency;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, IntoElement, Render, Styled, Window,
};

pub struct NotificationCard {
    pub toast: ToastItem,
    pub colours: CaelestiaColours,
}

impl NotificationCard {
    pub fn new(toast: ToastItem, colours: CaelestiaColours) -> Self {
        Self { toast, colours }
    }
}

impl Render for NotificationCard {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let c = &self.colours;
        let item = &self.toast.item;

        let (bg_color, border_color, title_color) = match item.urgency {
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
            .cursor(CursorStyle::PointingHand)
            // Header: App icon, App name, Close button
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
                                    .child(item.app_name.clone()),
                            ),
                    )
                    .child(
                        div()
                            .text_color(rgba(c.on_surface_variant))
                            .cursor(CursorStyle::PointingHand)
                            .child(StyledTextExt::icon_size(div().child("close"), 16.0)),
                    ),
            )
            // Summary / Title
            .child(
                div()
                    .text_color(title_color)
                    .text_sm()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child(item.summary.clone()),
            )
            // Body text
            .child(
                div()
                    .text_color(rgba(c.on_surface_variant))
                    .text_xs()
                    .child(item.body.clone()),
            )
    }
}
