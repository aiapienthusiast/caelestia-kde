//! Screenshot Overlay View Component
//! Complete parity with `shell/modules/screenshot/regionSelector/RegionSelection.qml`,
//! `CursorGuide.qml`, and `OptionsToolbar.qml`.

use super::actions::SnipAction;
use super::config::ScreenshotConfig;
use super::state::{ScreenshotState, SelectionPhase};
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::kwin::WindowItem;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, FocusHandle, IntoElement, KeyDownEvent,
    Render, Styled, Window,
};

pub struct ScreenshotOverlayView {
    pub state: ScreenshotState,
    pub config: ScreenshotConfig,
    pub colours: CaelestiaColours,
    pub windows: Vec<WindowItem>,
    pub screen_width: f32,
    pub screen_height: f32,
    pub focus_handle: FocusHandle,
}

impl ScreenshotOverlayView {
    pub fn new(
        cx: &mut gpui::Context<Self>,
        colours: CaelestiaColours,
        screen_width: f32,
        screen_height: f32,
    ) -> Self {
        Self {
            state: ScreenshotState::new(),
            config: ScreenshotConfig::default(),
            colours,
            windows: Vec::new(),
            screen_width,
            screen_height,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Update windows list for window snapping
    pub fn update_windows(&mut self, windows: Vec<WindowItem>) {
        self.windows = windows;
    }

    /// Handle keyboard navigation
    pub fn handle_key_down(&mut self, event: &KeyDownEvent) {
        if !self.state.is_active {
            return;
        }

        match event.keystroke.key.as_str() {
            "escape" => {
                self.state.dismiss();
            }
            "enter" | "return" => {
                if self.state.phase == SelectionPhase::Post {
                    self.state.execute_action(&self.config);
                }
            }
            _ => {}
        }
    }
}

impl Render for ScreenshotOverlayView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        if !self.state.is_active {
            return div().id("screenshot-hidden");
        }

        let bg_scrim = rgba(0x00000088);
        let secondary = rgba(self.colours.secondary);
        let surface_container = rgba(self.colours.surface_container);
        let surface_highest = rgba(self.colours.surface_container_highest);
        let primary = rgba(self.colours.primary);
        let on_primary = rgba(self.colours.on_primary);
        let on_surface = rgba(self.colours.on_surface);
        let on_surface_variant = rgba(self.colours.on_surface_variant);
        let selection_fill = rgba(0xFFFFFF33);

        let bounds = self.state.current_bounds();
        let (cx_pos, cy_pos) = self.state.cursor_pos;
        let is_post = self.state.phase == SelectionPhase::Post;

        div()
            .id("screenshot-overlay-window")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _win, _cx| {
                this.handle_key_down(event);
            }))
            .size_full()
            .bg(bg_scrim)
            .relative()
            .cursor(CursorStyle::Crosshair)
            // Mouse drag handlers
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|this, _, _win, _cx| {
                    if this.state.phase == SelectionPhase::Select {
                        let (x, y) = this.state.cursor_pos;
                        this.state.start_drag(x, y);
                    }
                }),
            )
            .on_mouse_up(
                gpui::MouseButton::Left,
                cx.listener(|this, _, _win, _cx| {
                    if this.state.is_dragging {
                        this.state.finish_drag();
                    }
                }),
            )
            // Cursor guide crosshairs (during selection)
            .when(!is_post, |d| {
                d.child(
                    div()
                        .id("cursor-guide-x")
                        .absolute()
                        .top(px(cy_pos))
                        .left_0()
                        .w_full()
                        .h(px(1.0))
                        .bg(rgba(0xFFFFFF44)),
                )
                .child(
                    div()
                        .id("cursor-guide-y")
                        .absolute()
                        .left(px(cx_pos))
                        .top_0()
                        .h_full()
                        .w(px(1.0))
                        .bg(rgba(0xFFFFFF44)),
                )
            })
            // Render selected region box
            .when_some(bounds, |d, b| {
                let rect_x = b.x;
                let rect_y = b.y;
                let rect_w = b.width;
                let rect_h = b.height;

                d.child(
                    div()
                        .id("selection-region-rect")
                        .absolute()
                        .left(px(rect_x))
                        .top(px(rect_y))
                        .w(px(rect_w))
                        .h(px(rect_h))
                        .bg(selection_fill)
                        .border_2()
                        .border_color(secondary)
                        .child(
                            // Dimensions badge tooltip
                            div()
                                .absolute()
                                .bottom(px(4.0))
                                .right(px(4.0))
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(surface_container)
                                .text_xs()
                                .text_color(on_surface)
                                .child(format!("{} × {}", rect_w.round() as u32, rect_h.round() as u32)),
                        ),
                )
                // Floating options toolbar upon finishing drag
                .when(is_post, |post_d| {
                    let tb_y = if rect_y + rect_h + 50.0 < self.screen_height {
                        rect_y + rect_h + 8.0
                    } else {
                        (rect_y - 48.0).max(8.0)
                    };
                    let tb_x = rect_x.clamp(8.0, (self.screen_width - 360.0).max(8.0));

                    post_d.child(
                        div()
                            .id("snip-action-toolbar")
                            .absolute()
                            .left(px(tb_x))
                            .top(px(tb_y))
                            .h(px(42.0))
                            .px(px(8.0))
                            .rounded(px(21.0))
                            .bg(surface_container)
                            .border_1()
                            .border_color(surface_highest)
                            .shadow_xl()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .on_mouse_down(gpui::MouseButton::Left, |_, _, _| {}) // Stop propagation
                            // Copy button
                            .child(
                                div()
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(14.0))
                                    .bg(primary)
                                    .text_color(on_primary)
                                    .text_xs()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(|this, _, _win, _cx| {
                                            this.state.set_action(SnipAction::Copy);
                                            this.state.execute_action(&this.config);
                                        }),
                                    )
                                    .child("📋 Copy"),
                            )
                            // Edit in Swappy
                            .child(
                                div()
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(14.0))
                                    .bg(surface_highest)
                                    .text_color(on_surface)
                                    .text_xs()
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(|this, _, _win, _cx| {
                                            this.state.set_action(SnipAction::Edit);
                                            this.state.execute_action(&this.config);
                                        }),
                                    )
                                    .child("✏️ Edit"),
                            )
                            // OCR Text recognition
                            .child(
                                div()
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(14.0))
                                    .bg(surface_highest)
                                    .text_color(on_surface)
                                    .text_xs()
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(|this, _, _win, _cx| {
                                            this.state.set_action(SnipAction::CharRecognition);
                                            this.state.execute_action(&this.config);
                                        }),
                                    )
                                    .child("🔤 OCR"),
                            )
                            // Cancel
                            .child(
                                div()
                                    .w(px(24.0))
                                    .h(px(24.0))
                                    .rounded(px(12.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(surface_highest)
                                    .text_color(on_surface_variant)
                                    .text_xs()
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(|this, _, _win, _cx| {
                                            this.state.dismiss();
                                        }),
                                    )
                                    .child("✕"),
                            ),
                    )
                })
            })
    }
}
