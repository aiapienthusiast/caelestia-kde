//! Material 3 Toggle Button matching `shell/components/controls/ToggleButton.qml`.
//! Binary state button with active accent fill, radius morphing, and icon/label layout.

use gpui::*;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

/// Toggle Button styling and state model.
#[derive(Clone, Debug, PartialEq)]
pub struct ToggleButton {
    pub toggled: bool,
    pub icon: Option<String>,
    pub label: Option<String>,
    pub accent: String,
    pub icon_size: f32,
    pub horizontal_padding: f32,
    pub vertical_padding: f32,
    pub explicit_width: Option<f32>,
    pub hovered: bool,
    pub pressed: bool,
    pub disabled: bool,
}

impl Default for ToggleButton {
    fn default() -> Self {
        Self {
            toggled: false,
            icon: None,
            label: None,
            accent: "Secondary".to_string(),
            icon_size: ICON_SIZE_MEDIUM,
            horizontal_padding: PADDING_LARGE,
            vertical_padding: PADDING_MEDIUM,
            explicit_width: None,
            hovered: false,
            pressed: false,
            disabled: false,
        }
    }
}

impl ToggleButton {
    pub fn new(toggled: bool) -> Self {
        Self {
            toggled,
            ..Default::default()
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_accent(mut self, accent: impl Into<String>) -> Self {
        self.accent = accent.into();
        self
    }

    pub fn with_explicit_width(mut self, width: f32) -> Self {
        self.explicit_width = Some(width);
        self
    }

    /// Calculates corner radius matching ToggleButton.qml:31.
    pub fn corner_radius(&self) -> f32 {
        if self.toggled || self.pressed {
            ROUNDING_MEDIUM
        } else {
            ROUNDING_FULL
        }
    }

    /// Resolves background and foreground colors from theme based on toggled state and accent.
    pub fn resolve_colors(&self, c: &CaelestiaColours) -> (Rgba, Rgba) {
        let is_primary = self.accent.eq_ignore_ascii_case("primary");
        if self.toggled {
            let bg = if is_primary { c.primary } else { c.secondary };
            let fg = if is_primary { c.on_primary } else { c.on_secondary };
            (rgba(bg), rgba(fg))
        } else {
            let bg = if is_primary { c.primary_container } else { c.secondary_container };
            let fg = if is_primary { c.on_primary_container } else { c.on_secondary_container };
            (rgba(bg), rgba(fg))
        }
    }
}

impl IntoElement for ToggleButton {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let radius = self.corner_radius();
        let colours = CaelestiaColours::default();
        let (bg, fg) = self.resolve_colors(&colours);

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(SPACING_MEDIUM));

        if let Some(ref icon) = self.icon {
            row = row.child(MaterialIcon::new(icon.clone()).text_size(px(self.icon_size)));
        }

        if let Some(ref label) = self.label {
            row = row.child(
                StyledText::new(label.clone())
                    .text_size(px(FONT_SIZE_NORMAL))
                    .text_color(fg),
            );
        }

        let mut btn = div()
            .px(px(self.horizontal_padding))
            .py(px(self.vertical_padding))
            .rounded(px(radius))
            .bg(bg)
            .text_color(fg)
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand)
            .child(row);

        if let Some(w) = self.explicit_width {
            btn = btn.w(px(w));
        }

        btn
    }
}
