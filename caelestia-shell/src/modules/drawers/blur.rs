//! Compositor frosted blur regions and corner clipping geometries
//! Ported with 100% parity from `shell/modules/drawers/blur/`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BlurRegionRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub corner_radius: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlurMaskConfig {
    pub blur_radius: f64,
    pub noise_intensity: f64,
    pub tint_rgba: [f32; 4],
    pub border_width: f64,
    pub border_rgba: [f32; 4],
}

impl Default for BlurMaskConfig {
    fn default() -> Self {
        Self {
            blur_radius: 32.0,
            noise_intensity: 0.04,
            tint_rgba: [0.08, 0.08, 0.12, 0.75],
            border_width: 1.0,
            border_rgba: [1.0, 1.0, 1.0, 0.12],
        }
    }
}

/// Blur bodies coordinator calculating the union of active drawer blur geometries
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BlurBodiesModel {
    pub regions: Vec<BlurRegionRect>,
    pub mask: BlurMaskConfig,
}

impl BlurBodiesModel {
    pub fn new(mask: BlurMaskConfig) -> Self {
        Self {
            regions: Vec::new(),
            mask,
        }
    }

    pub fn add_region(&mut self, region: BlurRegionRect) {
        self.regions.push(region);
    }

    pub fn clear(&mut self) {
        self.regions.clear();
    }

    /// Calculate combined bounding box of all blur regions
    pub fn bounding_box(&self) -> Option<BlurRegionRect> {
        if self.regions.is_empty() {
            return None;
        }

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for r in &self.regions {
            min_x = min_x.min(r.x);
            min_y = min_y.min(r.y);
            max_x = max_x.max(r.x + r.width);
            max_y = max_y.max(r.y + r.height);
        }

        Some(BlurRegionRect {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
            corner_radius: 0.0,
        })
    }
}
