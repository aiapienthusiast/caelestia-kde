use gpui::*;

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ScrollBarOrientation {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ScrollBarPolicy {
    #[default]
    AsNeeded,
    AlwaysOn,
    AlwaysOff,
}

/// Overlay scrollbar track and thumb indicator (port of `StyledScrollBar.qml`).
#[derive(Clone)]
pub struct StyledScrollBar {
    pub orientation: ScrollBarOrientation,
    pub policy: ScrollBarPolicy,
    pub position: f32, // 0.0 to 1.0
    pub size: f32,     // 0.0 to 1.0 (visible fraction)
    pub hovered: bool,
    pub pressed: bool,
    pub active: bool,
    pub colours: Option<CaelestiaColours>,
}

impl StyledScrollBar {
    pub fn new(orientation: ScrollBarOrientation) -> Self {
        Self {
            orientation,
            policy: ScrollBarPolicy::AsNeeded,
            position: 0.0,
            size: 0.25,
            hovered: false,
            pressed: false,
            active: false,
            colours: None,
        }
    }

    pub fn vertical() -> Self {
        Self::new(ScrollBarOrientation::Vertical)
    }

    pub fn horizontal() -> Self {
        Self::new(ScrollBarOrientation::Horizontal)
    }

    pub fn policy(mut self, policy: ScrollBarPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn position(mut self, position: f32) -> Self {
        self.position = position.clamp(0.0, 1.0);
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size.clamp(0.0, 1.0);
        self
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn with_colours(mut self, colours: CaelestiaColours) -> Self {
        self.colours = Some(colours);
        self
    }

    pub fn opacity(&self) -> f32 {
        if self.policy == ScrollBarPolicy::AlwaysOff || self.size >= 1.0 {
            0.0
        } else if self.pressed {
            1.0
        } else if self.hovered {
            0.8
        } else if self.policy == ScrollBarPolicy::AlwaysOn || self.active {
            0.6
        } else {
            0.0
        }
    }
}

impl IntoElement for StyledScrollBar {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let opacity = self.opacity();
        let colours = self.colours.unwrap_or_default();
        let thumb_color = rgba(colours.secondary);

        let track_thickness = PADDING_EXTRA_SMALL;
        let is_vert = self.orientation == ScrollBarOrientation::Vertical;

        let thumb_pos_pct = self.position * 100.0;
        let thumb_size_pct = (self.size * 100.0).clamp(10.0, 100.0);

        if is_vert {
            div()
                .w(px(track_thickness))
                .h_full()
                .flex()
                .flex_col()
                .child(
                    div()
                        .w_full()
                        .h(relative(thumb_size_pct / 100.0))
                        .mt(relative(thumb_pos_pct / 100.0))
                        .rounded_full()
                        .bg(thumb_color)
                        .opacity(opacity),
                )
        } else {
            div()
                .h(px(track_thickness))
                .w_full()
                .flex()
                .flex_row()
                .child(
                    div()
                        .h_full()
                        .w(relative(thumb_size_pct / 100.0))
                        .ml(relative(thumb_pos_pct / 100.0))
                        .rounded_full()
                        .bg(thumb_color)
                        .opacity(opacity),
                )
        }
    }
}
