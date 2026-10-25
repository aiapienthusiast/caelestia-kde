//! Gap Markers for Workspaces Strip
//! Parity with `shell/modules/bar/components/workspaces/GapMarkers.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct GapMarkersWidget;

impl GapMarkersWidget {
    pub fn render(
        marker_positions: &[f32],
        is_horizontal: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let mut container = div().absolute().inset_0();
        for &pos in marker_positions {
            container = container.child(
                div()
                    .absolute()
                    .bg(rgba(c.outline_variant))
                    .when(is_horizontal, |d| {
                        d.w(px(1.0)).h_full().left(px(pos))
                    })
                    .when(!is_horizontal, |d| {
                        d.h(px(1.0)).w_full().top(px(pos))
                    }),
            );
        }
        container
    }
}
