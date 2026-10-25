//! Animation primitive matching `shell/components/Anim.qml`.
//! Provides Material 3 animation durations and cubic-bezier easing curves.

use std::time::Duration;
use crate::domain::tokens::*;

/// Material 3 animation curve representing a cubic bezier.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBezier {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl CubicBezier {
    pub const fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// Evaluates the cubic bezier curve at parameter t in [0.0, 1.0].
    /// Solves for x(t) and returns y(t) approximation.
    pub fn sample(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        // Newton-Raphson approximation for parameter solving
        let mut u = t;
        for _ in 0..5 {
            let x = 3.0 * (1.0 - u) * (1.0 - u) * u * self.x1
                + 3.0 * (1.0 - u) * u * u * self.x2
                + u * u * u;
            let dx = 3.0 * (1.0 - u) * (1.0 - u) * self.x1
                + 6.0 * (1.0 - u) * u * (self.x2 - self.x1)
                + 3.0 * u * u * (1.0 - self.x2);
            if dx.abs() < 1e-5 {
                break;
            }
            u = (u - (x - t) / dx).clamp(0.0, 1.0);
        }

        3.0 * (1.0 - u) * (1.0 - u) * u * self.y1
            + 3.0 * (1.0 - u) * u * u * self.y2
            + u * u * u
    }
}

/// Standard Material 3 cubic bezier easing curves.
pub const EASING_STANDARD: CubicBezier = CubicBezier::new(0.2, 0.0, 0.0, 1.0);
pub const EASING_EMPHASIZED: CubicBezier = CubicBezier::new(0.05, 0.7, 0.1, 1.0);
pub const EASING_FAST_SPATIAL: CubicBezier = CubicBezier::new(0.4, 0.0, 0.2, 1.0);
pub const EASING_DEFAULT_SPATIAL: CubicBezier = CubicBezier::new(0.2, 0.0, 0.0, 1.0);
pub const EASING_SLOW_SPATIAL: CubicBezier = CubicBezier::new(0.2, 0.0, 0.0, 1.0);
pub const EASING_FAST_EFFECTS: CubicBezier = CubicBezier::new(0.4, 0.0, 1.0, 1.0);
pub const EASING_DEFAULT_EFFECTS: CubicBezier = CubicBezier::new(0.4, 0.0, 0.2, 1.0);
pub const EASING_SLOW_EFFECTS: CubicBezier = CubicBezier::new(0.0, 0.0, 0.2, 1.0);

/// Animation type enum mirroring `Anim.qml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AnimType {
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
    FastEffects = 11,
    DefaultEffects = 12,
    SlowEffects = 13,
}

impl Default for AnimType {
    fn default() -> Self {
        Self::DefaultSpatial
    }
}

impl AnimType {
    /// Returns duration matching token specifications in milliseconds.
    pub fn duration_ms(&self) -> u64 {
        match self {
            Self::FastSpatial => ANIM_DURATION_EXPRESSIVE_FAST_SPATIAL,
            Self::DefaultSpatial => ANIM_DURATION_EXPRESSIVE_DEFAULT_SPATIAL,
            Self::SlowSpatial => ANIM_DURATION_EXPRESSIVE_SLOW_SPATIAL,
            Self::FastEffects => ANIM_DURATION_EXPRESSIVE_FAST_EFFECTS,
            Self::DefaultEffects => ANIM_DURATION_EXPRESSIVE_DEFAULT_EFFECTS,
            Self::SlowEffects => ANIM_DURATION_EXPRESSIVE_SLOW_EFFECTS,
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

    /// Returns the animation duration as `std::time::Duration`.
    pub fn duration(&self) -> Duration {
        Duration::from_millis(self.duration_ms())
    }

    /// Returns the easing curve for this animation type.
    pub fn easing(&self) -> CubicBezier {
        match self {
            Self::FastSpatial => EASING_FAST_SPATIAL,
            Self::DefaultSpatial => EASING_DEFAULT_SPATIAL,
            Self::SlowSpatial => EASING_SLOW_SPATIAL,
            Self::FastEffects => EASING_FAST_EFFECTS,
            Self::DefaultEffects => EASING_DEFAULT_EFFECTS,
            Self::SlowEffects => EASING_SLOW_EFFECTS,
            Self::EmphasizedSmall
            | Self::Emphasized
            | Self::EmphasizedLarge
            | Self::EmphasizedExtraLarge => EASING_EMPHASIZED,
            _ => EASING_STANDARD,
        }
    }
}

/// Concrete animation controller holding duration, curve, and value bounds.
#[derive(Debug, Clone, PartialEq)]
pub struct Anim {
    pub anim_type: AnimType,
    pub duration: Duration,
    pub easing: CubicBezier,
    pub from: f32,
    pub to: f32,
}

impl Default for Anim {
    fn default() -> Self {
        Self::new(AnimType::default())
    }
}

impl Anim {
    pub fn new(anim_type: AnimType) -> Self {
        Self {
            anim_type,
            duration: anim_type.duration(),
            easing: anim_type.easing(),
            from: 0.0,
            to: 1.0,
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_easing(mut self, easing: CubicBezier) -> Self {
        self.easing = easing;
        self
    }

    pub fn with_range(mut self, from: f32, to: f32) -> Self {
        self.from = from;
        self.to = to;
        self
    }

    /// Calculates progress factor in [0.0, 1.0] given elapsed duration.
    pub fn progress(&self, elapsed: Duration) -> f32 {
        if self.duration.is_zero() {
            return 1.0;
        }
        let t = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        self.easing.sample(t)
    }

    /// Computes the animated scalar value given elapsed duration.
    pub fn value_at(&self, elapsed: Duration) -> f32 {
        let p = self.progress(elapsed);
        self.from + (self.to - self.from) * p
    }
}
