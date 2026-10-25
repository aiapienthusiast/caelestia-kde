//! Occupied Background Container for Workspaces Strip
//! Parity with `shell/modules/bar/components/workspaces/OccupiedBg.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct OccupiedBgWidget;

impl OccupiedBgWidget {
    pub fn render(
        occupied_ranges: &[(f32, f32)],
        is_horizontal: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let mut container = div().absolute().inset_0();
        for &(start, end) in occupied_ranges {
            let length = (end - start).max(4.0);
            container = container.child(
                div()
                    .absolute()
                    .rounded_full()
                    .bg(rgba(c.surface_container_high))
                    .when(is_horizontal, |d| {
                        d.left(px(start)).w(px(length)).h_full()
                    })
                    .when(!is_horizontal, |d| {
                        d.top(px(start)).h(px(length)).w_full()
                    }),
            );
        }
        container
    }
}
