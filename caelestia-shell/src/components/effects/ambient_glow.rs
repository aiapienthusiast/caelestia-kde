//! Ambient glow / bloom effect behind windows and panels
//! Parity with `shell/components/effects/AmbientGlow.qml`.

use gpui::prelude::FluentBuilder;
use gpui::*;

/// State model for ambient glow calculations
#[derive(Clone, Debug, PartialEq)]
pub struct AmbientGlowState {
    pub address: String,
    pub active_address: String,
    pub source: String,
    pub fallback_color: Rgba,
    pub glow_opacity: f32,
    pub blur_radius: f32,
    pub bloom_progress: f32,
    pub active: bool,
}

impl Default for AmbientGlowState {
    fn default() -> Self {
        Self {
            address: String::new(),
            active_address: String::new(),
            source: String::new(),
            fallback_color: rgb(0xd0bcff),
            glow_opacity: 0.45,
            blur_radius: 96.0,
            bloom_progress: 1.0,
            active: true,
        }
    }
}

impl AmbientGlowState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_address(mut self, address: impl Into<String>) -> Self {
        let addr = address.into();
        self.address = addr.clone();
        self.active_address = addr;
        self
    }

    pub fn with_fallback_color(mut self, color: Rgba) -> Self {
        self.fallback_color = color;
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.glow_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Triggers target address switch and bloom animation interpolation
    pub fn trigger_switch(&mut self, new_address: impl Into<String>) {
        let addr = new_address.into();
        if self.address != addr {
            self.address = addr.clone();
            self.active_address = addr;
            self.bloom_progress = 1.0;
        }
    }

    pub fn effective_glow_color(&self) -> Rgba {
        let alpha = self.glow_opacity * self.bloom_progress;
        Rgba {
            r: self.fallback_color.r,
            g: self.fallback_color.g,
            b: self.fallback_color.b,
            a: alpha.clamp(0.0, 1.0),
        }
    }
}

/// Element rendering an ambient diffused bloom behind its parent
#[derive(IntoElement)]
pub struct AmbientGlowView {
    state: AmbientGlowState,
}

impl AmbientGlowView {
    pub fn new(state: AmbientGlowState) -> Self {
        Self { state }
    }
}

impl RenderOnce for AmbientGlowView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_active = self.state.active && self.state.bloom_progress > 0.0;
        let glow_color = self.state.effective_glow_color();

        div()
            .w_full()
            .h_full()
            .when(is_active, |d| {
                d.rounded_2xl()
                    .bg(glow_color)
                    .shadow_lg()
            })
    }
}
