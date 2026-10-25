//! Tray Widget
//! Parity with `shell/modules/bar/components/Tray.qml` and `TrayItem.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use crate::services::tray_registry::TrayItem;
use gpui::{div, prelude::*, rgba, CursorStyle, Div, Styled};

pub struct TrayWidget;

impl TrayWidget {
    /// Renders the system tray container hosting individual SNI items.
    pub fn render(items: &[TrayItem], c: &CaelestiaColours) -> Div {
        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1();

        for item in items {
            row = row.child(Self::render_item(item, c));
        }

        row
    }

    /// Renders an individual SNI system tray item delegate.
    pub fn render_item(item: &TrayItem, c: &CaelestiaColours) -> Div {
        div()
            .px_2()
            .py_1()
            .rounded_full()
            .bg(rgba(c.surface_container_low))
            .text_color(rgba(c.on_surface))
            .cursor(CursorStyle::PointingHand)
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(item.id.clone()),
            )
    }
}
