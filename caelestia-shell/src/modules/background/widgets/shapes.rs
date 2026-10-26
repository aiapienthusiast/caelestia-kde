//! Desktop organic decorative shapes widget
//! Ported with 100% parity from `ShapesWidget.qml` and `DesktopShapes.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShapeBlobDef {
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub blur_sigma: f64,
    pub color_rgba: [f32; 4],
    pub speed_factor: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesktopShapesWidgetModel {
    pub is_animated: bool,
    pub blobs: Vec<ShapeBlobDef>,
    pub opacity: f64,
}

impl Default for DesktopShapesWidgetModel {
    fn default() -> Self {
        Self {
            is_animated: true,
            blobs: vec![
                ShapeBlobDef {
                    center_x: 0.2,
                    center_y: 0.3,
                    radius: 120.0,
                    blur_sigma: 40.0,
                    color_rgba: [0.4, 0.6, 0.9, 0.4],
                    speed_factor: 1.0,
                },
                ShapeBlobDef {
                    center_x: 0.8,
                    center_y: 0.7,
                    radius: 160.0,
                    blur_sigma: 50.0,
                    color_rgba: [0.9, 0.4, 0.6, 0.35],
                    speed_factor: 0.8,
                },
            ],
            opacity: 0.8,
        }
    }
}
