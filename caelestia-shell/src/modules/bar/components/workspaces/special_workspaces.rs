//! Special Workspaces (Scratchpads) Indicator
//! Parity with `shell/modules/bar/components/workspaces/SpecialWorkspaces.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct SpecialWorkspacesWidget;

impl SpecialWorkspacesWidget {
    pub fn render(
        special_names: &[String],
        active_special: &str,
        c: &CaelestiaColours,
    ) -> Div {
        if special_names.is_empty() {
            return div();
        }

        let mut row = div().flex().flex_row().items_center().gap_1();
        for name in special_names {
            let is_active = name == active_special;
            row = row.child(
                div()
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded_full()
                    .bg(if is_active {
                        rgba(c.secondary_container)
                    } else {
                        rgba(c.surface_container)
                    })
                    .child(
                        div()
                            .text_xs()
                            .text_color(if is_active {
                                rgba(c.on_secondary_container)
                            } else {
                                rgba(c.on_surface_variant)
                            })
                            .child(name.clone()),
                    ),
            );
        }
        row
    }
}
