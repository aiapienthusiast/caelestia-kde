//! Nexus Standard Window Component
//!
//! Standard desktop window for Caelestia Control Center & Settings.
//! First-class, resizable, movable, managed Wayland/X11 application window
//! with server or client-side decorations and frosted Material 3 surface styling.

use super::action::NexusAction;
use super::nav_pane::NavPane;
use super::pages::*;
use super::state::NexusState;
use crate::components::icon::{IconSize, MaterialIcon};
use crate::domain::tokens::*;
use crate::infrastructure::config::ShellConfig;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, FocusHandle, KeyDownEvent,
    MouseButton, Render, Styled, Window, WindowBackgroundAppearance,
};
use std::time::{Duration, Instant, SystemTime};

/// Primary Caelestia Settings & Control Center Application Window
pub struct NexusWindow {
    focus_handle: FocusHandle,
    pub state: NexusState,
    pub config: ShellConfig,
    pub colours: CaelestiaColours,
    pub is_maximized: bool,
    last_scheme_check: Instant,
    last_scheme_mtime: Option<SystemTime>,
}

impl NexusWindow {
    pub fn new(cx: &mut gpui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let config = ShellConfig::load();
        let colours = CaelestiaColours::load();

        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let scheme_path = format!("{}/.local/state/caelestia/scheme.json", home);
        let last_scheme_mtime = std::fs::metadata(&scheme_path).ok().and_then(|m| m.modified().ok());

        tracing::info!("NexusWindow::new initialized standard application window");

        // Background scheme synchronization timer (zero-idle repaints)
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_millis(500)).await;
                let _ = this.update(cx, |nexus, cx| {
                    if nexus.check_scheme_reload() {
                        cx.notify();
                    }
                });
            }
        }).detach();

        Self {
            focus_handle,
            state: NexusState::new(),
            config,
            colours,
            is_maximized: false,
            last_scheme_check: Instant::now(),
            last_scheme_mtime,
        }
    }

    /// Hot-reloads Material 3 scheme palette when wallpaper changes. Returns true if reloaded.
    pub fn check_scheme_reload(&mut self) -> bool {
        if self.last_scheme_check.elapsed() > Duration::from_millis(500) {
            self.last_scheme_check = Instant::now();
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let scheme_path = format!("{}/.local/state/caelestia/scheme.json", home);
            if let Ok(metadata) = std::fs::metadata(&scheme_path) {
                if let Ok(mtime) = metadata.modified() {
                    if self.last_scheme_mtime != Some(mtime) {
                        self.last_scheme_mtime = Some(mtime);
                        self.colours = CaelestiaColours::load();
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Centralized high-performance action dispatcher
    pub fn dispatch(&mut self, action: NexusAction, cx: &mut gpui::Context<Self>) {
        match action {
            NexusAction::TogglePitchBlack => {
                let current = self.config.pitch_black();
                self.config.set_pitch_black(!current);
                let _ = self.config.save();
                cx.notify();
            }
            NexusAction::ToggleTransparency => {
                let current = self.config.transparency_enabled();
                self.config.set_transparency_enabled(!current);
                let _ = self.config.save();
                cx.notify();
            }
            NexusAction::ToggleBlur => {
                let current = self.config.blur_enabled();
                self.config.set_blur_enabled(!current);
                let _ = self.config.save();
                cx.notify();
            }
            NexusAction::SetBaseOpacity(val) => {
                self.config.set_transparency_base(val);
                let _ = self.config.save();
                cx.notify();
            }
            NexusAction::SetCornerRadiusScale(scale) => {
                self.config.set_rounding_scale(scale);
                let _ = self.config.save();
                cx.notify();
            }
            NexusAction::SetBorderThickness(thickness) => {
                self.config.set_border_thickness(thickness);
                let _ = self.config.save();
                cx.notify();
            }
            NexusAction::ResetAppearanceSettings => {
                self.reset_appearance_settings(cx);
            }
            NexusAction::ToggleKrohnkiteTiling(_) => {
                cx.notify();
            }
            NexusAction::ToggleDesktopIcons(_) => {
                cx.notify();
            }
            NexusAction::ToggleContextMenu(_) => {
                cx.notify();
            }
            NexusAction::ToggleWorkspacesIndicator(_) => {
                cx.notify();
            }
            NexusAction::ToggleTaskbar(_)
            | NexusAction::ToggleDashboard(_)
            | NexusAction::ToggleLauncher(_)
            | NexusAction::ToggleDockMode(_)
            | NexusAction::ToggleSidebarToggles(_)
            | NexusAction::ToggleWifi(_)
            | NexusAction::ToggleHotspot(_)
            | NexusAction::ToggleVpn(_)
            | NexusAction::ConnectWifi(_)
            | NexusAction::ToggleBluetooth(_)
            | NexusAction::ConnectBtDevice(_)
            | NexusAction::SetDisplayScaling(_)
            | NexusAction::ToggleNightLight(_)
            | NexusAction::SetDisplayRefreshRate(_)
            | NexusAction::SetMasterVolume(_)
            | NexusAction::ToggleMute(_)
            | NexusAction::SelectAudioSink(_)
            | NexusAction::SetPowerProfile(_)
            | NexusAction::ToggleBatteryConservation(_)
            | NexusAction::UninstallApp(_)
            | NexusAction::ToggleAutostartApp(_, _)
            | NexusAction::ToggleGamemode(_)
            | NexusAction::ToggleQuickShare(_)
            | NexusAction::ToggleFocusMode(_)
            | NexusAction::ToggleIdleInhibitor(_) => {
                cx.notify();
            }
            NexusAction::SelectCategory(idx) => {
                self.state.select_category(idx);
                cx.notify();
            }
            NexusAction::SetSearchQuery(q) => {
                self.state.set_search_query(&q);
                cx.notify();
            }
            NexusAction::ClearSearch => {
                self.state.clear_search();
                cx.notify();
            }
            NexusAction::CloseWindow => {
                cx.quit();
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
                self.dispatch(NexusAction::CloseWindow, cx);
            }
            "tab" | "down" => {
                let next = (self.state.selected_index + 1) % super::state::CATEGORIES.len();
                self.dispatch(NexusAction::SelectCategory(next), cx);
            }
            "up" => {
                let prev = if self.state.selected_index == 0 {
                    super::state::CATEGORIES.len() - 1
                } else {
                    self.state.selected_index - 1
                };
                self.dispatch(NexusAction::SelectCategory(prev), cx);
            }
            key if key.len() == 1 && key.chars().next().unwrap().is_ascii_digit() => {
                let digit = key.chars().next().unwrap().to_digit(10).unwrap_or(0);
                if digit >= 1 && (digit as usize) <= super::state::CATEGORIES.len() {
                    self.dispatch(NexusAction::SelectCategory((digit - 1) as usize), cx);
                }
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

impl Render for NexusWindow {
    fn render(&mut self, window: &mut Window, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        // Apply frosted blur background appearance to the window
        let target_appearance = if self.config.blur_enabled() && self.config.transparency_enabled() {
            WindowBackgroundAppearance::Blurred
        } else if self.config.transparency_enabled() {
            WindowBackgroundAppearance::Transparent
        } else {
            WindowBackgroundAppearance::Opaque
        };
        window.set_background_appearance(target_appearance);

        let c = &self.colours;
        let selected_idx = self.state.selected_index;
        let current_category = self.state.current_category();

        let current_page = match selected_idx {
            0 => AppearancePage::render(&self.config, c, cx),
            1 => DesktopPage::render(&self.config, c, cx),
            2 => PanelsPage::render(&self.config, c, cx),
            3 => NetworkPage::render(&self.config, c, cx),
            4 => BluetoothPage::render(&self.config, c, cx),
            5 => DisplayPage::render(&self.config, c, cx),
            6 => AudioPage::render(&self.config, c, cx),
            7 => PowerPage::render(&self.config, c, cx),
            8 => AppsPage::render(&self.config, c, cx),
            9 => ServicesPage::render(&self.config, c, cx),
            10 => AboutPage::render(&self.config, c, cx),
            _ => AppearancePage::render(&self.config, c, cx),
        };

        // Window Header Bar with title, section breadcrumb, and client-side controls
        let header_bar = div()
            .h(px(46.0))
            .w_full()
            .bg(rgba(c.surface_container_highest))
            .border_b_1()
            .border_color(rgba(c.outline_variant))
            .px(px(PADDING_MEDIUM))
            .flex()
            .items_center()
            .justify_between()
            .child(
                // Left Title and Breadcrumbs
                div()
                    .flex()
                    .items_center()
                    .gap(px(SPACING_SMALL))
                    .child(MaterialIcon::with_size("tune", IconSize::Small, c.primary))
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_LABEL_MEDIUM))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(rgba(c.on_surface))
                            .child("Caelestia Settings"),
                    )
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_SMALL))
                            .text_color(rgba(c.on_surface_variant))
                            .child("•"),
                    )
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_SMALL))
                            .text_color(rgba(c.primary))
                            .child(current_category.label),
                    ),
            )
            .child(
                // Right Window Action Controls
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    // Close button
                    .child(
                        div()
                            .cursor(CursorStyle::PointingHand)
                            .p(px(6.0))
                            .rounded(px(ROUNDING_SMALL))
                            .hover(|s| s.bg(rgba(0xFF000033)))
                            .on_mouse_down(MouseButton::Left, cx.listener(|nexus, _, _, cx| {
                                nexus.dispatch(NexusAction::CloseWindow, cx);
                            }))
                            .child(MaterialIcon::with_size("close", IconSize::Small, c.on_surface_variant)),
                    ),
            );

        // Standard Application Window Container
        div()
            .id("nexus-window-root")
            .size_full()
            .relative()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key_down))
            .bg(rgba(c.surface_container_low))
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(header_bar)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .w_full()
                    .overflow_hidden()
                    // Left Navigation Pane
                    .child(NavPane::render(&self.state, c, cx))
                    // Right Content Page with interactive controls
                    .child(current_page),
            )
    }
}
