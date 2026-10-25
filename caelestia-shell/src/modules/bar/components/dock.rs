//! Bar Dock / Active Window Component & Model
//! Complete parity with `shell/modules/bar/components/Dock.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use crate::services::kwin::WindowItem;
use gpui::{div, prelude::*, px, rgba, Div, Styled};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DockSnapshot {
    pub title: String,
    pub app_id: String,
    pub has_window: bool,
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
    pub fn render(snapshot: &DockSnapshot, c: &CaelestiaColours) -> Div {
        if !snapshot.has_window {
            return div();
        }

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .px(px(6.0))
            .py(px(2.0))
            .rounded_md()
            .bg(rgba(c.surface_container))
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface_variant))
                    .child(snapshot.title.clone()),
            )
    }
}
