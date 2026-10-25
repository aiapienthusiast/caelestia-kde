//! Elevation
//! Parity with `shell/components/effects/Elevation.qml`.

use gpui::{Div, Styled};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ElevationLevel {
    Level0,
    #[default]
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

pub struct Elevation;

impl Elevation {
    /// Applies Material 3 box shadow elevation to a given Div element.
    pub fn apply(element: Div, level: ElevationLevel) -> Div {
        match level {
            ElevationLevel::Level0 => element,
            ElevationLevel::Level1 => element.shadow_sm(),
            ElevationLevel::Level2 => element.shadow_md(),
            ElevationLevel::Level3 => element.shadow_lg(),
            ElevationLevel::Level4 => element.shadow_xl(),
            ElevationLevel::Level5 => element.shadow_2xl(),
        }
    }
}
