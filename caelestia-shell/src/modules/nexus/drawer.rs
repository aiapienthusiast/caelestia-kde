//! Nexus Drawer Component
//! Parity with `shell/modules/drawers/ContentWindow.qml` and `shell/modules/nexus/Nexus.qml`.
//! Renders the complete Caelestia Control Center with KWin frosted blur and live shell.json synchronization.

use super::nav_pane::NavPane;
use super::pages::*;
use super::state::NexusState;
use crate::domain::tokens::*;
use crate::infrastructure::config::ShellConfig;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, FocusHandle, KeyDownEvent,
    MouseButton, MouseDownEvent, Render, Styled, Window, WindowBackgroundAppearance,
};
use std::time::{Duration, Instant, SystemTime};

pub struct NexusDrawer {
    focus_handle: FocusHandle,
    pub state: NexusState,
    pub config: ShellConfig,
    pub colours: CaelestiaColours,
    pub is_open: bool,
    open_progress: f32,
    last_scheme_check: Instant,
    last_scheme_mtime: Option<SystemTime>,
}

impl NexusDrawer {
    pub fn new(cx: &mut gpui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let config = ShellConfig::load();
        let colours = CaelestiaColours::load();

        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let scheme_path = format!("{}/.local/state/caelestia/scheme.json", home);
        let last_scheme_mtime = std::fs::metadata(&scheme_path).ok().and_then(|m| m.modified().ok());

        let is_open = std::env::var("NEXUS_OPEN").map(|v| v != "0").unwrap_or(true);
        tracing::info!("NexusDrawer::new initialized: is_open={}", is_open);

        // 60 FPS animation loop
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_millis(16)).await;
                let _ = this.update(cx, |nexus, cx| {
                    nexus.tick_animation();
                    cx.notify();
                });
            }
        }).detach();

        Self {
            focus_handle,
            state: NexusState::new(),
            config,
            colours,
            is_open,
            open_progress: if is_open { 1.0 } else { 0.0 },
            last_scheme_check: Instant::now(),
            last_scheme_mtime,
        }
    }

    fn tick_animation(&mut self) {
        let target = if self.is_open { 1.0 } else { 0.0 };
        let diff = target - self.open_progress;
        self.open_progress += diff * 0.18;
        self.open_progress = self.open_progress.clamp(0.0, 1.0);

        // Hot-reload scheme.json when wallpaper/palette changes
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

    pub fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) {
        match event.keystroke.key.as_str() {
            "escape" => {
                cx.quit();
            }
            "tab" | "down" => {
                let next = (self.state.selected_index + 1) % super::state::CATEGORIES.len();
                self.state.select_category(next);
                cx.notify();
            }
            "up" => {
                let prev = if self.state.selected_index == 0 {
                    super::state::CATEGORIES.len() - 1
                } else {
                    self.state.selected_index - 1
                };
                self.state.select_category(prev);
                cx.notify();
            }
            _ => {}
        }
    }

    /// Mutates pitch black setting and saves to shell.json
    pub fn toggle_pitch_black(&mut self, cx: &mut gpui::Context<Self>) {
        let current = self.config.pitch_black();
        self.config.set_pitch_black(!current);
        let _ = self.config.save();
        cx.notify();
    }

    /// Mutates transparency setting and saves to shell.json
    pub fn toggle_transparency(&mut self, cx: &mut gpui::Context<Self>) {
        let current = self.config.transparency_enabled();
        self.config.set_transparency_enabled(!current);
        let _ = self.config.save();
        cx.notify();
    }

    /// Mutates blur setting and saves to shell.json
    pub fn toggle_blur(&mut self, cx: &mut gpui::Context<Self>) {
        let current = self.config.blur_enabled();
        self.config.set_blur_enabled(!current);
        let _ = self.config.save();
        cx.notify();
    }

    /// Sets base opacity (0.0 to 1.0) and saves to shell.json
    pub fn set_base_opacity(&mut self, val: f64, cx: &mut gpui::Context<Self>) {
        self.config.set_transparency_base(val);
        let _ = self.config.save();
        cx.notify();
    }

    /// Sets corner radius scale (0.5 to 2.0) and saves to shell.json
    pub fn set_corner_radius_scale(&mut self, scale: f64, cx: &mut gpui::Context<Self>) {
        self.config.set_rounding_scale(scale);
        let _ = self.config.save();
        cx.notify();
    }

    /// Sets border thickness (0 to 50) and saves to shell.json
    pub fn set_border_thickness(&mut self, thickness: f64, cx: &mut gpui::Context<Self>) {
        self.config.set_border_thickness(thickness);
        let _ = self.config.save();
        cx.notify();
    }

    /// Resets all appearance settings to default values and saves to shell.json
    pub fn reset_appearance_settings(&mut self, cx: &mut gpui::Context<Self>) {
        self.config.set_pitch_black(false);
        self.config.set_transparency_enabled(true);
        self.config.set_blur_enabled(true);
        self.config.set_transparency_base(0.7);
        self.config.set_transparency_layers(0.85);
        self.config.set_rounding_scale(1.0);
        self.config.set_border_thickness(1.0);
        let _ = self.config.save();
        cx.notify();
    }
}

impl Render for NexusDrawer {
    fn render(&mut self, window: &mut Window, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        // Request KWin native blur when open
        let target_appearance = if self.open_progress > 0.01 {
            WindowBackgroundAppearance::Blurred
        } else {
            WindowBackgroundAppearance::Transparent
        };
        window.set_background_appearance(target_appearance);

        let c = &self.colours;
        let selected_idx = self.state.selected_index;
        tracing::info!("NexusDrawer::render called! selected_idx={}, open_progress={}", selected_idx, self.open_progress);

        let current_page = match selected_idx {
            0 => AppearancePage::render(&self.config, c, cx),
            1 => DesktopPage::render(c),
            2 => PanelsPage::render(c),
            3 => NetworkPage::render(c),
            4 => BluetoothPage::render(c),
            5 => DisplayPage::render(c),
            6 => AudioPage::render(c),
            7 => PowerPage::render(c),
            8 => AppsPage::render(c),
            9 => ServicesPage::render(c),
            10 => AboutPage::render(c),
            _ => AppearancePage::render(&self.config, c, cx),
        };

        div()
            .id("nexus-drawer-backdrop")
            .size_full()
            .relative()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key_down))
            .bg(rgba(0x00000040))
            .flex()
            .items_center()
            .justify_center()
            // Dismiss on clicking backdrop outside the nexus dialog
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|nexus, _event: &MouseDownEvent, _window, cx| {
                    if nexus.is_open {
                        nexus.is_open = false;
                        cx.quit();
                    }
                }),
            )
            // Centered Nexus Dialog
            .child(
                div()
                    .id("nexus-dialog-surface")
                    .w(px(1080.0))
                    .h(px(640.0))
                    .rounded(px(ROUNDING_EXTRA_LARGE))
                    .bg(rgba(c.surface_container_low))
                    .border_1()
                    .border_color(rgba(c.outline_variant))
                    .shadow_xl()
                    .flex()
                    .overflow_hidden()
                    .cursor(CursorStyle::Arrow)
                    // Intercept click so clicking inside dialog does not dismiss
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    // Left NavPane with category selection callback
                    .child(NavPane::render(&self.state, c, cx))
                    // Right Content Page with live settings interactions
                    .child(current_page),
            )
    }
}
