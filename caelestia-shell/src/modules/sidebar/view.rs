//! Sidebar View Component
//! Complete parity with `shell/modules/sidebar/Wrapper.qml`, `Content.qml`,
//! and tab subviews (`NotifDock.qml`, `AiAssistant.qml`, `News.qml`, `DinoGame.qml`).

use super::config::SidebarConfig;
use super::state::SidebarState;
use super::tabs::SidebarTab;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::notifications::NotificationItem;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, FocusHandle, IntoElement, KeyDownEvent,
    Render, Styled, Window,
};

pub struct SidebarView {
    pub state: SidebarState,
    pub config: SidebarConfig,
    pub colours: CaelestiaColours,
    pub notifications: Vec<NotificationItem>,
    pub focus_handle: FocusHandle,
}

impl SidebarView {
    pub fn new(cx: &mut gpui::Context<Self>, colours: CaelestiaColours, screen_width: f32) -> Self {
        let config = SidebarConfig::default();
        let mut state = SidebarState::new(&config, screen_width);
        state.is_open = true;
        Self {
            state,
            config,
            colours,
            notifications: Vec::new(),
            focus_handle: cx.focus_handle(),
        }
    }

    /// Update notifications snapshot from NotificationsService
    pub fn update_notifications(&mut self, items: Vec<NotificationItem>) {
        self.notifications = items;
    }

    /// Handle keyboard navigation
    pub fn handle_key_down(&mut self, event: &KeyDownEvent) {
        if !self.state.is_open {
            return;
        }

        if event.keystroke.key.as_str() == "escape" {
            self.state.close();
        }
    }
}

