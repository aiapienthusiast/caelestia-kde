//! Performance Popout
//! 100% Parity with `shell/modules/bar/popouts/Performance.qml`.
//! Embeds the dashboard performance view with live sparklines and resource cards.

use super::wrapper::PopoutWrapper;
use crate::infrastructure::theme::CaelestiaColours;
use crate::modules::dashboard::performance::PerformanceView;
use crate::services::system_monitor::SystemMonitorService;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct PerformancePopout;

impl PerformancePopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let monitor = SystemMonitorService::new();
        Self::render_with_monitor(center_x, &monitor, c)
    }

    /// Renders the performance popout with full parity to Performance.qml.
    pub fn render_with_monitor(center_x: f32, monitor: &SystemMonitorService, c: &CaelestiaColours) -> Div {
        let content = div()
            .w(px(320.0))
            .child(PerformanceView::render(monitor, c));

        PopoutWrapper::render("Performance", "speed", center_x, content, c)
    }
}
