//! Animated deferred loader matching `shell/components/AnimLoader.qml`.
//! Provides sequential fade-out, component swap, and fade-in transitions.

use std::time::Duration;
use gpui::*;
use super::anim::{Anim, AnimType};

/// State transition phase of the animated loader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoaderPhase {
    Idle,
    FadingOut,
    Swapping,
    FadingIn,
}

/// Animated loader controller and state machine.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimLoaderState {
    pub current_id: Option<String>,
    pub pending_id: Option<String>,
    pub is_complete: bool,
    pub out_anim: Anim,
    pub in_anim: Anim,
    pub phase: LoaderPhase,
    pub elapsed: Duration,
    pub opacity: f32,
}

impl Default for AnimLoaderState {
    fn default() -> Self {
        Self {
            current_id: None,
            pending_id: None,
            is_complete: false,
            out_anim: Anim::new(AnimType::FastEffects).with_range(1.0, 0.0),
            in_anim: Anim::new(AnimType::DefaultEffects).with_range(0.0, 1.0),
            phase: LoaderPhase::Idle,
            elapsed: Duration::ZERO,
            opacity: 1.0,
        }
    }
}

impl AnimLoaderState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the initial source component without animation.
    pub fn init_source(&mut self, source_id: impl Into<String>) {
        let id = source_id.into();
        self.current_id = Some(id.clone());
        self.pending_id = Some(id);
        self.is_complete = true;
        self.phase = LoaderPhase::Idle;
        self.opacity = 1.0;
    }

    /// Requests transition to a new component. If already complete, starts fade out.
    pub fn set_source(&mut self, source_id: impl Into<String>) {
        let new_id = source_id.into();
        if self.current_id.as_deref() == Some(&new_id) && self.pending_id.as_deref() == Some(&new_id) {
            return;
        }

        self.pending_id = Some(new_id);
        if self.is_complete {
            self.phase = LoaderPhase::FadingOut;
            self.elapsed = Duration::ZERO;
            self.out_anim.from = self.opacity;
            self.out_anim.to = 0.0;
        } else {
            self.current_id = self.pending_id.clone();
            self.is_complete = true;
            self.opacity = 1.0;
            self.phase = LoaderPhase::Idle;
        }
    }

    /// Advances the animation by `delta`.
    pub fn tick(&mut self, delta: Duration) {
        match self.phase {
            LoaderPhase::Idle => {}
            LoaderPhase::FadingOut => {
                self.elapsed += delta;
                self.opacity = self.out_anim.value_at(self.elapsed);
                if self.elapsed >= self.out_anim.duration {
                    self.phase = LoaderPhase::Swapping;
                    self.current_id = self.pending_id.clone();
                    self.phase = LoaderPhase::FadingIn;
                    self.elapsed = Duration::ZERO;
                    self.in_anim.from = 0.0;
                    self.in_anim.to = 1.0;
                    self.opacity = 0.0;
                }
            }
            LoaderPhase::Swapping => {
                self.current_id = self.pending_id.clone();
                self.phase = LoaderPhase::FadingIn;
                self.elapsed = Duration::ZERO;
                self.opacity = 0.0;
            }
            LoaderPhase::FadingIn => {
                self.elapsed += delta;
                self.opacity = self.in_anim.value_at(self.elapsed);
                if self.elapsed >= self.in_anim.duration {
                    self.phase = LoaderPhase::Idle;
                    self.opacity = 1.0;
                }
            }
        }
    }
}

/// GPUI component wrapper rendering children with the animated loader's opacity.
#[derive(IntoElement)]
pub struct AnimLoader {
    opacity: f32,
    child: AnyElement,
}

impl AnimLoader {
    pub fn new(opacity: f32, child: impl IntoElement) -> Self {
        Self {
            opacity: opacity.clamp(0.0, 1.0),
            child: child.into_any_element(),
        }
    }
}

impl RenderOnce for AnimLoader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .opacity(self.opacity)
            .child(self.child)
    }
}
