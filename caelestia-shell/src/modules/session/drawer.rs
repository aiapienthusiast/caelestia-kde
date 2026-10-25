//! Session Drawer module providing edge-attached soft-body drawer with KWin blur.

use crate::domain::session::{SessionAction, SessionActionKind};
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::session::SessionService;
use gpui::{
    div, prelude::*, px, rgba,
    Context, CursorStyle, FocusHandle, FontWeight,
    InteractiveElement, IntoElement, KeyDownEvent, MouseButton,
    MouseDownEvent, MouseExitEvent, MouseMoveEvent, MouseUpEvent,
    Render, Styled, Window, WindowBackgroundAppearance,
};
use std::time::{Duration, Instant, SystemTime};

pub struct SessionDrawer {
    focus_handle: FocusHandle,
    actions: Vec<SessionAction>,
    focused_index: usize,
    pressed_index: Option<usize>,
    hovered_index: Option<usize>,
    mascot_pulse: f32,

    // Edge Hover & Blobs Physics State
    is_hovered: bool,
    open_progress: f32,
    velocity: f32,
    deform_x: f32,
    deform_y: f32,

    colours: CaelestiaColours,
    last_scheme_check: Instant,
    last_scheme_mtime: Option<SystemTime>,
}

impl SessionDrawer {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let actions = SessionAction::ALL.to_vec();

        // 60 FPS Animation & Soft-Body Physics Loop
        cx.spawn(async move |this, cx| {
            let mut step: f32 = 0.0;
            let dt = 0.016; // 16ms (~60 FPS)
            loop {
                cx.background_executor().timer(Duration::from_millis(16)).await;
                step += 0.05;
                let pulse = (step.sin() + 1.0) / 2.0;
                let _ = this.update(cx, |drawer, cx| {
                    drawer.mascot_pulse = pulse;
                    drawer.tick_physics(dt);
                    cx.notify();
                });
            }
        }).detach();

        let colours = CaelestiaColours::load();
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let scheme_path = format!("{}/.local/state/caelestia/scheme.json", home);
        let last_scheme_mtime = std::fs::metadata(&scheme_path).ok().and_then(|m| m.modified().ok());

        let is_open = std::env::var("DRAWER_OPEN").is_ok();

        Self {
            focus_handle,
            actions,
            focused_index: 0,
            pressed_index: None,
            hovered_index: None,
            mascot_pulse: 0.5,
            is_hovered: is_open,
            open_progress: if is_open { 1.0 } else { 0.0 },
            velocity: 0.0,
            deform_x: 1.0,
            deform_y: 1.0,
            colours,
            last_scheme_check: Instant::now(),
            last_scheme_mtime,
        }
    }

    /// Spring & inertia physics directly implementing Caelestia Blobs soft-body dynamics
    fn tick_physics(&mut self, dt: f32) {
        let target = if self.is_hovered || std::env::var("DRAWER_OPEN").is_ok() { 1.0 } else { 0.0 };
        let diff = target - self.open_progress;

        let stiffness = 170.0;
        let damping = 17.0;
        let force = diff * stiffness;
        self.velocity += (force - self.velocity * damping) * dt;
        self.open_progress += self.velocity * dt;
        self.open_progress = self.open_progress.clamp(0.0, 1.0);

        // Organic stretch from spring velocity
        let speed = self.velocity.abs();
        let target_stretch = 1.0 + (speed * 0.14).min(0.35);
        let target_compress = 1.0 / target_stretch;

        self.deform_x += (target_stretch - self.deform_x) * (dt * 14.0).min(1.0);
        self.deform_y += (target_compress - self.deform_y) * (dt * 14.0).min(1.0);

        // Dynamic hot-reloading of scheme.json
        if self.last_scheme_check.elapsed() > Duration::from_millis(500) {
            self.last_scheme_check = Instant::now();
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let scheme_path = format!("{}/.local/state/caelestia/scheme.json", home);
            if let Ok(metadata) = std::fs::metadata(&scheme_path) {
                if let Ok(mtime) = metadata.modified() {
                    if self.last_scheme_mtime != Some(mtime) {
                        self.last_scheme_mtime = Some(mtime);
                        self.colours = CaelestiaColours::load();
                    }
                }
            }
        }
    }

    fn execute_action(&self, index: usize) {
        if let Some(action) = self.actions.get(index) {
            SessionService::execute(action.kind);
        }
    }

    fn focus_next(&mut self, cx: &mut Context<Self>) {
        if !self.actions.is_empty() {
            self.focused_index = (self.focused_index + 1) % self.actions.len();
            cx.notify();
        }
    }

    fn focus_prev(&mut self, cx: &mut Context<Self>) {
        if !self.actions.is_empty() {
            if self.focused_index == 0 {
                self.focused_index = self.actions.len() - 1;
            } else {
                self.focused_index -= 1;
            }
            cx.notify();
        }
    }

    pub fn handle_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let key = event.keystroke.key.as_str();
        let shift = event.keystroke.modifiers.shift;
        let ctrl = event.keystroke.modifiers.control;

        match key {
            "down" | "tab" if !shift => self.focus_next(cx),
            "up" | "tab" if shift => self.focus_prev(cx),
            "j" | "n" if ctrl => self.focus_next(cx),
            "k" | "p" if ctrl => self.focus_prev(cx),
            "enter" | "space" => {
                self.execute_action(self.focused_index);
            }
            "escape" => {
                if self.is_hovered {
                    self.is_hovered = false;
                    cx.notify();
                }
            }
            _ => {}
        }
    }
}

