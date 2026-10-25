//! Bar View Component
//! Parity with `shell/modules/bar/Bar.qml` and `BarWrapper.qml`.
//!
//! Renders the complete desktop status bar with left, middle, and right zones,
//! supporting island borders, workspace pills, active window dock, status cluster,
//! clock, system tray, media spectrum visualizer, and contextual popouts.

use super::clock::{BarClockModel, ClockSnapshot};
use super::config::BarConfig;
use super::dock::{BarDockModel, DockSnapshot};
use super::popouts::{BarPopouts, BarPopoutsState, PopoutType};
use super::status::{BarStatusSnapshot, StatusIconsModel};
use super::workspaces::{WorkspacePill, WorkspacesModel};
use crate::components::styled_text::StyledTextExt;
use crate::components::widgets::visualizer_bars::VisualizerBars;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::audio::AudioService;
use crate::services::bluetooth::BluetoothService;
use crate::services::kwin::KWinService;
use crate::services::network::NetworkService;
use crate::services::tray_registry::TrayItem;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, IntoElement, Render, Styled, Window,
};

pub struct BarView {
    pub config: BarConfig,
    pub colours: CaelestiaColours,
    pub popouts: BarPopoutsState,

    // Live snapshots
    pub workspaces: Vec<WorkspacePill>,
    pub dock: DockSnapshot,
    pub status: BarStatusSnapshot,
    pub clock: ClockSnapshot,
    pub tray_items: Vec<TrayItem>,
    pub media_title: String,
    pub media_artist: String,
    pub media_playing: bool,
    pub visualizer_levels: Vec<f32>,
}

impl BarView {
    pub fn new(colours: CaelestiaColours) -> Self {
        let dummy_audio = AudioService::new();
        let dummy_net = NetworkService::new();
        let dummy_bt = BluetoothService::new();

        let status = StatusIconsModel::snapshot(&dummy_audio, &dummy_net, &dummy_bt);
        let clock = BarClockModel::current(true, false);
        let dock = DockSnapshot {
            title: String::new(),
            app_id: String::new(),
            has_window: false,
        };

        Self {
            config: BarConfig::default(),
            colours,
            popouts: BarPopoutsState::default(),
            workspaces: Vec::new(),
            dock,
            status,
            clock,
            tray_items: Vec::new(),
            media_title: "Starboy".to_string(),
            media_artist: "The Weeknd".to_string(),
            media_playing: true,
            visualizer_levels: vec![0.3, 0.7, 0.9, 0.6, 0.4, 0.8, 0.5, 0.2],
        }
    }

    /// Update live models from services
    pub fn update_services(
        &mut self,
        kwin: &KWinService,
        audio: &AudioService,
        network: &NetworkService,
        bluetooth: &BluetoothService,
    ) {
        self.workspaces = WorkspacesModel::compute_pills(
            kwin,
            self.config.workspaces_shown,
            self.config.workspaces_show_unoccupied,
            None,
        );
        self.dock = BarDockModel::from_active_window(kwin.state.active_window.as_ref());
        self.status = StatusIconsModel::snapshot(audio, network, bluetooth);
        self.clock = BarClockModel::current(self.config.clock_24h, self.config.clock_show_seconds);
    }
}

