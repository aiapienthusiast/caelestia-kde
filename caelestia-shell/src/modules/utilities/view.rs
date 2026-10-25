//! Utilities View Component
//! Complete parity with `shell/modules/utilities/Wrapper.qml`, `Content.qml`,
//! and cards (`IdleInhibit.qml`, `CaptureCard.qml`, `QuickShareList.qml`, `Toggles.qml`).

use super::config::UtilitiesConfig;
use super::state::UtilitiesState;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, FocusHandle, IntoElement, KeyDownEvent,
    Render, Styled, Window,
};

pub struct UtilitiesView {
    pub state: UtilitiesState,
    pub config: UtilitiesConfig,
    pub colours: CaelestiaColours,
    pub focus_handle: FocusHandle,
}

impl UtilitiesView {
    pub fn new(cx: &mut gpui::Context<Self>, colours: CaelestiaColours) -> Self {
        let config = UtilitiesConfig::default();
        let mut state = UtilitiesState::new(&config);
        state.is_open = true;
        Self {
            state,
            config,
            colours,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Handle keyboard navigation
    pub fn handle_key_down(&mut self, event: &KeyDownEvent) {
        if !self.state.is_open {
            return;
        }

        if event.keystroke.key.as_str() == "escape" {
            if self.state.capture.pending_delete_path.is_some() {
                self.state.capture.cancel_delete();
            } else if self.state.quick_share.is_selector_open {
                self.state.quick_share.close_device_selector();
            } else {
                self.state.close();
            }
        }
    }
}

impl Render for UtilitiesView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        if !self.state.is_open {
            return div().id("utilities-hidden");
        }

        let bg_scrim = rgba(0x00000080);
        let surface = rgba(self.colours.surface);
        let surface_container = rgba(self.colours.surface_container);
        let surface_highest = rgba(self.colours.surface_container_highest);
        let primary = rgba(self.colours.primary);
        let on_primary = rgba(self.colours.on_primary);
        let on_surface = rgba(self.colours.on_surface);
        let on_surface_variant = rgba(self.colours.on_surface_variant);
        let primary_container = rgba(self.colours.primary_container);
        let on_primary_container = rgba(self.colours.on_primary_container);
        let secondary = rgba(self.colours.secondary);
        let on_secondary = rgba(self.colours.on_secondary);
        let secondary_container = rgba(self.colours.secondary_container);
        let on_secondary_container = rgba(self.colours.on_secondary_container);
        let error_container = rgba(self.colours.error_container);
        let error = rgba(self.colours.error);

        let is_recording = self.state.capture.is_recording;
        let is_keep_awake = self.state.idle_inhibit.state.enabled;

        div()
            .id("utilities-drawer-overlay")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _win, _cx| {
                this.handle_key_down(event);
            }))
            .size_full()
            .bg(bg_scrim)
            .flex()
            .flex_row()
            .justify_end()
            // Click outside cards to dismiss
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|this, _, _win, _cx| {
                    if this.state.capture.pending_delete_path.is_none()
                        && !this.state.quick_share.is_selector_open
                    {
                        this.state.close();
                    }
                }),
            )
            .child(
                // Utilities drawer panel container
                div()
                    .id("utilities-panel")
                    .w(px(380.0))
                    .h_full()
                    .bg(surface_container)
                    .border_l_1()
                    .border_color(surface_highest)
                    .shadow_xl()
                    .p(px(16.0))
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .overflow_hidden()
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, _| {}) // Stop propagation
                    .child(
                        // Drawer header
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(on_surface)
                                    .child("Utilities"),
                            )
                            .child(
                                div()
                                    .w(px(24.0))
                                    .h(px(24.0))
                                    .rounded(px(12.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(surface_highest)
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(|this, _, _win, _cx| {
                                            this.state.close();
                                        }),
                                    )
                                    .child(div().text_xs().text_color(on_surface).child("✕")),
                            ),
                    )
                    // CARD 1: Idle Inhibit (Keep Awake)
                    .when(self.config.show_keep_awake, |d| {
                        d.child(
                            div()
                                .id("card-keep-awake")
                                .p(px(12.0))
                                .rounded(px(16.0))
                                .bg(surface)
                                .border_1()
                                .border_color(if is_keep_awake { primary } else { surface_highest })
                                .flex()
                                .flex_col()
                                .gap(px(8.0))
                                .child(
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
                                                .gap(px(10.0))
                                                .child(
                                                    div()
                                                        .w(px(36.0))
                                                        .h(px(36.0))
                                                        .rounded(px(18.0))
                                                        .bg(if is_keep_awake { secondary } else { secondary_container })
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .text_color(if is_keep_awake { on_secondary } else { on_secondary_container })
                                                                .child("☕"),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                                .text_color(on_surface)
                                                                .child("Keep Awake"),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(on_surface_variant)
                                                                .child(if is_keep_awake {
                                                                    "Preventing sleep mode"
                                                                } else {
                                                                    "Normal power management"
                                                                }),
                                                        ),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .id("keep-awake-toggle")
                                                .w(px(40.0))
                                                .h(px(22.0))
                                                .rounded(px(11.0))
                                                .bg(if is_keep_awake { primary } else { surface_highest })
                                                .p(px(2.0))
                                                .cursor(CursorStyle::PointingHand)
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .justify_between()
                                                .on_mouse_down(
                                                    gpui::MouseButton::Left,
                                                    cx.listener(|this, _, _win, _cx| {
                                                        this.state.idle_inhibit.toggle();
                                                    }),
                                                )
                                                .child(
                                                    div()
                                                        .w(px(18.0))
                                                        .h(px(18.0))
                                                        .rounded(px(9.0))
                                                        .bg(if is_keep_awake { on_primary } else { on_surface_variant }),
                                                ),
                                        ),
                                )
                                .when(is_keep_awake, |card| {
                                    let active_str = self
                                        .state
                                        .idle_inhibit
                                        .active_time_string()
                                        .unwrap_or_else(|| "00:00".to_string());
                                    card.child(
                                        div()
                                            .px(px(8.0))
                                            .py(px(2.0))
                                            .rounded(px(12.0))
                                            .bg(primary_container)
                                            .text_xs()
                                            .text_color(on_primary_container)
                                            .child(format!("Active for {}", active_str)),
                                    )
                                }),
                        )
                    })
                    // CARD 2: Capture (Recording & Screenshots)
                    .when(self.config.show_screen_recorder, |d| {
                        d.child(
                            div()
                                .id("card-capture")
                                .p(px(12.0))
                                .rounded(px(16.0))
                                .bg(surface)
                                .border_1()
                                .border_color(surface_highest)
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
                                                .child("Screen Capture"),
                                        )
                                        .when(is_recording, |hdr| {
                                            hdr.child(
                                                div()
                                                    .px(px(6.0))
                                                    .py(px(2.0))
                                                    .rounded(px(6.0))
                                                    .bg(error_container)
                                                    .text_xs()
                                                    .text_color(error)
                                                    .child("● REC"),
                                            )
                                        }),
                                )
                                .child(
                                    // Action buttons row
                                    div()
                                        .flex()
                                        .flex_row()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .id("btn-record-screen")
                                                .flex_1()
                                                .py(px(8.0))
                                                .rounded(px(10.0))
                                                .bg(if is_recording { error_container } else { primary_container })
                                                .text_color(if is_recording { error } else { on_primary_container })
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .cursor(CursorStyle::PointingHand)
                                                .on_mouse_down(
                                                    gpui::MouseButton::Left,
                                                    cx.listener(|this, _, _win, _cx| {
                                                        if this.state.capture.is_recording {
                                                            this.state.capture.stop_recording();
                                                        } else {
                                                            this.state.capture.start_recording();
                                                        }
                                                    }),
                                                )
                                                .child(if is_recording { "Stop Record" } else { "Record Screen" }),
                                        )
                                        .child(
                                            div()
                                                .id("btn-screenshot-region")
                                                .flex_1()
                                                .py(px(8.0))
                                                .rounded(px(10.0))
                                                .bg(surface_highest)
                                                .text_color(on_surface)
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .cursor(CursorStyle::PointingHand)
                                                .on_mouse_down(
                                                    gpui::MouseButton::Left,
                                                    cx.listener(|this, _, _win, _cx| {
                                                        this.state.capture.take_screenshot(true);
                                                    }),
                                                )
                                                .child("Screenshot"),
                                        ),
                                ),
                        )
                    })
                    // CARD 3: QuickShare
                    .when(self.config.show_quick_share, |d| {
                        d.child(
                            div()
                                .id("card-quick-share")
                                .p(px(12.0))
                                .rounded(px(16.0))
                                .bg(surface)
                                .border_1()
                                .border_color(surface_highest)
                                .flex()
                                .flex_col()
                                .gap(px(8.0))
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
                                                .child("Quick Share"),
                                        )
                                        .child(
                                            div()
                                                .px(px(8.0))
                                                .py(px(4.0))
                                                .rounded(px(8.0))
                                                .bg(secondary_container)
                                                .text_xs()
                                                .text_color(on_secondary_container)
                                                .cursor(CursorStyle::PointingHand)
                                                .on_mouse_down(
                                                    gpui::MouseButton::Left,
                                                    cx.listener(|this, _, _win, _cx| {
                                                        this.state.quick_share.open_device_selector();
                                                    }),
                                                )
                                                .child("Discover"),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(on_surface_variant)
                                        .child(format!("Device: {}", self.state.quick_share.device_name)),
                                ),
                        )
                    })
                    // CARD 4: Quick Toggles Grid
                    .when(self.config.show_quick_toggles, |d| {
                        let toggles_model = &self.state.toggles;
                        let page_items = toggles_model.page_slice().to_vec();
                        let current_page = toggles_model.current_page;
                        let total_pages = toggles_model.page_count();

                        d.child(
                            div()
                                .id("card-quick-toggles")
                                .p(px(12.0))
                                .rounded(px(16.0))
                                .bg(surface)
                                .border_1()
                                .border_color(surface_highest)
                                .flex()
                                .flex_col()
                                .gap(px(10.0))
                                .child(
                                    // Header with pagination chevrons
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
                                                .child("Quick Toggles"),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap(px(6.0))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(on_surface)
                                                        .cursor(CursorStyle::PointingHand)
                                                        .on_mouse_down(
                                                            gpui::MouseButton::Left,
                                                            cx.listener(|this, _, _win, _cx| {
                                                                this.state.toggles.prev_page();
                                                            }),
                                                        )
                                                        .child("‹"),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(on_surface_variant)
                                                        .child(format!("{}/{}", current_page + 1, total_pages)),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(on_surface)
                                                        .cursor(CursorStyle::PointingHand)
                                                        .on_mouse_down(
                                                            gpui::MouseButton::Left,
                                                            cx.listener(|this, _, _win, _cx| {
                                                                this.state.toggles.next_page();
                                                            }),
                                                        )
                                                        .child("›"),
                                                ),
                                        ),
                                )
                                .child(
                                    // 2-column grid of toggles for current page
                                    div()
                                        .flex()
                                        .flex_row()
                                        .flex_wrap()
                                        .gap(px(8.0))
                                        .children(page_items.into_iter().map(|item| {
                                            let id = item.id.clone();
                                            let is_active = self.state.toggles.is_active(
                                                &item.id, None, None, None, None, None,
                                            );
                                            let chip_bg = if is_active { primary_container } else { surface_highest };
                                            let chip_fg = if is_active { on_primary_container } else { on_surface };

                                            div()
                                                .id(format!("toggle-{}", item.id))
                                                .w(px(168.0))
                                                .p(px(8.0))
                                                .rounded(px(12.0))
                                                .bg(chip_bg)
                                                .cursor(CursorStyle::PointingHand)
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap(px(8.0))
                                                .on_mouse_down(
                                                    gpui::MouseButton::Left,
                                                    cx.listener(move |this, _, _win, _cx| {
                                                        this.state.toggles.toggle_action(
                                                            &id, None, None, None, None, None,
                                                        );
                                                    }),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(gpui::FontWeight::MEDIUM)
                                                        .text_color(chip_fg)
                                                        .truncate()
                                                        .child(item.name),
                                                )
                                        })),
                                ),
                        )
                    }),
            )
            .when(self.state.quick_share.is_selector_open, |d| {
                // QuickShare Device Selector Modal
                d.child(
                    div()
                        .id("quick-share-device-selector-modal")
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(rgba(0x00000088))
                        .child(
                            div()
                                .w(px(400.0))
                                .p(px(20.0))
                                .rounded(px(16.0))
                                .bg(surface_container)
                                .border_2()
                                .border_color(primary)
                                .flex()
                                .flex_col()
                                .gap(px(12.0))
                                .child(
                                    div()
                                        .text_base()
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .text_color(on_surface)
                                        .child("Select Nearby Device"),
                                )
                                .child(
                                    div()
                                        .p(px(8.0))
                                        .rounded(px(8.0))
                                        .bg(surface_highest)
                                        .text_color(on_surface)
                                        .text_xs()
                                        .cursor(CursorStyle::PointingHand)
                                        .on_mouse_down(
                                            gpui::MouseButton::Left,
                                            cx.listener(|this, _, _win, _cx| {
                                                this.state.quick_share.close_device_selector();
                                            }),
                                        )
                                        .child("Cancel"),
                                ),
                        ),
                )
            })
    }
}
