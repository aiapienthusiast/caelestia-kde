//! Spatial/anchor animation primitive matching `shell/components/AnchorAnim.qml`.

use std::time::Duration;
use super::anim::{
    CubicBezier, EASING_DEFAULT_SPATIAL, EASING_EMPHASIZED, EASING_FAST_SPATIAL, EASING_SLOW_SPATIAL,
    EASING_STANDARD,
};
use crate::domain::tokens::*;

/// Anchor animation type enum mirroring `AnchorAnim.qml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AnchorAnimType {
    StandardSmall = 0,
    Standard = 1,
    StandardLarge = 2,
    StandardExtraLarge = 3,
    EmphasizedSmall = 4,
    Emphasized = 5,
    EmphasizedLarge = 6,
    EmphasizedExtraLarge = 7,
    FastSpatial = 8,
    DefaultSpatial = 9,
    SlowSpatial = 10,
}

impl Default for AnchorAnimType {
    fn default() -> Self {
        Self::DefaultSpatial
    }
}

impl AnchorAnimType {
    pub fn duration_ms(&self) -> u64 {
        match self {
            Self::FastSpatial => ANIM_DURATION_EXPRESSIVE_FAST_SPATIAL,
            Self::DefaultSpatial => ANIM_DURATION_EXPRESSIVE_DEFAULT_SPATIAL,
            Self::SlowSpatial => ANIM_DURATION_EXPRESSIVE_SLOW_SPATIAL,
            _ => {
                let idx = (*self as u8) % 4;
                match idx {
                    0 => ANIM_DURATION_SMALL,
                    1 => ANIM_DURATION_NORMAL,
                    2 => ANIM_DURATION_LARGE,
                    _ => ANIM_DURATION_EXTRA_LARGE,
                }
            }
        }
    }

    pub fn duration(&self) -> Duration {
        Duration::from_millis(self.duration_ms())
    }

    pub fn easing(&self) -> CubicBezier {
        match self {
            Self::FastSpatial => EASING_FAST_SPATIAL,
            Self::DefaultSpatial => EASING_DEFAULT_SPATIAL,
            Self::SlowSpatial => EASING_SLOW_SPATIAL,
            Self::EmphasizedSmall
            | Self::Emphasized
            | Self::EmphasizedLarge
            | Self::EmphasizedExtraLarge => EASING_EMPHASIZED,
            _ => EASING_STANDARD,
        }
    }
}

/// Anchor animation controller.
#[derive(Debug, Clone, PartialEq)]
pub struct AnchorAnim {
    pub anim_type: AnchorAnimType,
    pub duration: Duration,
    pub easing: CubicBezier,
    pub from_offset: f32,
    pub to_offset: f32,
}

impl Default for AnchorAnim {
    fn default() -> Self {
        Self::new(AnchorAnimType::default())
    }
}

impl AnchorAnim {
    pub fn new(anim_type: AnchorAnimType) -> Self {
        Self {
            anim_type,
            duration: anim_type.duration(),
            easing: anim_type.easing(),
            from_offset: 0.0,
            to_offset: 0.0,
        }
    }

    pub fn with_range(mut self, from_offset: f32, to_offset: f32) -> Self {
        self.from_offset = from_offset;
        self.to_offset = to_offset;
        self
    }

    pub fn progress(&self, elapsed: Duration) -> f32 {
        if self.duration.is_zero() {
            return 1.0;
        }
        let t = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        self.easing.sample(t)
    }

    pub fn offset_at(&self, elapsed: Duration) -> f32 {
        let p = self.progress(elapsed);
        self.from_offset + (self.to_offset - self.from_offset) * p
    }
}
