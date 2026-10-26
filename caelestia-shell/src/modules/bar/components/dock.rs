//! Bar Dock / Active Window Component & Model
//! Complete parity with `shell/modules/bar/components/Dock.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::kwin::WindowItem;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, Styled};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DockSnapshot {
    pub title: String,
    pub app_id: String,
    pub has_window: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DockItem {
    pub app_id: String,
    pub name: String,
    pub icon: String,
    pub is_pinned: bool,
    pub is_running: bool,
    pub is_active: bool,
    pub is_urgent: bool,
    pub window_count: usize,
}

impl Default for DockItem {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            name: String::new(),
            icon: "apps".to_string(),
            is_pinned: false,
            is_running: false,
            is_active: false,
            is_urgent: false,
            window_count: 0,
        }
    }
}

pub struct BarDockModel;

impl BarDockModel {
    pub fn from_active_window(window: Option<&WindowItem>) -> DockSnapshot {
        if let Some(win) = window {
            let title = if win.title.is_empty() {
                win.app_id.clone()
            } else {
                win.title.clone()
            };

            DockSnapshot {
                title,
                app_id: win.app_id.clone(),
                has_window: true,
            }
        } else {
            DockSnapshot {
                title: String::new(),
                app_id: String::new(),
                has_window: false,
            }
        }
    }
}

pub struct DockWidget;

impl DockWidget {
    /// Renders active window title pill (compact bar mode).
    pub fn render(snapshot: &DockSnapshot, c: &CaelestiaColours) -> Div {
        if !snapshot.has_window {
            return div();
        }

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(SPACING_EXTRA_SMALL))
            .px(px(PADDING_MEDIUM))
            .py(px(PADDING_EXTRA_SMALL))
            .rounded_full()
            .bg(rgba(c.surface_container))
            .cursor(CursorStyle::PointingHand)
            .child(
                MaterialIcon::with_size("window", IconSize::Small, c.secondary),
            )
            .child(
                StyledText::new(&snapshot.title)
                    .text_size(px(FONT_SIZE_BODY_SMALL))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(c.on_surface)),
            )
    }

    /// Renders the complete application dock with pinned + running icons, active indicators,
    /// and M3 state layers matching Dock.qml.
    pub fn render_dock(
        items: &[DockItem],
        is_horizontal: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let padding_h = PADDING_MEDIUM;
        let padding_v = PADDING_EXTRA_SMALL;

        let container = div()
            .relative()
            .flex()
            .items_center()
            .px(px(padding_h))
            .py(px(padding_v))
            .rounded_full()
            .bg(rgba(c.surface_container));

        let layout = if is_horizontal {
            div().flex().flex_row().items_center().gap(px(SPACING_SMALL))
        } else {
            div().flex().flex_col().items_center().gap(px(SPACING_SMALL))
        };

        let items_view = layout.children(items.iter().map(|item| {
            Self::render_dock_item(item, is_horizontal, false, false, c)
        }));

        container.child(items_view)
    }

    /// Renders an individual dock item delegate with active indicator dot/pill.
    pub fn render_dock_item(
        item: &DockItem,
        is_horizontal: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let item_size = 36.0;

        let mut delegate = div()
            .relative()
            .w(px(item_size))
            .h(px(item_size))
            .rounded(px(ROUNDING_MEDIUM))
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand);

        // Active background highlight (Dock.qml:521-532)
        if item.is_active {
            delegate = delegate.bg(rgba(0xffffff1a)); // 10% onSurface
        } else if item.is_urgent {
            delegate = delegate.bg(rgba(c.error | 0x59000000)); // 35% error
        }

        // StateLayer
        delegate = delegate.child(StateLayer::overlay(
            is_hovered,
            is_pressed,
            false,
            ROUNDING_MEDIUM,
            c.primary,
        ));

        // App Icon
        let icon_name = if item.icon.is_empty() { "apps" } else { &item.icon };
        delegate = delegate.child(
            MaterialIcon::with_size(icon_name, IconSize::Medium, c.on_surface),
        );

        // Running indicator pill at bottom/side (Dock.qml: running dot/pill)
        if item.is_running {
            let indicator_size = if item.is_active { 12.0 } else { 4.0 };
            let indicator = if is_horizontal {
                div()
                    .absolute()
                    .bottom(px(2.0))
                    .w(px(indicator_size))
                    .h(px(3.0))
                    .rounded_full()
                    .bg(rgba(if item.is_active { c.primary } else { c.outline }))
            } else {
                div()
                    .absolute()
                    .left(px(2.0))
                    .w(px(3.0))
                    .h(px(indicator_size))
                    .rounded_full()
                    .bg(rgba(if item.is_active { c.primary } else { c.outline }))
            };

            delegate = delegate.child(indicator);
        }

        delegate
    }
}

