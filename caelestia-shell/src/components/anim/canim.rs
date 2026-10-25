//! Color animation primitive matching `shell/components/CAnim.qml`.
//! Suppresses animation duration during theme switches to prevent smearing.

use std::time::Duration;
use gpui::Rgba;
use super::anim::{CubicBezier, EASING_SLOW_EFFECTS};
use crate::domain::tokens::ANIM_DURATION_EXPRESSIVE_SLOW_EFFECTS;

/// Color animation controller with theme switch suppression.
#[derive(Debug, Clone, PartialEq)]
pub struct CAnim {
    pub switching_theme: bool,
    pub duration: Duration,
    pub easing: CubicBezier,
    pub from: Rgba,
    pub to: Rgba,
}

impl Default for CAnim {
    fn default() -> Self {
        Self::new(false)
    }
}

impl CAnim {
    pub fn new(switching_theme: bool) -> Self {
        let duration = if switching_theme {
            Duration::ZERO
        } else {
            Duration::from_millis(ANIM_DURATION_EXPRESSIVE_SLOW_EFFECTS)
        };

        Self {
            switching_theme,
            duration,
            easing: EASING_SLOW_EFFECTS,
            from: Rgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
            to: Rgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        }
    }

    pub fn with_range(mut self, from: Rgba, to: Rgba) -> Self {
        self.from = from;
        self.to = to;
        self
    }

    pub fn set_switching_theme(&mut self, switching: bool) {
        self.switching_theme = switching;
        self.duration = if switching {
            Duration::ZERO
        } else {
            Duration::from_millis(ANIM_DURATION_EXPRESSIVE_SLOW_EFFECTS)
        };
    }

    pub fn progress(&self, elapsed: Duration) -> f32 {
        if self.duration.is_zero() {
            return 1.0;
        }
        let t = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        self.easing.sample(t)
    }

    pub fn color_at(&self, elapsed: Duration) -> Rgba {
        let p = self.progress(elapsed);
        Rgba {
            r: self.from.r + (self.to.r - self.from.r) * p,
            g: self.from.g + (self.to.g - self.from.g) * p,
            b: self.from.b + (self.to.b - self.from.b) * p,
            a: self.from.a + (self.to.a - self.from.a) * p,
        }
    }
}
