//! Calendar month slide animation state matching `shell/components/CalendarMonthSlide.qml`.

use std::time::Duration;
use chrono::NaiveDate;
use crate::components::anim::{Anim, AnimType};
use crate::domain::tokens::PADDING_EXTRA_LARGE;

/// Tracks sliding horizontal transition and opacity during month switches.
#[derive(Debug, Clone, PartialEq)]
pub struct CalendarMonthSlide {
    pub shown_date: NaiveDate,
    pub prev_date: NaiveDate,
    pub offset: f32,
    pub anim_translate: f32,
    pub anim_opacity: f32,
    pub translate_anim: Anim,
    pub opacity_anim: Anim,
    pub elapsed: Duration,
    pub is_animating: bool,
}

impl Default for CalendarMonthSlide {
    fn default() -> Self {
        let today = chrono::Local::now().date_naive();
        Self::new(today)
    }
}

impl CalendarMonthSlide {
    pub fn new(initial_date: NaiveDate) -> Self {
        Self {
            shown_date: initial_date,
            prev_date: initial_date,
            offset: PADDING_EXTRA_LARGE,
            anim_translate: 0.0,
            anim_opacity: 1.0,
            translate_anim: Anim::new(AnimType::DefaultSpatial),
            opacity_anim: Anim::new(AnimType::FastEffects),
            elapsed: Duration::ZERO,
            is_animating: false,
        }
    }

    /// Sets the offset in logical pixels for horizontal sliding.
    pub fn with_offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// Shifts or sets the target shown date. Starts horizontal slide animation.
    pub fn set_shown_date(&mut self, new_date: NaiveDate) {
        if self.shown_date == new_date {
            return;
        }

        let direction = if new_date > self.shown_date { 1.0 } else { -1.0 };
        let parked = self.offset * direction;

        self.prev_date = self.shown_date;
        self.shown_date = new_date;

        self.translate_anim = Anim::new(AnimType::DefaultSpatial).with_range(parked, 0.0);
        self.opacity_anim = Anim::new(AnimType::FastEffects).with_range(0.0, 1.0);
        self.elapsed = Duration::ZERO;
        self.is_animating = true;
        self.anim_translate = parked;
        self.anim_opacity = 0.0;
    }

    /// Advances the slide animation by `delta`.
    pub fn tick(&mut self, delta: Duration) {
        if !self.is_animating {
            return;
        }

        self.elapsed += delta;
        self.anim_translate = self.translate_anim.value_at(self.elapsed);
        self.anim_opacity = self.opacity_anim.value_at(self.elapsed);

        let max_duration = self.translate_anim.duration.max(self.opacity_anim.duration);
        if self.elapsed >= max_duration {
            self.anim_translate = 0.0;
            self.anim_opacity = 1.0;
            self.is_animating = false;
        }
    }
}
