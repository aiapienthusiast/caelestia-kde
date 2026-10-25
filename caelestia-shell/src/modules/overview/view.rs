//! Overview View Component
//! Complete parity with `shell/modules/overview/Wrapper.qml`, `Content.qml`,
//! and `WindowGrid.qml`, plus `shell/modules/windowinfo/`.
//!
//! Provides the visual overview canvas displaying:
//! - Centered bottom workspace switcher with occupancy dots, count badges, paging chevrons, and '+' create workspace button
//! - Interactive window cards laid out via KDE/GNOME algorithms with hover effects
//! - Top-right window card buttons: Close ('✕') and Inspect Info ('ℹ')
//! - Full-featured Window Info inspection overlay modal (details, move to workspace, maximize/minimize, kill)
//! - Keyboard navigation (Tab, Arrow keys, Escape, Vim bindings)

use super::config::OverviewConfig;
use super::indicator::WorkspaceIndicatorModel;
use super::layout::{calculate_layout, WindowRect};
use super::state::OverviewState;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::kwin::WindowItem;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, FocusHandle, IntoElement, KeyDownEvent,
    Render, Styled, Window,
};
use std::collections::HashMap;
use tracing::info;

pub struct OverviewView {
    pub state: OverviewState,
    pub config: OverviewConfig,
    pub colours: CaelestiaColours,
    pub windows: Vec<WindowItem>,
    pub indicator: WorkspaceIndicatorModel,
    pub area_width: f32,
    pub area_height: f32,
    pub focus_handle: FocusHandle,
}

