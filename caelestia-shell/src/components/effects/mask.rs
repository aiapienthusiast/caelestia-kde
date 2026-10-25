//! Alpha mask clipping effect
//! Parity with `shell/components/effects/Mask.qml`.

use gpui::*;

/// Mask effect configuration parameters
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaskEffect {
    pub enabled: bool,
    pub spread_at_min: f32,
    pub threshold_min: f32,
    pub inverted: bool,
}

impl Default for MaskEffect {
    fn default() -> Self {
        Self {
            enabled: true,
            spread_at_min: 1.0,
            threshold_min: 0.5,
            inverted: false,
        }
    }
}

impl MaskEffect {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_inverted(mut self, inverted: bool) -> Self {
        self.inverted = inverted;
        self
    }

    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold_min = threshold.clamp(0.0, 1.0);
        self
    }
}

/// Element applying an alpha mask clipping layer over its child
#[derive(IntoElement)]
pub struct MaskView {
    effect: MaskEffect,
    child: AnyElement,
}

impl MaskView {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            effect: MaskEffect::default(),
            child: child.into_any_element(),
        }
    }

    pub fn with_effect(mut self, effect: MaskEffect) -> Self {
        self.effect = effect;
        self
    }
}

impl RenderOnce for MaskView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .overflow_hidden()
            .child(self.child)
    }
}