impl Render for SessionDrawer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let target_appearance = if self.open_progress > 0.001 {
            WindowBackgroundAppearance::Blurred
        } else {
            WindowBackgroundAppearance::Transparent
        };
        window.set_background_appearance(target_appearance);

        let focused_idx = self.focused_index;
        let pressed_idx = self.pressed_index;
        let hovered_idx = self.hovered_index;
        let mascot_scale = 0.95 + 0.1 * self.mascot_pulse;

        let drawer_width = 116.0;
        let slide_offset = px((1.0 - self.open_progress) * drawer_width);

        let base_curve = 28.0;
        let speed = self.velocity.abs();
        let velocity_bulge = (speed * 4.5).min(20.0);
        let dynamic_curve = (base_curve + velocity_bulge) * self.open_progress;
        let left_curve_radius = px(dynamic_curve);

        let stretch_x = px(((self.deform_x - 1.0) * 16.0).max(0.0));
        let c = &self.colours;

        let render_button = |action: &SessionAction, idx: usize, is_focused: bool, is_pressed: bool, is_hovered: bool| {
            let radius = if is_pressed {
                px(ROUNDING_SMALL)
            } else if is_focused {
                px(ROUNDING_LARGE)
            } else {
                px(ROUNDING_NORMAL)
            };

            let bg_color = if is_focused {
                rgba(c.secondary_container)
            } else if is_hovered {
                rgba(c.surface_container_high)
            } else {
                rgba(c.surface_container)
            };

            let text_color = if is_focused {
                rgba(c.on_secondary_container)
            } else {
                rgba(c.on_surface)
            };

            let icon_char = match action.kind {
                SessionActionKind::Logout => "󰍃",
                SessionActionKind::Shutdown => "⏻",
                SessionActionKind::Hibernate => "󰒲",
                SessionActionKind::Reboot => "󰑓",
                SessionActionKind::Lock => "🔒",
                SessionActionKind::Suspend => "💤",
            };

            div()
                .id(format!("action-btn-{}", idx))
                .w(px(BUTTON_WIDTH))
                .h(px(BUTTON_HEIGHT))
                .bg(bg_color)
                .rounded(radius)
                .border_1()
                .border_color(if is_focused { rgba(c.primary) } else { rgba(c.outline_variant) })
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(SPACING_SMALL / 2.0))
                .cursor(CursorStyle::PointingHand)
                .on_hover(cx.listener(move |drawer, hovered: &bool, _window, cx| {
                    drawer.hovered_index = if *hovered { Some(idx) } else { None };
                    if *hovered {
                        drawer.focused_index = idx;
                    }
                    cx.notify();
                }))
                .on_mouse_down(MouseButton::Left, cx.listener(move |drawer, _event: &MouseDownEvent, _window, cx| {
                    drawer.pressed_index = Some(idx);
                    cx.notify();
                }))
                .on_mouse_up(MouseButton::Left, cx.listener(move |drawer, _event: &MouseUpEvent, _window, cx| {
                    if drawer.pressed_index == Some(idx) {
                        drawer.pressed_index = None;
                        drawer.execute_action(idx);
                        cx.notify();
                    }
                }))
                .child(
                    div()
                        .text_size(px(24.0))
                        .text_color(text_color)
                        .font_weight(FontWeight::BOLD)
                        .child(icon_char)
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(text_color)
                        .font_weight(if is_focused { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                        .child(action.title)
                )
        };

        div()
            .id("edge-drawer-container")
            .size_full()
            .relative()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key_down))
            .on_mouse_move(cx.listener(|drawer, event: &MouseMoveEvent, _window, cx| {
                let x = event.position.x;
                if drawer.open_progress > 0.05 || x >= px(88.0) {
                    if !drawer.is_hovered {
                        drawer.is_hovered = true;
                        cx.notify();
                    }
                }
            }))
            .on_mouse_exit(cx.listener(|drawer, _event: &MouseExitEvent, _window, cx| {
                if std::env::var("DRAWER_OPEN").is_err() && drawer.is_hovered {
                    drawer.is_hovered = false;
                    cx.notify();
                }
            }))
            .child(
                // Sliding & Curving Blob Drawer Body
                div()
                    .id("sliding-drawer-body")
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .w(px(drawer_width))
                    .left(slide_offset - stretch_x)
                    .opacity(self.open_progress.clamp(0.0, 1.0))
                    .when(self.open_progress <= 0.001, |el| el.invisible())
                    .p(px(PADDING_NORMAL))
                    .pr(px(16.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(SPACING_NORMAL))
                    // Soft-Body Blob Background
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .bg(rgba(c.surface))
                            .rounded_tl(left_curve_radius)
                            .rounded_bl(left_curve_radius)
                            .rounded_tr(px(0.0))
                            .rounded_br(px(0.0))
                            .border_1()
                            .border_r_0()
                            .border_color(rgba(c.outline_variant))
                            .shadow_lg()
                    )
                    // Content Layer
                    .child(
                        div()
                            .opacity((self.open_progress * 1.2).min(1.0))
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(SPACING_NORMAL))
                            .child(render_button(&self.actions[0], 0, focused_idx == 0, pressed_idx == Some(0), hovered_idx == Some(0)))
                            .child(render_button(&self.actions[1], 1, focused_idx == 1, pressed_idx == Some(1), hovered_idx == Some(1)))
                            // Mascot Centerpiece
                            .child(
                                div()
                                    .w(px(BUTTON_WIDTH))
                                    .h(px(BUTTON_WIDTH))
                                    .rounded(px(ROUNDING_NORMAL))
                                    .bg(rgba(c.mascot_bg))
                                    .border_1()
                                    .border_color(rgba(c.outline_variant))
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_size(px(32.0 * mascot_scale))
                                            .child("🦖")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgba(c.primary))
                                            .child("Caelestia")
                                    )
                            )
                            .child(render_button(&self.actions[2], 2, focused_idx == 2, pressed_idx == Some(2), hovered_idx == Some(2)))
                            .child(render_button(&self.actions[3], 3, focused_idx == 3, pressed_idx == Some(3), hovered_idx == Some(3)))
                    )
            )
            .child(
                // Persistent Edge Hitbox on Right Screen Margin
                div()
                    .id("right-edge-hover-trigger")
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .right_0()
                    .w(px(28.0))
                    .cursor(CursorStyle::PointingHand)
                    .on_hover(cx.listener(|drawer, hovered: &bool, _window, cx| {
                        if *hovered && !drawer.is_hovered {
                            drawer.is_hovered = true;
                            cx.notify();
                        }
                    }))
                    .child(
                        div()
                            .absolute()
                            .top(px(220.0))
                            .right_0()
                            .w(px(3.0))
                            .h(px(60.0))
                            .rounded_l(px(2.0))
                            .bg(rgba(c.primary))
                            .opacity((1.0 - self.open_progress * 2.0).clamp(0.0, 0.7))
                    )
            )
    }
}
