use gpui::*;

use crate::components::controls::split_button::{SplitButton, SplitButtonItem, SplitButtonType};
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

/// Row container pairing a setting or action label with a SplitButton (port of `SplitButtonRow.qml`).
#[derive(Clone)]
pub struct SplitButtonRow {
    pub label: String,
    pub enabled: bool,
    pub expanded_z: i32,
    pub split_button: SplitButton,
    pub colours: Option<CaelestiaColours>,
}

impl SplitButtonRow {
    pub fn new(
        label: impl Into<String>,
        fallback_icon: impl Into<String>,
        fallback_text: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            enabled: true,
            expanded_z: 100,
            split_button: SplitButton::new(fallback_icon, fallback_text),
            colours: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self.split_button.disabled = !enabled;
        self
    }

    pub fn button_type(mut self, button_type: SplitButtonType) -> Self {
        self.split_button = self.split_button.button_type(button_type);
        self
    }

    pub fn menu_items(mut self, items: Vec<SplitButtonItem>) -> Self {
        self.split_button = self.split_button.menu_items(items);
        self
    }

    pub fn active_item(mut self, item: Option<SplitButtonItem>) -> Self {
        self.split_button = self.split_button.active_item(item);
        self
    }

    pub fn with_colours(mut self, colours: CaelestiaColours) -> Self {
        self.colours = Some(colours);
        self
    }
}

impl IntoElement for SplitButtonRow {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let colours = self.colours.unwrap_or_default();

        let bg_color = rgba(colours.surface_container_high);
        let text_color = rgba(if self.enabled {
            colours.on_surface
        } else {
            colours.on_surface_variant
        });

        let radius = ROUNDING_LARGE;
        let padding = PADDING_LARGE;
        let spacing = SPACING_MEDIUM;

        div()
            .w_full()
            .p(px(padding))
            .bg(bg_color)
            .rounded(px(radius))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .gap(px(spacing))
            .opacity(if self.enabled { 1.0 } else { 0.5 })
            .child(
                StyledText::new(self.label)
                    .text_color(text_color)
                    .font_weight(FontWeight::NORMAL),
            )
            .child(self.split_button)
    }
}
