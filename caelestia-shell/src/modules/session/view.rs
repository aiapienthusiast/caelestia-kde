//! Session Drawer View Component
//! 100% parity with `shell/modules/session/Content.qml` and `Wrapper.qml`.

use super::config::SessionConfig;
use super::state::SessionState;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::{StyledText, StyledTextExt};
use crate::domain::session::SessionActionKind;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::session::SessionService;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, FocusHandle, IntoElement, KeyDownEvent,
    MouseButton, Render, Styled, Window,
};

pub struct SessionView {
    pub state: SessionState,
    pub config: SessionConfig,
    pub colours: CaelestiaColours,
    pub focus_handle: FocusHandle,
    pub pressed_index: Option<usize>,
    pub mascot_pulse: f32,
}

impl SessionView {
    pub fn new(cx: &mut gpui::Context<Self>) -> Self {
        let config = SessionConfig::default();
        let state = SessionState::new(&config);
        let colours = CaelestiaColours::load();

        Self {
            state,
            config,
            colours,
            focus_handle: cx.focus_handle(),
            pressed_index: None,
            mascot_pulse: 0.5,
        }
    }

    pub fn with_colours(cx: &mut gpui::Context<Self>, colours: CaelestiaColours) -> Self {
        let config = SessionConfig::default();
        let state = SessionState::new(&config);

        Self {
            state,
            config,
            colours,
            focus_handle: cx.focus_handle(),
            pressed_index: None,
            mascot_pulse: 0.5,
        }
    }

    /// Handle key down event matching `Content.qml`
    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut gpui::Context<Self>) {
        let key = event.keystroke.key.to_lowercase();
        let ctrl = event.keystroke.modifiers.control;
        let shift = event.keystroke.modifiers.shift;

        if let Some(action) = self.state.handle_key(&key, ctrl, shift) {
            self.execute_action(action);
        }
        cx.notify();
    }

    /// Executes session action
    pub fn execute_action(&mut self, kind: SessionActionKind) {
        SessionService::execute(kind);
        self.state.close();
    }

    /// Renders an individual SessionButton matching `Content.qml`
    fn render_button(
        &self,
        index: usize,
        title: &'static str,
        _icon_name: &'static str,
        kind: SessionActionKind,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let is_focused = self.state.focused_index == index;
        let is_pressed = self.pressed_index == Some(index);

        let bg_color = if is_focused {
            rgba(self.colours.secondary_container)
        } else {
            rgba(self.colours.surface_container)
        };

        let text_color = if is_focused {
            rgba(self.colours.on_secondary_container)
        } else {
            rgba(self.colours.on_surface)
        };

        let radius = if is_pressed {
            ROUNDING_MEDIUM
        } else if is_focused {
            ROUNDING_EXTRA_LARGE
        } else {
            ROUNDING_LARGE_INCREASED
        };

        div()
            .id(index)
            .w(px(BUTTON_WIDTH))
            .h(px(BUTTON_HEIGHT))
            .p(px(PADDING_SMALL))
            .rounded(px(radius))
            .bg(bg_color)
            .border_1()
            .border_color(if is_focused {
                rgba(self.colours.primary)
            } else {
                rgba(0x00000000)
            })
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(SPACING_EXTRA_SMALL))
            .cursor(CursorStyle::PointingHand)
            .hover(|d| d.bg(rgba(self.colours.surface_container_highest)))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _win, cx| {
                this.state.set_focused_index(index);
                this.pressed_index = Some(index);
                cx.notify();
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(move |this, _, _win, cx| {
                this.pressed_index = None;
                this.execute_action(kind);
                cx.notify();
            }))
            .child(
                MaterialIcon::new(match kind {
                    SessionActionKind::Logout => "logout",
                    SessionActionKind::Shutdown => "power_settings_new",
                    SessionActionKind::Hibernate => "bedtime",
                    SessionActionKind::Reboot => "restart_alt",
                    _ => "more_horiz",
                })
                .text_size(px(ICON_SIZE_MEDIUM * 1.2))
                .child("")
                .color(if is_focused {
                    self.colours.on_secondary_container
                } else {
                    self.colours.on_surface
                }),
            )
            .child(
                StyledText::new(title)
                    .color(if is_focused {
                        self.colours.on_secondary_container
                    } else {
                        self.colours.on_surface
                    })
                    .font_size(FONT_SIZE_SMALL),
            )
    }

    /// Center Dinosaur / Mascot Emblem matching `Content.qml`
    fn render_mascot_emblem(&self) -> impl IntoElement {
        div()
            .w(px(BUTTON_WIDTH))
            .h(px(BUTTON_WIDTH))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(48.0))
                    .text_color(rgba(self.colours.on_surface_variant))
                    .opacity(0.85)
                    .child("🦖"),
            )
    }
}

impl Render for SessionView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let actions = self.state.actions.clone();

        let logout_action = actions.iter().find(|a| a.kind == SessionActionKind::Logout);
        let shutdown_action = actions.iter().find(|a| a.kind == SessionActionKind::Shutdown);
        let hibernate_action = actions.iter().find(|a| a.kind == SessionActionKind::Hibernate);
        let reboot_action = actions.iter().find(|a| a.kind == SessionActionKind::Reboot);

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _win, cx| {
                this.handle_key_down(event, cx);
            }))
            .p(px(PADDING_LARGE))
            .flex()
            .flex_col()
            .gap(px(SPACING_LARGE))
            .bg(rgba(self.colours.surface_container_low))
            .rounded(px(ROUNDING_EXTRA_LARGE))
            .border_1()
            .border_color(rgba(self.colours.outline_variant))
            .shadow_xl()
            // 1. Logout button
            .when_some(logout_action, |d, a| {
                let idx = self.state.actions.iter().position(|x| x.kind == a.kind).unwrap_or(0);
                d.child(self.render_button(idx, a.title, a.icon_name, a.kind, cx))
            })
            // 2. Shutdown button
            .when_some(shutdown_action, |d, a| {
                let idx = self.state.actions.iter().position(|x| x.kind == a.kind).unwrap_or(1);
                d.child(self.render_button(idx, a.title, a.icon_name, a.kind, cx))
            })
            // 3. Center Mascot Emblem (Dino)
            .child(self.render_mascot_emblem())
            // 4. Hibernate button
            .when_some(hibernate_action, |d, a| {
                let idx = self.state.actions.iter().position(|x| x.kind == a.kind).unwrap_or(2);
                d.child(self.render_button(idx, a.title, a.icon_name, a.kind, cx))
            })
            // 5. Restart button
            .when_some(reboot_action, |d, a| {
                let idx = self.state.actions.iter().position(|x| x.kind == a.kind).unwrap_or(3);
                d.child(self.render_button(idx, a.title, a.icon_name, a.kind, cx))
            })
    }
}

/// Backwards compatibility alias
pub type SessionDrawer = SessionView;
