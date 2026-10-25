//! Individual Workspace Pill
//! Parity with `shell/modules/bar/components/workspaces/Workspace.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use super::workspaces::WorkspacePill;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct WorkspacePillWidget;

impl WorkspacePillWidget {
    pub fn render(pill: &WorkspacePill, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .items_center()
            .justify_center()
            .min_w(px(20.0))
            .h(px(20.0))
            .px(px(4.0))
            .rounded_full()
            .cursor_pointer()
            .bg(if pill.active {
                rgba(c.primary)
            } else if pill.occupied {
                rgba(c.surface_container_highest)
            } else {
                rgba(0x00000000)
            })
            .child(
                div()
                    .text_xs()
                    .font_weight(if pill.active {
                        gpui::FontWeight::BOLD
                    } else {
                        gpui::FontWeight::NORMAL
                    })
                    .text_color(if pill.active {
                        rgba(c.on_primary)
                    } else if pill.occupied {
                        rgba(c.on_surface)
                    } else {
                        rgba(c.outline)
                    })
                    .child(pill.name.clone()),
            )
    }
}
