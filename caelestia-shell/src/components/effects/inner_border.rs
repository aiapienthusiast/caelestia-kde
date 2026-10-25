//! InnerBorder
//! Parity with `shell/components/effects/InnerBorder.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, px, rgba, Div, Styled};

pub struct InnerBorder;

impl InnerBorder {
    /// Renders a 1px crisp frosted border overlay inset inside rounded corners.
    pub fn overlay(
        rounding: f32,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom_0()
            .rounded(px(rounding))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .opacity(0.4)
    }
}