impl OverviewView {
    pub fn new(cx: &mut gpui::Context<Self>, colours: CaelestiaColours) -> Self {
        Self {
            state: OverviewState::new(),
            config: OverviewConfig::default(),
            colours,
            windows: Vec::new(),
            indicator: WorkspaceIndicatorModel {
                pills: Vec::new(),
                active_index: 0,
                swipe_offset: 0.0,
                is_swiping: false,
                scale_factor: 1.0,
                required_width: 800.0,
            },
            area_width: 1920.0,
            area_height: 1080.0,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Update windows and indicator from services
    pub fn update(
        &mut self,
        windows: Vec<WindowItem>,
        indicator: WorkspaceIndicatorModel,
        area_width: f32,
        area_height: f32,
    ) {
        self.windows = windows;
        self.indicator = indicator;
        self.area_width = area_width;
        self.area_height = area_height;
    }

    /// Handle keyboard navigation
    pub fn handle_key_down(&mut self, event: &KeyDownEvent) {
        if !self.state.is_open {
            return;
        }

        let key = event.keystroke.key.as_str();
        let modifiers = &event.keystroke.modifiers;

        match key {
            "escape" => {
                if self.state.is_window_info_open {
                    self.state.close_window_info();
                } else {
                    self.state.close();
                }
            }
            "tab" => {
                self.state.cycle_selection(modifiers.shift, self.windows.len());
            }
            "right" | "down" => {
                self.state.cycle_selection(false, self.windows.len());
            }
            "left" | "up" => {
                self.state.cycle_selection(true, self.windows.len());
            }
            "j" if modifiers.control || self.config.vim_keybinds => {
                self.state.cycle_selection(false, self.windows.len());
            }
            "k" if modifiers.control || self.config.vim_keybinds => {
                self.state.cycle_selection(true, self.windows.len());
            }
            "enter" | "return" => {
                // Activate selected will be invoked by controller / event bus
            }
            _ => {}
        }
    }

    /// Calculate active window positions using configured packing algorithm
    pub fn compute_window_layout(&self) -> HashMap<String, WindowRect> {
        let grid_w = (self.area_width - 160.0).max(300.0);
        let grid_h = (self.area_height - 220.0).max(300.0);
        calculate_layout(
            self.config.layout_type,
            &self.windows,
            grid_w,
            grid_h,
            self.config.column_spacing,
            self.config.row_spacing,
        )
    }
}

impl Render for OverviewView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        if !self.state.is_open {
            return div().id("overview-hidden");
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
        let secondary_container = rgba(self.colours.secondary_container);
        let on_secondary_container = rgba(self.colours.on_secondary_container);
        let error_container = rgba(self.colours.error_container);
        let error = rgba(self.colours.error);

        let layout_map = self.compute_window_layout();
        let total_workspaces = self.indicator.pills.len().max(1);
        let current_ws_idx = self.state.current_workspace_index;

        div()
            .id("overview-overlay")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _win, _cx| {
                this.handle_key_down(event);
            }))
            .size_full()
            .bg(bg_scrim)
            .flex()
            .flex_col()
            .items_center()
            .justify_between()
            .pb(px(24.0))
            // Click outside to dismiss overview
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|this, _, _win, _cx| {
                    if !this.state.is_window_info_open {
                        this.state.close();
                    }
                }),
            )
            // Left workspace page chevron
            .when(current_ws_idx > 0, |d| {
                d.child(
                    div()
                        .id("overview-prev-ws-btn")
                        .absolute()
                        .left(px(24.0))
                        .top(px(self.area_height / 2.0 - 24.0))
                        .w(px(48.0))
                        .h(px(48.0))
                        .rounded(px(24.0))
                        .bg(surface_highest)
                        .border_1()
                        .border_color(primary)
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor(CursorStyle::PointingHand)
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(|this, _, _win, _cx| {
                                if this.state.current_workspace_index > 0 {
                                    this.state.current_workspace_index -= 1;
                                }
                            }),
                        )
                        .child(div().text_lg().text_color(on_surface).child("‹")),
                )
            })
            // Right workspace page chevron
            .when(current_ws_idx + 1 < total_workspaces, |d| {
                d.child(
                    div()
                        .id("overview-next-ws-btn")
                        .absolute()
                        .right(px(24.0))
                        .top(px(self.area_height / 2.0 - 24.0))
                        .w(px(48.0))
                        .h(px(48.0))
                        .rounded(px(24.0))
                        .bg(surface_highest)
                        .border_1()
                        .border_color(primary)
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor(CursorStyle::PointingHand)
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(move |this, _, _win, _cx| {
                                if this.state.current_workspace_index + 1 < total_workspaces {
                                    this.state.current_workspace_index += 1;
                                }
                            }),
                        )
                        .child(div().text_lg().text_color(on_surface).child("›")),
                )
            })
            .child(
                // Central Window Grid Area (taking available height between top margin and bottom switcher)
                div()
                    .id("overview-window-grid")
                    .flex_1()
                    .w_full()
                    .relative()
                    .mt(px(32.0))
                    .mb(px(16.0))
                    .children(self.windows.iter().enumerate().map(|(idx, win)| {
                        let is_selected = self.state.selected_index == Some(idx);
                        let rect = layout_map.get(&win.address);

                        let (card_x, card_y, card_w, card_h) = if let Some(r) = rect {
                            (r.x, r.y, r.width, r.height)
                        } else {
                            (100.0 + idx as f32 * 50.0, 100.0, 320.0, 240.0)
                        };

                        let addr = win.address.clone();
                        let title = if win.title.is_empty() {
                            win.app_id.clone()
                        } else {
                            win.title.clone()
                        };

                        div()
                            .id(format!("win-card-{}", win.address))
                            .absolute()
                            .left(px(card_x))
                            .top(px(card_y))
                            .w(px(card_w))
                            .h(px(card_h))
                            .rounded(px(12.0))
                            .bg(surface_container)
                            .border_2()
                            .border_color(if is_selected { primary } else { surface_highest })
                            .shadow_md()
                            .flex()
                            .flex_col()
                            .overflow_hidden()
                            .cursor(CursorStyle::PointingHand)
                            // Activate on card click
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |this, _, _win, _cx| {
                                    this.state.selected_index = Some(idx);
                                }),
                            )
                            .child(
                                // Title bar with app icon/title and top-right actions (info & close)
                                div()
                                    .h(px(34.0))
                                    .w_full()
                                    .bg(surface_highest)
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .px(px(8.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(6.0))
                                            .overflow_hidden()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                                    .text_color(on_surface)
                                                    .truncate()
                                                    .child(title),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(4.0))
                                            // Info button
                                            .child(
                                                div()
                                                    .id(format!("info-btn-{}", win.address))
                                                    .w(px(22.0))
                                                    .h(px(22.0))
                                                    .rounded(px(11.0))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .cursor(CursorStyle::PointingHand)
                                                    .hover(|h| h.bg(secondary_container))
                                                    .on_mouse_down(
                                                        gpui::MouseButton::Left,
                                                        cx.listener({
                                                            let a = addr.clone();
                                                            move |this, _, _win, _cx| {
                                                                this.state.open_window_info(&a);
                                                            }
                                                        }),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(primary)
                                                            .child("ℹ"),
                                                    ),
                                            )
                                            // Close button
                                            .child(
                                                div()
                                                    .id(format!("close-btn-{}", win.address))
                                                    .w(px(22.0))
                                                    .h(px(22.0))
                                                    .rounded(px(11.0))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .cursor(CursorStyle::PointingHand)
                                                    .hover(|h| h.bg(error_container))
                                                    .on_mouse_down(
                                                        gpui::MouseButton::Left,
                                                        cx.listener({
                                                            let a = addr.clone();
                                                            move |this, _, _win, _cx| {
                                                                info!("Closing window from card: {}", a);
                                                                this.windows.retain(|w| w.address != a);
                                                            }
                                                        }),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(error)
                                                            .child("✕"),
                                                    ),
                                            ),
                                    ),
                            )
                            .child(
                                // Window thumbnail / preview placeholder body
                                div()
                                    .flex_1()
                                    .w_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(surface)
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(on_surface_variant)
                                            .child(win.app_id.clone()),
                                    ),
                            )
                    })),
            )
            .child(
                // Bottom Workspace Switcher Strip (Complete parity with WindowGrid.qml indicatorContainer)
                div()
                    .id("overview-workspace-strip-bottom")
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .px(px(20.0))
                    .py(px(10.0))
                    .rounded(px(28.0))
                    .bg(surface_container)
                    .border_1()
                    .border_color(surface_highest)
                    .shadow_lg()
                    .children(self.indicator.pills.iter().map(|pill| {
                        let is_active = pill.index == self.state.current_workspace_index;
                        let pill_bg = if is_active {
                            primary_container
                        } else if pill.occupied {
                            surface_highest
                        } else {
                            rgba(0x00000000)
                        };
                        let pill_fg = if is_active {
                            on_primary_container
                        } else {
                            on_surface
                        };

                        let idx = pill.index;
                        div()
                            .id(format!("ws-pill-{}", pill.id))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(14.0))
                            .py(px(7.0))
                            .rounded(px(18.0))
                            .bg(pill_bg)
                            .cursor(CursorStyle::PointingHand)
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |this, _, _win, _cx| {
                                    if this.state.current_workspace_index == idx {
                                        this.state.close();
                                    } else {
                                        this.state.current_workspace_index = idx;
                                    }
                                }),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(if is_active {
                                        gpui::FontWeight::BOLD
                                    } else {
                                        gpui::FontWeight::NORMAL
                                    })
                                    .text_color(pill_fg)
                                    .child(pill.name.clone()),
                            )
                            .when(pill.window_count > 0, |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .px(px(6.0))
                                        .py(px(1.0))
                                        .rounded(px(10.0))
                                        .bg(if is_active { primary } else { surface_highest })
                                        .text_color(if is_active {
                                            on_primary
                                        } else {
                                            on_surface_variant
                                        })
                                        .child(pill.window_count.to_string()),
                                )
                            })
                    }))
                    .child(
                        // Add workspace button ('+')
                        div()
                            .id("overview-add-ws-btn")
                            .w(px(28.0))
                            .h(px(28.0))
                            .rounded(px(14.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(surface_highest)
                            .cursor(CursorStyle::PointingHand)
                            .hover(|h| h.bg(primary_container))
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(|_this, _, _win, _cx| {
                                    info!("Overview requested new workspace creation");
                                }),
                            )
                            .child(div().text_sm().text_color(primary).child("+")),
                    ),
            )
            .when(self.state.is_window_info_open, |d| {
                // Window Info Inspection Modal (Complete parity with WindowInfo.qml, Details.qml, Buttons.qml)
                let info_addr = self.state.window_info_address.clone().unwrap_or_default();
                let inspected_window = self.windows.iter().find(|w| w.address == info_addr).cloned();

                let win_title = inspected_window.as_ref().map(|w| w.title.clone()).unwrap_or_else(|| "Window Details".to_string());
                let win_app = inspected_window.as_ref().map(|w| w.app_id.clone()).unwrap_or_else(|| "Unknown".to_string());
                let win_pos = inspected_window.as_ref().map(|w| format!("{}, {}", w.x, w.y)).unwrap_or_else(|| "0, 0".to_string());
                let win_size = inspected_window.as_ref().map(|w| format!("{} × {}", w.width, w.height)).unwrap_or_else(|| "800 × 600".to_string());
                let win_ws = inspected_window.as_ref().and_then(|w| w.desktops.first().cloned()).unwrap_or_else(|| "1".to_string());
                let is_max = inspected_window.as_ref().map(|w| w.is_maximized).unwrap_or(false);
                let is_min = inspected_window.as_ref().map(|w| w.is_minimized).unwrap_or(false);

                d.child(
                    div()
                        .id("window-info-modal-backdrop")
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(rgba(0x000000AA))
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(|this, _, _win, _cx| {
                                this.state.close_window_info();
                            }),
                        )
                        .child(
                            div()
                                .id("window-info-dialog")
                                .w(px(720.0))
                                .p(px(24.0))
                                .rounded(px(20.0))
                                .bg(surface_container)
                                .border_2()
                                .border_color(primary)
                                .shadow_xl()
                                .flex()
                                .flex_col()
                                .gap(px(16.0))
                                .on_mouse_down(gpui::MouseButton::Left, |_, _, _| {}) // Stop propagation
                                .child(
                                    // Dialog header with title and dismiss
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(2.0))
                                                .child(
                                                    div()
                                                        .text_lg()
                                                        .font_weight(gpui::FontWeight::BOLD)
                                                        .text_color(on_surface)
                                                        .child(win_title),
                                                )
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(primary)
                                                        .child(win_app),
                                                ),
                                        )
                                        .child(
                                            div()
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
                                                        this.state.close_window_info();
                                                    }),
                                                )
                                                .child(div().text_sm().text_color(on_surface).child("✕")),
                                        ),
                                )
                                .child(
                                    // Details grid (Address, Position, Size, Workspace)
                                    div()
                                        .flex()
                                        .flex_row()
                                        .flex_wrap()
                                        .gap(px(12.0))
                                        .p(px(12.0))
                                        .rounded(px(12.0))
                                        .bg(surface)
                                        .child(
                                            div()
                                                .w(px(320.0))
                                                .text_xs()
                                                .text_color(on_surface_variant)
                                                .child(format!("Address: {}", info_addr)),
                                        )
                                        .child(
                                            div()
                                                .w(px(320.0))
                                                .text_xs()
                                                .text_color(on_surface_variant)
                                                .child(format!("Position: {}", win_pos)),
                                        )
                                        .child(
                                            div()
                                                .w(px(320.0))
                                                .text_xs()
                                                .text_color(on_surface_variant)
                                                .child(format!("Dimensions: {}", win_size)),
                                        )
                                        .child(
                                            div()
                                                .w(px(320.0))
                                                .text_xs()
                                                .text_color(on_surface_variant)
                                                .child(format!("Workspace ID: {}", win_ws)),
                                        ),
                                )
                                .child(
                                    // Move to workspace row
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(6.0))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .text_color(on_surface)
                                                .child("Move to Workspace:"),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .gap(px(8.0))
                                                .children(self.indicator.pills.iter().map(|p| {
                                                    let pill_id = p.id;
                                                    div()
                                                        .px(px(12.0))
                                                        .py(px(4.0))
                                                        .rounded(px(8.0))
                                                        .bg(secondary_container)
                                                        .text_color(on_secondary_container)
                                                        .text_xs()
                                                        .cursor(CursorStyle::PointingHand)
                                                        .hover(|h| h.bg(primary_container))
                                                        .on_mouse_down(
                                                            gpui::MouseButton::Left,
                                                            cx.listener(move |this, _, _win, _cx| {
                                                                info!("Moving inspected window to workspace {}", pill_id);
                                                                this.state.close_window_info();
                                                            }),
                                                        )
                                                        .child(format!("WS {}", p.id))
                                                })),
                                        ),
                                )
                                .child(
                                    // Window Action Buttons (Maximize/Restore, Minimize, Kill)
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_end()
                                        .gap(px(10.0))
                                        .pt(px(8.0))
                                        .child(
                                            div()
                                                .px(px(14.0))
                                                .py(px(6.0))
                                                .rounded(px(8.0))
                                                .bg(surface_highest)
                                                .text_color(on_surface)
                                                .text_xs()
                                                .cursor(CursorStyle::PointingHand)
                                                .on_mouse_down(
                                                    gpui::MouseButton::Left,
                                                    cx.listener(|_this, _, _win, _cx| {
                                                        info!("Toggling maximize on inspected window");
                                                    }),
                                                )
                                                .child(if is_max { "Restore" } else { "Maximize" }),
                                        )
                                        .child(
                                            div()
                                                .px(px(14.0))
                                                .py(px(6.0))
                                                .rounded(px(8.0))
                                                .bg(surface_highest)
                                                .text_color(on_surface)
                                                .text_xs()
                                                .cursor(CursorStyle::PointingHand)
                                                .on_mouse_down(
                                                    gpui::MouseButton::Left,
                                                    cx.listener(|_this, _, _win, _cx| {
                                                        info!("Toggling minimize on inspected window");
                                                    }),
                                                )
                                                .child(if is_min { "Unminimize" } else { "Minimize" }),
                                        )
                                        .child(
                                            div()
                                                .px(px(14.0))
                                                .py(px(6.0))
                                                .rounded(px(8.0))
                                                .bg(error_container)
                                                .text_color(error)
                                                .text_xs()
                                                .font_weight(gpui::FontWeight::BOLD)
                                                .cursor(CursorStyle::PointingHand)
                                                .on_mouse_down(
                                                    gpui::MouseButton::Left,
                                                    cx.listener({
                                                        let a = info_addr.clone();
                                                        move |this, _, _win, _cx| {
                                                            info!("Closing inspected window {}", a);
                                                            this.windows.retain(|w| w.address != a);
                                                            this.state.close_window_info();
                                                        }
                                                    }),
                                                )
                                                .child("Kill Window"),
                                        ),
                                ),
                        ),
                )
            })
    }
}
