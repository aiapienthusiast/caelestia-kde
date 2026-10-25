//! Colorizer effect adjusting brightness and tinting
//! Parity with `shell/components/effects/Colouriser.qml`.

use gpui::*;

/// Parameters for colorization and brightness matching
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColouriserEffect {
    pub colorization: f32,
    pub target_color: Rgba,
    pub brightness: f32,
}

impl Default for ColouriserEffect {
    fn default() -> Self {
        Self {
            colorization: 1.0,
            target_color: rgb(0x000000),
            brightness: 0.5,
        }
    }
}

impl ColouriserEffect {
    pub fn new(target_color: Rgba) -> Self {
        // Luminance approximation for brightness adjustment: 1.0 - lightness
        let luminance = 0.299 * target_color.r + 0.587 * target_color.g + 0.114 * target_color.b;
        let brightness = 1.0 - luminance;

        Self {
            colorization: 1.0,
            target_color,
            brightness,
        }
    }

    pub fn with_colorization(mut self, factor: f32) -> Self {
        self.colorization = factor.clamp(0.0, 1.0);
        self
    }
}

/// Element applying a colorizing tint over its contents
#[derive(IntoElement)]
pub struct ColouriserView {
    effect: ColouriserEffect,
    child: AnyElement,
}

impl ColouriserView {
    pub fn new(target_color: Rgba, child: impl IntoElement) -> Self {
        Self {
            effect: ColouriserEffect::new(target_color),
            child: child.into_any_element(),
        }
    }
}

impl RenderOnce for ColouriserView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .text_color(self.effect.target_color)
            .child(self.child)
    }
}
