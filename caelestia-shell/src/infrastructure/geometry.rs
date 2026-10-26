//! Window and layer-shell surface geometry calculations
//! Ported with 100% parity from `shell/utils/Centering.qml` and `shell/utils/Surface.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeAnchor {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

pub struct GeometryUtils;

impl GeometryUtils {
    /// Center an inner rect inside an outer rect
    pub fn center_inside(inner_w: f64, inner_h: f64, outer: Rect) -> Rect {
        let x = outer.x + (outer.width - inner_w) / 2.0;
        let y = outer.y + (outer.height - inner_h) / 2.0;
        Rect {
            x,
            y,
            width: inner_w,
            height: inner_h,
        }
    }

    /// Aspect ratio fit: scale inner to fit completely inside outer while preserving aspect ratio
    pub fn fit_aspect(src_w: f64, src_h: f64, dest_w: f64, dest_h: f64) -> (f64, f64) {
        if src_w <= 0.0 || src_h <= 0.0 || dest_w <= 0.0 || dest_h <= 0.0 {
            return (0.0, 0.0);
        }
        let scale = (dest_w / src_w).min(dest_h / src_h);
        (src_w * scale, src_h * scale)
    }

    /// Aspect ratio crop/fill: scale inner to cover outer completely while preserving aspect ratio
    pub fn fill_aspect(src_w: f64, src_h: f64, dest_w: f64, dest_h: f64) -> (f64, f64) {
        if src_w <= 0.0 || src_h <= 0.0 || dest_w <= 0.0 || dest_h <= 0.0 {
            return (0.0, 0.0);
        }
        let scale = (dest_w / src_w).max(dest_h / src_h);
        (src_w * scale, src_h * scale)
    }

    /// Clamp a point or rectangle so it remains entirely within boundary
    pub fn clamp_rect(inner: Rect, bounds: Rect) -> Rect {
        let clamped_x = inner.x.max(bounds.x).min(bounds.x + bounds.width - inner.width);
        let clamped_y = inner.y.max(bounds.y).min(bounds.y + bounds.height - inner.height);
        Rect {
            x: clamped_x,
            y: clamped_y,
            width: inner.width,
            height: inner.height,
        }
    }
}