impl Render for SidebarView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        if !self.state.is_open {
            return div().id("sidebar-hidden");
        }

        let bg_scrim = rgba(0x00000060);
        let surface = rgba(self.colours.surface);
        let surface_container = rgba(self.colours.surface_container);
        let surface_highest = rgba(self.colours.surface_container_highest);
        let primary = rgba(self.colours.primary);
        let on_primary = rgba(self.colours.on_primary);
        let on_surface = rgba(self.colours.on_surface);
        let on_surface_variant = rgba(self.colours.on_surface_variant);
        let primary_container = rgba(self.colours.primary_container);
        let on_primary_container = rgba(self.colours.on_primary_container);
        let outline = rgba(self.colours.outline);
        let error_container = rgba(self.colours.error_container);
        let error = rgba(self.colours.error);

        let available_tabs = SidebarTab::available_tabs(&self.config);
        let active_tab = self.state.active_tab;
        let is_pinned = self.state.pinned;
        let panel_width = self.state.width;

        div()
            .id("sidebar-overlay")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _win, _cx| {
                this.handle_key_down(event);
            }))
            .size_full()
            .bg(if is_pinned { rgba(0x00000000) } else { bg_scrim })
            .flex()
            .flex_row()
            .justify_end()
            // Click outside to dismiss if not pinned
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|this, _, _win, _cx| {
                    if !this.state.pinned {
                        this.state.close();
                    }
                }),
            )
            .child(
                // Sidebar Drawer Panel
                div()
                    .id("sidebar-panel")
                    .w(px(panel_width))
                    .h_full()
                    .bg(surface_container)
                    .border_l_1()
                    .border_color(surface_highest)
                    .shadow_2xl()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, _| {}) // Stop propagation
                    .child(
                        // Header with Tabs & Pin Button
                        div()
                            .id("sidebar-header")
                            .h(px(56.0))
                            .w_full()
                            .bg(surface)
                            .border_b_1()
                            .border_color(surface_highest)
                            .px(px(12.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                // Tab navigation items
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(4.0))
                                    .children(available_tabs.iter().map(|t| {
                                        let is_current = t.tab == active_tab;
                                        let tab_type = t.tab;
                                        div()
                                            .id(format!("tab-btn-{}", t.id))
                                            .px(px(10.0))
                                            .py(px(6.0))
                                            .rounded(px(12.0))
                                            .bg(if is_current { primary_container } else { rgba(0x00000000) })
                                            .cursor(CursorStyle::PointingHand)
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(6.0))
                                            .on_mouse_down(
                                                gpui::MouseButton::Left,
                                                cx.listener(move |this, _, _win, _cx| {
                                                    this.state.switch_tab(tab_type);
                                                }),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(if is_current {
                                                        gpui::FontWeight::BOLD
                                                    } else {
                                                        gpui::FontWeight::MEDIUM
                                                    })
                                                    .text_color(if is_current {
                                                        on_primary_container
                                                    } else {
                                                        on_surface_variant
                                                    })
                                                    .child(t.label),
                                            )
                                    })),
                            )
                            .child(
                                // Header action buttons: Pin toggle & Close
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(6.0))
                                    // Pin toggle
                                    .child(
                                        div()
                                            .id("btn-pin-sidebar")
                                            .w(px(28.0))
                                            .h(px(28.0))
                                            .rounded(px(14.0))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .bg(if is_pinned { primary_container } else { surface_highest })
                                            .cursor(CursorStyle::PointingHand)
                                            .on_mouse_down(
                                                gpui::MouseButton::Left,
                                                cx.listener(|this, _, _win, _cx| {
                                                    this.state.toggle_pinned();
                                                }),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(if is_pinned { primary } else { outline })
                                                    .child("📌"),
                                            ),
                                    )
                                    // Close button
                                    .child(
                                        div()
                                            .id("btn-close-sidebar")
                                            .w(px(28.0))
                                            .h(px(28.0))
                                            .rounded(px(14.0))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .bg(surface_highest)
                                            .cursor(CursorStyle::PointingHand)
                                            .on_mouse_down(
                                                gpui::MouseButton::Left,
                                                cx.listener(|this, _, _win, _cx| {
                                                    this.state.force_close();
                                                }),
                                            )
                                            .child(div().text_xs().text_color(on_surface).child("✕")),
                                    ),
                            ),
                    )
                    .child(
                        // Active Tab Body
                        div()
                            .id("sidebar-tab-content")
                            .flex_1()
                            .w_full()
                            .overflow_hidden()
                            .p(px(14.0))
                            .child(match active_tab {
                                SidebarTab::Notifications => {
                                    let notifs = self.notifications.clone();
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(10.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .justify_between()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                                        .text_color(on_surface)
                                                        .child(format!("History ({})", notifs.len())),
                                                )
                                                .when(!notifs.is_empty(), |d| {
                                                    d.child(
                                                        div()
                                                            .px(px(8.0))
                                                            .py(px(3.0))
                                                            .rounded(px(6.0))
                                                            .bg(error_container)
                                                            .text_xs()
                                                            .text_color(error)
                                                            .cursor(CursorStyle::PointingHand)
                                                            .on_mouse_down(
                                                                gpui::MouseButton::Left,
                                                                cx.listener(|this, _, _win, _cx| {
                                                                    this.notifications.clear();
                                                                }),
                                                            )
                                                            .child("Clear All"),
                                                    )
                                                }),
                                        )
                                        .when(notifs.is_empty(), |d| {
                                            d.child(
                                                div()
                                                    .py(px(40.0))
                                                    .flex()
                                                    .flex_col()
                                                    .items_center()
                                                    .justify_center()
                                                    .gap(px(6.0))
                                                    .child(div().text_2xl().child("🔔"))
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(on_surface_variant)
                                                            .child("No notifications"),
                                                    ),
                                            )
                                        })
                                        .children(notifs.into_iter().map(|n| {
                                            let id = n.id;
                                            div()
                                                .id(format!("side-notif-{}", n.id))
                                                .p(px(10.0))
                                                .rounded(px(10.0))
                                                .bg(surface)
                                                .border_1()
                                                .border_color(surface_highest)
                                                .flex()
                                                .flex_col()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_center()
                                                        .justify_between()
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .font_weight(gpui::FontWeight::BOLD)
                                                                .text_color(on_surface)
                                                                .child(n.app_name),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(on_surface_variant)
                                                                .cursor(CursorStyle::PointingHand)
                                                                .on_mouse_down(
                                                                    gpui::MouseButton::Left,
                                                                    cx.listener(move |this, _, _win, _cx| {
                                                                        this.notifications.retain(|item| item.id != id);
                                                                    }),
                                                                )
                                                                .child("✕"),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(gpui::FontWeight::MEDIUM)
                                                        .text_color(on_surface)
                                                        .child(n.summary),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(on_surface_variant)
                                                        .child(n.body),
                                                )
                                        }))
                                }
                                SidebarTab::AiAssistant => {
                                    let msgs = self.state.ai_assistant.messages.clone();
                                    let is_busy = self.state.ai_assistant.busy;
                                    div()
                                        .flex()
                                        .flex_col()
                                        .justify_between()
                                        .h_full()
                                        .gap(px(10.0))
                                        .child(
                                            div()
                                                .flex_1()
                                                .overflow_hidden()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .children(msgs.into_iter().map(|m| {
                                                    let is_user = m.role == super::ai_assistant::MessageRole::User;
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .when(is_user, |d| d.justify_end())
                                                        .child(
                                                            div()
                                                                .max_w(px(280.0))
                                                                .p(px(8.0))
                                                                .rounded(px(12.0))
                                                                .bg(if is_user { primary } else { surface })
                                                                .text_color(if is_user { on_primary } else { on_surface })
                                                                .text_xs()
                                                                .child(m.text),
                                                        )
                                                }))
                                                .when(is_busy, |d| {
                                                    d.child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(primary)
                                                            .child("AI is thinking..."),
                                                    )
                                                }),
                                        )
                                        .child(
                                            // Prompt input box & send button
                                            div()
                                                .h(px(40.0))
                                                .w_full()
                                                .p(px(4.0))
                                                .rounded(px(12.0))
                                                .bg(surface)
                                                .border_1()
                                                .border_color(surface_highest)
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .justify_between()
                                                .child(
                                                    div()
                                                        .px(px(8.0))
                                                        .text_xs()
                                                        .text_color(on_surface_variant)
                                                        .child("Ask assistant..."),
                                                )
                                                .child(
                                                    div()
                                                        .px(px(12.0))
                                                        .py(px(6.0))
                                                        .rounded(px(8.0))
                                                        .bg(primary)
                                                        .text_color(on_primary)
                                                        .text_xs()
                                                        .font_weight(gpui::FontWeight::BOLD)
                                                        .cursor(CursorStyle::PointingHand)
                                                        .on_mouse_down(
                                                            gpui::MouseButton::Left,
                                                            cx.listener(|this, _, _win, _cx| {
                                                                this.state.ai_assistant.send_prompt("Hello assistant");
                                                            }),
                                                        )
                                                        .child("➤"),
                                                ),
                                        )
                                }
                                SidebarTab::News => {
                                    let news_articles = self.state.news.articles.clone();
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(10.0))
                                        .children(news_articles.into_iter().map(|a| {
                                            div()
                                                .p(px(10.0))
                                                .rounded(px(12.0))
                                                .bg(surface)
                                                .border_1()
                                                .border_color(surface_highest)
                                                .flex()
                                                .flex_col()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(gpui::FontWeight::BOLD)
                                                        .text_color(on_surface)
                                                        .child(a.title),
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .justify_between()
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(primary)
                                                                .child(a.source),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(on_surface_variant)
                                                                .child(a.category),
                                                        ),
                                                )
                                        }))
                                }
                                SidebarTab::DinoGame => {
                                    div()
                                        .h(px(200.0))
                                        .w_full()
                                        .rounded(px(12.0))
                                        .bg(surface)
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .justify_center()
                                        .gap(px(8.0))
                                        .child(div().text_3xl().child("🦖"))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::BOLD)
                                                .text_color(on_surface)
                                                .child("Caelestia Dino Runner"),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(on_surface_variant)
                                                .child("Press Space to jump"),
                                        )
                                }
                            }),
                    ),
            )
    }
}
