//! Performance Tab
//! Parity with `shell/modules/dashboard/Performance.qml`.
//! Displays real-time CPU, RAM, Disk, and Swap metrics with live sparklines.

use crate::components::icon::MaterialIcon;
use crate::components::styled_text::{StyledText, StyledTextExt};
use crate::components::widgets::sparkline::Sparkline;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::system_monitor::SystemMonitorService;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct PerformanceView;

impl PerformanceView {
    pub fn render(monitor: &SystemMonitorService, c: &CaelestiaColours) -> Div {
        let cpu_pct = monitor.state.cpu_usage.round() as u32;
        let ram_used = monitor.ram_used_gib();
        let ram_total = monitor.ram_total_gib();
        let ram_pct = if ram_total > 0.0 {
            ((ram_used / ram_total) * 100.0).round() as u32
        } else {
            0
        };

        let disk_pct = monitor.disk_percent().round() as u32;
        let disk_used_gb = monitor.state.disk_used_bytes / (1024 * 1024 * 1024);
        let disk_total_gb = monitor.state.disk_total_bytes / (1024 * 1024 * 1024);

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .w_full()
            .h_full()
            .child(
                // Hero Row: CPU Card + RAM Card
                div()
                    .flex()
                    .gap(px(16.0))
                    .w_full()
                    .child(Self::render_cpu_card(cpu_pct, monitor, c))
                    .child(Self::render_ram_card(ram_used, ram_total, ram_pct, monitor, c)),
            )
            .child(
                // Bottom Row: Disk Card + Swap Card
                div()
                    .flex()
                    .gap(px(16.0))
                    .w_full()
                    .child(Self::render_disk_card(disk_used_gb, disk_total_gb, disk_pct, c))
                    .child(Self::render_swap_card(monitor, c)),
            )
    }

    fn render_cpu_card(
        cpu_pct: u32,
        monitor: &SystemMonitorService,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .flex_1()
            .p(px(20.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .flex()
            .flex_col()
            .justify_between()
            .h(px(170.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .child(MaterialIcon::new("memory").color(c.primary).icon_size(24.0))
                            .child(
                                StyledText::new("Processor")
                                    .color(c.on_surface)
                                    .font_size(16.0)
                                    .font_weight(gpui::FontWeight::BOLD),
                            ),
                    )
                    .child(
                        StyledText::new(&format!("{}%", cpu_pct))
                            .color(c.primary)
                            .font_size(24.0)
                            .font_weight(gpui::FontWeight::BOLD),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        StyledText::new(&format!("{} Cores Active", monitor.state.cpu_cores_usage.len()))
                            .color(c.on_surface_variant)
                            .font_size(12.0),
                    )
                    .child(Sparkline::render(&monitor.state.cpu_history, c.primary, 48.0)),
            )
    }

    fn render_ram_card(
        used: f64,
        total: f64,
        pct: u32,
        monitor: &SystemMonitorService,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .flex_1()
            .p(px(20.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .flex()
            .flex_col()
            .justify_between()
            .h(px(170.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .child(MaterialIcon::new("storage").color(c.secondary).icon_size(24.0))
                            .child(
                                StyledText::new("Memory")
                                    .color(c.on_surface)
                                    .font_size(16.0)
                                    .font_weight(gpui::FontWeight::BOLD),
                            ),
                    )
                    .child(
                        StyledText::new(&format!("{}%", pct))
                            .color(c.secondary)
                            .font_size(24.0)
                            .font_weight(gpui::FontWeight::BOLD),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        StyledText::new(&format!("{:.1} / {:.1} GiB", used, total))
                            .color(c.on_surface_variant)
                            .font_size(12.0),
                    )
                    .child(Sparkline::render(&monitor.state.ram_history, c.secondary, 48.0)),
            )
    }

    fn render_disk_card(used_gb: u64, total_gb: u64, pct: u32, c: &CaelestiaColours) -> Div {
        div()
            .flex_1()
            .p(px(20.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .flex()
            .flex_col()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .child(MaterialIcon::new("hard_drive").color(c.secondary).icon_size(22.0))
                            .child(
                                StyledText::new("Storage")
                                    .color(c.on_surface)
                                    .font_size(15.0)
                                    .font_weight(gpui::FontWeight::BOLD),
                            ),
                    )
                    .child(
                        StyledText::new(&format!("{} GB / {} GB ({}%)", used_gb, total_gb, pct))
                            .color(c.on_surface_variant)
                            .font_size(13.0),
                    ),
            )
            .child(
                // Progress bar
                div()
                    .w_full()
                    .h(px(8.0))
                    .rounded_full()
                    .bg(rgba(c.surface_container_high))
                    .overflow_hidden()
                    .child(
                        div()
                            .w(px(pct.min(100) as f32 * 3.5))
                            .h_full()
                            .rounded_full()
                            .bg(rgba(c.secondary)),
                    ),
            )
    }

    fn render_swap_card(monitor: &SystemMonitorService, c: &CaelestiaColours) -> Div {
        let swap_used_gb = monitor.state.swap_used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        let swap_total_gb = monitor.state.swap_total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        let swap_pct = if swap_total_gb > 0.0 {
            ((swap_used_gb / swap_total_gb) * 100.0).round() as u32
        } else {
            0
        };

        div()
            .flex_1()
            .p(px(20.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .flex()
            .flex_col()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .child(MaterialIcon::new("swap_horiz").color(c.primary).icon_size(22.0))
                            .child(
                                StyledText::new("Swap Space")
                                    .color(c.on_surface)
                                    .font_size(15.0)
                                    .font_weight(gpui::FontWeight::BOLD),
                            ),
                    )
                    .child(
                        StyledText::new(&format!("{:.1} / {:.1} GiB ({}%)", swap_used_gb, swap_total_gb, swap_pct))
                            .color(c.on_surface_variant)
                            .font_size(13.0),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .h(px(8.0))
                    .rounded_full()
                    .bg(rgba(c.surface_container_high))
                    .overflow_hidden()
                    .child(
                        div()
                            .w(px(swap_pct.min(100) as f32 * 3.5))
                            .h_full()
                            .rounded_full()
                            .bg(rgba(c.primary)),
                    ),
            )
    }
}
