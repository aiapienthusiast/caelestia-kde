//! Tray Widget
//! Parity with `shell/modules/bar/components/Tray.qml` and `TrayItem.qml`.

use super::tray_item::TrayItemWidget;
use crate::components::icon::{IconSize, MaterialIcon};
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::tray_registry::TrayItem;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct TrayWidget;

impl TrayWidget {
    /// Renders the system tray container hosting individual SNI items.
    pub fn render(items: &[TrayItem], c: &CaelestiaColours) -> Div {
        Self::render_full(items, true, true, false, c)
    }

    /// Renders with orientation, background, and compact expander options.
    pub fn render_full(
        items: &[TrayItem],
        is_horizontal: bool,
        has_background: bool,
        is_expanded: bool,
        c: &CaelestiaColours,
    ) -> Div {
        if items.is_empty() {
            return div();
        }

        let padding_h = if has_background { PADDING_MEDIUM } else { PADDING_EXTRA_SMALL };
        let padding_v = PADDING_EXTRA_SMALL;
        let spacing = if has_background { SPACING_SMALL } else { 0.0 };

        let mut container = div()
            .relative()
            .flex()
            .items_center()
            .px(px(padding_h))
            .py(px(padding_v))
            .rounded_full();

        if has_background {
            container = container.bg(rgba(c.surface_container));
        }

        let mut layout = if is_horizontal {
            div().flex().flex_row().items_center().gap(px(spacing))
        } else {
            div().flex().flex_col().items_center().gap(px(spacing))
        };

        for item in items {
            layout = layout.child(Self::render_item(item, c));
        }

        // Expand / collapse arrow for compact tray
        if is_expanded {
            let arrow_icon = if is_horizontal { "expand_more" } else { "chevron_left" };
            layout = layout.child(
                div()
                    .w(px(20.0))
                    .h(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor(CursorStyle::PointingHand)
                    .child(MaterialIcon::with_size(arrow_icon, IconSize::Small, c.secondary)),
            );
        }

        container.child(layout)
    }

    /// Renders an individual SNI system tray item delegate.
    pub fn render_item(item: &TrayItem, c: &CaelestiaColours) -> Div {
        let icon_name = if item.icon_name.is_empty() {
            "extension"
        } else {
            &item.icon_name
        };
        TrayItemWidget::render(&item.id, &item.title, icon_name, c)
    }
}

