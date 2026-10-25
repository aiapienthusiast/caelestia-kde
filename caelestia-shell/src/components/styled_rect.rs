//! StyledRect & StyledClippingRect
//! Parity with `shell/components/StyledRect.qml` and `shell/components/StyledClippingRect.qml`.

use gpui::{div, px, rgba, Div, Styled};

/// Builder function returning a StyledRect configured `Div`.
pub fn styled_rect() -> Div {
    div().relative()
}

/// Helper extension trait for common StyledRect properties.
pub trait StyledRectExt: Styled + Sized {
    /// Apply M3 surface background color.
    fn surface_bg(self, color: u32) -> Self {
        self.bg(rgba(color))
    }

    /// Apply M3 border outline.
    fn outline_border(self, color: u32, width_px: f32) -> Self {
        self.border(px(width_px)).border_color(rgba(color))
    }

    /// Apply standard corner rounding token.
    fn corner_radius(self, radius_px: f32) -> Self {
        self.rounded(px(radius_px))
    }

    /// Apply asymmetric corner rounding.
    fn asymmetric_corners(
        self,
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    ) -> Self {
        self.rounded_tl(px(top_left))
            .rounded_tr(px(top_right))
            .rounded_br(px(bottom_right))
            .rounded_bl(px(bottom_left))
    }
}

impl<T: Styled> StyledRectExt for T {}

/// Styled clipping rect container that clips its children to its bounding radius.
pub fn styled_clipping_rect() -> Div {
    div().relative().overflow_hidden()
}
