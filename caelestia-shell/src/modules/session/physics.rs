//! Soft-body physics and curved blobrect path construction.

#![allow(dead_code)]

use gpui::{point, px, Bounds, Path, Pixels};

#[derive(Debug, Clone, Copy)]
pub struct BlobCurve {
    pub left: Pixels,
    pub right: Pixels,
    pub top: Pixels,
    pub bottom: Pixels,
    pub ctrl_x: Pixels,
    pub mid_y: Pixels,
}

impl BlobCurve {
    pub fn calculate(bounds: Bounds<Pixels>, open_progress: f32, target_width: f32) -> Self {
        let right = bounds.origin.x + bounds.size.width;
        let left = right - px(target_width * open_progress);
        let top = bounds.origin.y;
        let bottom = bounds.origin.y + bounds.size.height;
        let mid_y = (top + bottom) / 2.0;

        // Deformation curvature peaks at open_progress == 0.5
        let deformation = (1.0 - (open_progress - 0.5).abs() * 2.0).max(0.0) * 28.0;
        let ctrl_x = left - px(deformation);

        Self {
            left,
            right,
            top,
            bottom,
            ctrl_x,
            mid_y,
        }
    }

    pub fn to_path(&self) -> Path<Pixels> {
        let mut path = Path::new(point(self.right, self.top));
        path.line_to(point(self.left, self.top));
        path.curve_to(
            point(self.left, self.bottom),
            point(self.ctrl_x, self.mid_y),
        );
        path.line_to(point(self.right, self.bottom));
        path.line_to(point(self.right, self.top));
        path
    }
}