impl Render for BarView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let c = &self.colours;

        let bar_height = self.config.thickness;
        let margin_top = if self.config.islands { self.config.island_spacing } else { 0.0 };
        let margin_x = if self.config.islands { self.config.island_spacing } else { 0.0 };
        let radius = if self.config.islands { 20.0 } else { 0.0 };

        let mut root = div()
            .relative()
            .w_full()
            .h(px(bar_height + margin_top))
            .pt(px(margin_top))
            .px(px(margin_x))
            .flex()
            .flex_col();

        // Bar Island Container
        let bar_island = div()
            .w_full()
            .h(px(bar_height))
            .rounded(px(radius))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .px_3()
            .flex()
            .items_center()
            .justify_between()
            // =========================================================
            // Left Zone: OS Logo + Workspace Switcher
            // =========================================================
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Logo Pill
                    .child(
                        div()
                            .w(px(32.0))
                            .h(px(32.0))
                            .rounded_full()
                            .bg(rgba(c.primary))
                            .text_color(rgba(c.on_primary))
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor(CursorStyle::PointingHand)
                            .child(StyledTextExt::icon_size(div().child("auto_awesome"), 16.0)),
                    )
                    // Workspaces Pills
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .p_1()
                            .rounded_full()
                            .bg(rgba(c.surface_container_low))
                            .children(
                                self.workspaces.iter().map(|ws| {
                                    let is_active = ws.active;
                                    div()
                                        .px_2()
                                        .py_1()
                                        .rounded_full()
                                        .cursor(CursorStyle::PointingHand)
                                        .bg(if is_active {
                                            rgba(c.primary)
                                        } else if ws.occupied {
                                            rgba(c.surface_container_highest)
                                        } else {
                                            rgba(0x00000000)
                                        })
                                        .text_color(if is_active {
                                            rgba(c.on_primary)
                                        } else {
                                            rgba(c.on_surface)
                                        })
                                        .text_xs()
                                        .font_weight(if is_active {
                                            gpui::FontWeight::BOLD
                                        } else {
                                            gpui::FontWeight::NORMAL
                                        })
                                        .child(ws.name.clone())
                                }),
                            ),
                    ),
            )
            // =========================================================
            // Middle Zone: Active Window Dock + System Tray
            // =========================================================
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Active Window Title
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_1()
                            .rounded_full()
                            .bg(rgba(c.surface_container_low))
                            .cursor(CursorStyle::PointingHand)
                            .child(
                                div()
                                    .text_color(rgba(c.on_surface_variant))
                                    .child(StyledTextExt::icon_size(div().child("window"), 14.0)),
                            )
                            .child(
                                div()
                                    .text_color(rgba(c.on_surface))
                                    .text_xs()
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .child(if self.dock.has_window {
                                        self.dock.title.clone()
                                    } else {
                                        "Caelestia Shell".to_string()
                                    }),
                            )
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(|this, _, _, _cx| {
                                    this.popouts.toggle(PopoutType::DockContext, 500.0);
                                }),
                            ),
                    )
                    // System Tray Icons
                    .children(self.tray_items.iter().map(|item| {
                        let item_id = item.id.clone();
                        div()
                            .px_2()
                            .py_1()
                            .rounded_full()
                            .bg(rgba(c.surface_container_low))
                            .cursor(CursorStyle::PointingHand)
                            .child(
                                div()
                                    .text_color(rgba(c.on_surface))
                                    .text_xs()
                                    .child(item_id),
                            )
                    })),
            )
            // =========================================================
            // Right Zone: Media Pill + Status Cluster + Clock + Power
            // =========================================================
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Media Pill with Audio Spectrum Visualizer
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_1()
                            .rounded_full()
                            .bg(rgba(c.surface_container_low))
                            .cursor(CursorStyle::PointingHand)
                            .child(
                                div()
                                    .text_color(rgba(c.primary))
                                    .child(StyledTextExt::icon_size(div().child("music_note"), 14.0)),
                            )
                            .child(
                                div()
                                    .text_color(rgba(c.on_surface))
                                    .text_xs()
                                    .max_w(px(100.0))
                                    .overflow_hidden()
                                    .child(self.media_title.clone()),
                            )
                            .child(VisualizerBars::render(
                                8,
                                0.5,
                                self.media_playing,
                                c.primary,
                                14.0,
                            ))
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(|this, _, _, _cx| {
                                    this.popouts.toggle(PopoutType::Media, 720.0);
                                }),
                            ),
                    )
                    // Status Cluster Pill (Volume, Network, Bluetooth, Battery)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .px_3()
                            .py_1()
                            .rounded_full()
                            .bg(rgba(c.surface_container_low))
                            .cursor(CursorStyle::PointingHand)
                            .child(
                                div()
                                    .text_color(rgba(c.on_surface))
                                    .child(StyledTextExt::icon_size(
                                        div().child(self.status.network_icon),
                                        16.0,
                                    )),
                            )
                            .child(
                                div()
                                    .text_color(rgba(c.on_surface))
                                    .child(StyledTextExt::icon_size(
                                        div().child(self.status.bluetooth_icon),
                                        16.0,
                                    )),
                            )
                            .child(
                                div()
                                    .text_color(rgba(c.on_surface))
                                    .child(StyledTextExt::icon_size(
                                        div().child(self.status.volume_icon),
                                        16.0,
                                    )),
                            )
                            .child(
                                div()
                                    .text_color(rgba(c.on_surface))
                                    .text_xs()
                                    .child(format!("{}%", self.status.battery_pct)),
                            )
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(|this, _, _, _cx| {
                                    this.popouts.toggle(PopoutType::Audio, 850.0);
                                }),
                            ),
                    )
                    // Clock Pill
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_1()
                            .rounded_full()
                            .bg(rgba(c.surface_container_low))
                            .cursor(CursorStyle::PointingHand)
                            .child(
                                div()
                                    .text_color(rgba(c.on_surface))
                                    .text_xs()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child(self.clock.time_str.clone()),
                            )
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(|this, _, _, _cx| {
                                    this.popouts.toggle(PopoutType::Calendar, 940.0);
                                }),
                            ),
                    )
                    // Power Button
                    .child(
                        div()
                            .w(px(32.0))
                            .h(px(32.0))
                            .rounded_full()
                            .bg(rgba(c.error_container))
                            .text_color(rgba(c.on_error_container))
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor(CursorStyle::PointingHand)
                            .child(StyledTextExt::icon_size(div().child("power_settings_new"), 16.0))
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(|this, _, _, _cx| {
                                    this.popouts.toggle(PopoutType::Power, 980.0);
                                }),
                            ),
                    ),
            );

        root = root.child(bar_island);

        // Popout overlay rendering
        if self.popouts.is_open {
            if let Some(popout_type) = self.popouts.current {
                root = root.child(BarPopouts::render(popout_type, self.popouts.current_center, c));
            }
        }

        root
    }
}
