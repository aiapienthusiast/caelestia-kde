//! Dashboard Drawer Component
//! Parity with `shell/modules/drawers/ContentWindow.qml` and `shell/modules/dashboard/Content.qml`.
//! Hosts the complete Caelestia Dashboard with KWin frosted blur, tabs, and live services.

use super::dash::DashView;
use super::media::MediaView;
use super::performance::PerformanceView;
use super::tabs::{DashboardTab, DashboardTabs};
use super::weather_tab::WeatherTabView;
use crate::domain::tokens::*;
use crate::infrastructure::paths::CaelestiaPaths;
use crate::infrastructure::sysinfo::SystemInfo;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::mpris::MprisService;
use crate::services::system_monitor::SystemMonitorService;
use crate::services::weather::WeatherService;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, FocusHandle, KeyDownEvent,
    MouseButton, MouseDownEvent, Render, Styled, Window, WindowBackgroundAppearance,
};
use std::time::{Duration, Instant, SystemTime};

pub struct DashboardDrawer {
    focus_handle: FocusHandle,
    pub current_tab: DashboardTab,
    pub colours: CaelestiaColours,
    pub paths: CaelestiaPaths,
    pub sysinfo: SystemInfo,
    pub monitor: SystemMonitorService,
    pub weather: WeatherService,
    pub mpris: MprisService,
    pub is_open: bool,
    open_progress: f32,
    anim_phase: f32,
    last_monitor_tick: Instant,
    last_scheme_check: Instant,
    last_scheme_mtime: Option<SystemTime>,
}

impl DashboardDrawer {
    pub fn new(cx: &mut gpui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let colours = CaelestiaColours::load();
        let paths = CaelestiaPaths::new();
        let sysinfo = SystemInfo::load();
        let monitor = SystemMonitorService::new();
        let weather = WeatherService::load();
        let mpris = MprisService::new();

        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let scheme_path = format!("{}/.local/state/caelestia/scheme.json", home);
        let last_scheme_mtime = std::fs::metadata(&scheme_path)
            .ok()
            .and_then(|m| m.modified().ok());

        let is_open = std::env::var("DASHBOARD_OPEN").map(|v| v != "0").unwrap_or(true);

        // 60 FPS animation loop for visualizer and physics
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_millis(16)).await;
                let _ = this.update(cx, |dash, cx| {
                    dash.tick_animation();
                    cx.notify();
                });
            }
        })
        .detach();

        Self {
            focus_handle,
            current_tab: DashboardTab::Dashboard,
            colours,
            paths,
            sysinfo,
            monitor,
            weather,
            mpris,
            is_open,
            open_progress: if is_open { 1.0 } else { 0.0 },
            anim_phase: 0.0,
            last_monitor_tick: Instant::now(),
            last_scheme_check: Instant::now(),
            last_scheme_mtime,
        }
    }

    fn tick_animation(&mut self) {
        let target = if self.is_open { 1.0 } else { 0.0 };
        let diff = target - self.open_progress;
        self.open_progress += diff * 0.18;
        self.open_progress = self.open_progress.clamp(0.0, 1.0);
        self.anim_phase += 0.05;

        // Refresh hardware metrics every 1.5 seconds
        if self.last_monitor_tick.elapsed() > Duration::from_millis(1500) {
            self.last_monitor_tick = Instant::now();
            self.monitor.refresh();
            self.mpris.poll_playerctl();
        }

        // Hot-reload scheme.json
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

    pub fn set_tab(&mut self, tab: DashboardTab, cx: &mut gpui::Context<Self>) {
        self.current_tab = tab;
        cx.notify();
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
            "tab" | "right" => {
                let next = (self.current_tab as usize + 1) % DashboardTab::ALL.len();
                self.current_tab = DashboardTab::ALL[next];
                cx.notify();
            }
            "left" => {
                let prev = if (self.current_tab as usize) == 0 {
                    DashboardTab::ALL.len() - 1
                } else {
                    self.current_tab as usize - 1
                };
                self.current_tab = DashboardTab::ALL[prev];
                cx.notify();
            }
            _ => {}
        }
    }
}

impl Render for DashboardDrawer {
    fn render(&mut self, window: &mut Window, cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        // Request KWin frosted blur
        let target_appearance = if self.open_progress > 0.01 {
            WindowBackgroundAppearance::Blurred
        } else {
            WindowBackgroundAppearance::Transparent
        };
        window.set_background_appearance(target_appearance);

        let c = &self.colours;
        let current_tab = self.current_tab;
        tracing::info!("DashboardDrawer::render called! current_tab={:?}, open_progress={}", current_tab, self.open_progress);

        let content = match current_tab {
            DashboardTab::Dashboard => {
                DashView::render(&self.weather.data, &self.sysinfo, &self.paths, c)
            }
            DashboardTab::Media => MediaView::render(&self.mpris, self.anim_phase, c, cx),
            DashboardTab::Performance => PerformanceView::render(&self.monitor, c),
            DashboardTab::Weather => WeatherTabView::render(&self.weather.data, c),
        };

        div()
            .id("dashboard-drawer-backdrop")
            .size_full()
            .relative()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key_down))
            .bg(rgba(0x00000040))
            .flex()
            .items_center()
            .justify_center()
            // Dismiss on clicking backdrop outside the dialog surface
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|dash, _event: &MouseDownEvent, _window, cx| {
                    if dash.is_open {
                        dash.is_open = false;
                        cx.quit();
                    }
                }),
            )
            // Centered Dashboard Dialog (matching drawers/ContentWindow.qml proportions)
            .child(
                div()
                    .id("dashboard-dialog-surface")
                    .w(px(880.0))
                    .h(px(560.0))
                    .rounded(px(ROUNDING_EXTRA_LARGE))
                    .bg(rgba(c.surface_container_low))
                    .border_1()
                    .border_color(rgba(c.outline_variant))
                    .shadow_xl()
                    .flex()
                    .flex_col()
                    .p(px(24.0))
                    .gap(px(20.0))
                    .cursor(CursorStyle::Arrow)
                    // Intercept clicks inside dialog so it doesn't dismiss
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    // Top Tabs Row
                    .child(DashboardTabs::render(
                        current_tab,
                        c,
                        cx,
                        |dash: &mut DashboardDrawer, tab, cx| {
                            dash.set_tab(tab, cx);
                        },
                    ))
                    // Active Tab Content View
                    .child(
                        div()
                            .flex_1()
                            .w_full()
                            .overflow_hidden()
                            .child(content),
                    ),
            )
    }
}
