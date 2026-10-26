//! Circular selection geometry and aspect calculations
//! Ported with 100% parity from `CircleSelectionDetails.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CircleSelectionDetails {
    pub center_x: f32,
    pub center_y: f32,
    pub radius: f32,
}

impl CircleSelectionDetails {
    pub fn from_points(start_x: f32, start_y: f32, current_x: f32, current_y: f32) -> Self {
        let center_x = (start_x + current_x) / 2.0;
        let center_y = (start_y + current_y) / 2.0;
        let dx = current_x - start_x;
        let dy = current_y - start_y;
        let radius = (dx * dx + dy * dy).sqrt() / 2.0;

        Self {
            center_x,
            center_y,
            radius,
        }
    }

    pub fn diameter(&self) -> f32 {
        self.radius * 2.0
    }

    pub fn bounding_rect(&self) -> (f32, f32, f32, f32) {
        (
            self.center_x - self.radius,
            self.center_y - self.radius,
            self.diameter(),
            self.diameter(),
        )
    }

    pub fn formatted_dimensions(&self) -> String {
        let d = self.diameter().round() as u32;
        format!("{d} × {d} px (r={:.0})", self.radius)
    }
}
