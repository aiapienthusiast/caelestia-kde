//! Background view component
//! Complete parity with `shell/modules/background/Background.qml`, `DesktopClock.qml`,
//! `DesktopContextMenu.qml`, and `Wallpaper.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use super::config::DesktopClockLayout;
use super::state::BackgroundState;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, FocusHandle, IntoElement, MouseButton, MouseDownEvent,
    Render, Styled, Window,
};

pub struct BackgroundView {
    pub state: BackgroundState,
    pub colours: CaelestiaColours,
    pub focus_handle: FocusHandle,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl BackgroundView {
    pub fn new(cx: &mut gpui::Context<Self>, colours: CaelestiaColours) -> Self {
        Self {
            state: BackgroundState::default(),
            colours,
            focus_handle: cx.focus_handle(),
            screen_width: 1920.0,
            screen_height: 1080.0,
        }
    }

    pub fn with_state(
        cx: &mut gpui::Context<Self>,
        colours: CaelestiaColours,
        state: BackgroundState,
    ) -> Self {
        Self {
            state,
            colours,
            focus_handle: cx.focus_handle(),
            screen_width: 1920.0,
            screen_height: 1080.0,
        }
    }

    /// Renders the clock widget according to its layout mode
    fn render_clock(&self) -> impl IntoElement {
        let clock = self.state.clock.format_now();
        let primary = rgba(self.colours.primary);
        let secondary = rgba(self.colours.secondary);
        let on_surface_variant = rgba(self.colours.on_surface_variant);

        match clock.layout {
            DesktopClockLayout::Wide => {
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .p(px(24.0))
                    .rounded(px(24.0))
                    .bg(rgba(0x00000066))
                    .border_1()
                    .border_color(rgba(0xffffff22))
                    .child(
                        div()
                            .text_size(px(64.0))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(primary)
                            .child(clock.primary_time),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .text_size(px(18.0))
                            .text_color(on_surface_variant)
                            .child(clock.day_of_week)
                            .child("•")
                            .child(clock.date),
                    )
            }
            DesktopClockLayout::Compact => {
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.0))
                    .p(px(16.0))
                    .rounded(px(16.0))
                    .bg(rgba(0x00000066))
                    .border_1()
                    .border_color(rgba(0xffffff22))
                    .child(
                        div()
                            .text_size(px(36.0))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(primary)
                            .child(clock.primary_time),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .text_size(px(13.0))
                            .text_color(on_surface_variant)
                            .child(clock.day_of_week)
                            .child(clock.date),
                    )
            }
            DesktopClockLayout::Stacked => {
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .p(px(20.0))
                    .rounded(px(20.0))
                    .bg(rgba(0x00000066))
                    .border_1()
                    .border_color(rgba(0xffffff22))
                    .child(
                        div()
                            .text_size(px(52.0))
                            .font_weight(gpui::FontWeight::EXTRA_BOLD)
                            .text_color(primary)
                            .child(clock.primary_time),
                    )
                    .when_some(clock.secondary_time, |d, sec| {
                        d.child(
                            div()
                                .text_size(px(52.0))
                                .font_weight(gpui::FontWeight::EXTRA_BOLD)
                                .text_color(secondary)
                                .child(sec),
                        )
                    })
                    .child(
                        div()
                            .mt(px(8.0))
                            .flex()
                            .flex_col()
                            .items_center()
                            .text_size(px(14.0))
                            .text_color(on_surface_variant)
                            .child(clock.day_of_week)
                            .child(clock.date),
                    )
            }
        }
    }

    /// Renders the floating context menu if active
    fn render_context_menu(&self, cx: &mut gpui::Context<Self>) -> Option<impl IntoElement> {
        if !self.state.context_menu.is_open {
            return None;
        }

        let pos_x = self.state.context_menu.position_x;
        let pos_y = self.state.context_menu.position_y;
        let items = self.state.context_menu.items.clone();
        let surface_container_high = rgba(self.colours.surface_container_high);
        let outline_variant = rgba(self.colours.outline_variant);
        let on_surface = rgba(self.colours.on_surface);

        Some(
            div()
                .absolute()
                .left(px(pos_x))
                .top(px(pos_y))
                .w(px(220.0))
                .p(px(8.0))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .bg(surface_container_high)
                .rounded(px(16.0))
                .border_1()
                .border_color(outline_variant)
                .shadow_lg()
                .children(items.into_iter().map(|item| {
                    let label = item.label();
                    let icon_name = item.icon();
                    let action = item;
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .px(px(12.0))
                        .py(px(8.0))
                        .rounded(px(8.0))
                        .cursor(CursorStyle::PointingHand)
                        .hover(|d| d.bg(rgba(0xffffff15)))
                        .text_size(px(13.0))
                        .text_color(on_surface)
                        .child(MaterialIcon::with_size(
                            icon_name,
                            IconSize::Small,
                            self.colours.on_surface_variant,
                        ))
                        .child(label)
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _win, cx| {
                            this.state.execute_context_menu_action(action);
                            cx.notify();
                        }))
                })),
        )
    }
}

impl Render for BackgroundView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let wallpaper_text = self
            .state
            .wallpaper
            .current_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|f| f.to_str())
            .unwrap_or("Default Wallpaper");

        let surface_lowest = rgba(self.colours.surface_container_lowest);
        let on_surface = rgba(self.colours.on_surface);

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .relative()
            .bg(surface_lowest)
            .on_mouse_down(MouseButton::Right, cx.listener(|this, event: &MouseDownEvent, _win, cx| {
                let x: f32 = event.position.x.into();
                let y: f32 = event.position.y.into();
                this.state.on_empty_desktop_right_click(x, y, "primary");
                cx.notify();
            }))
            .on_mouse_down(MouseButton::Middle, cx.listener(|this, _, _win, cx| {
                this.state.on_empty_desktop_middle_click();
                cx.notify();
            }))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _win, cx| {
                if this.state.context_menu.is_open {
                    this.state.context_menu.close();
                    cx.notify();
                }
            }))
            // Wallpaper placeholder/image surface
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .opacity(0.15)
                    .text_size(px(24.0))
                    .text_color(on_surface)
                    .child(wallpaper_text.to_string()),
            )
            // Desktop clock center-top
            .when(self.state.config.desktop_clock_enabled, |d| {
                d.child(
                    div()
                        .absolute()
                        .top(px(120.0))
                        .left(px(self.screen_width / 2.0 - 180.0))
                        .child(self.render_clock()),
                )
            })
            // Context menu overlay
            .children(self.render_context_menu(cx))
    }
}
