use gpui::*;

use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SplitButtonType {
    #[default]
    Filled,
    Tonal,
}

#[derive(Clone, Debug)]
pub struct SplitButtonItem {
    pub id: String,
    pub icon: Option<String>,
    pub text: String,
}

/// Split button offering a primary action alongside a dropdown menu (port of `SplitButton.qml`).
#[derive(Clone)]
pub struct SplitButton {
    pub button_type: SplitButtonType,
    pub disabled: bool,
    pub expanded: bool,
    pub menu_on_top: bool,
    pub fallback_icon: String,
    pub fallback_text: String,
    pub min_left_width: f32,
    pub active_item: Option<SplitButtonItem>,
    pub menu_items: Vec<SplitButtonItem>,
    pub colours: Option<CaelestiaColours>,
}

impl SplitButton {
    pub fn new(fallback_icon: impl Into<String>, fallback_text: impl Into<String>) -> Self {
        Self {
            button_type: SplitButtonType::Filled,
            disabled: false,
            expanded: false,
            menu_on_top: false,
            fallback_icon: fallback_icon.into(),
            fallback_text: fallback_text.into(),
            min_left_width: 0.0,
            active_item: None,
            menu_items: Vec::new(),
            colours: None,
        }
    }

    pub fn button_type(mut self, button_type: SplitButtonType) -> Self {
        self.button_type = button_type;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn menu_on_top(mut self, on_top: bool) -> Self {
        self.menu_on_top = on_top;
        self
    }

    pub fn min_left_width(mut self, width: f32) -> Self {
        self.min_left_width = width;
        self
    }

    pub fn active_item(mut self, item: Option<SplitButtonItem>) -> Self {
        self.active_item = item;
        self
    }

    pub fn menu_items(mut self, items: Vec<SplitButtonItem>) -> Self {
        self.menu_items = items;
        self
    }

    pub fn with_colours(mut self, colours: CaelestiaColours) -> Self {
        self.colours = Some(colours);
        self
    }

    pub fn current_icon(&self) -> &str {
        if let Some(ref item) = self.active_item {
            if let Some(ref icon) = item.icon {
                return icon.as_str();
            }
        }
        &self.fallback_icon
    }

    pub fn current_text(&self) -> &str {
        if let Some(ref item) = self.active_item {
            &item.text
        } else {
            &self.fallback_text
        }
    }
}

impl IntoElement for SplitButton {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let icon = self.current_icon().to_string();
        let text = self.current_text().to_string();
        let colours = self.colours.unwrap_or_default();

        let (bg_u32, fg_u32) = if self.disabled {
            (colours.surface_container_highest, colours.outline)
        } else {
            match self.button_type {
                SplitButtonType::Filled => (colours.primary, colours.on_primary),
                SplitButtonType::Tonal => (colours.secondary_container, colours.on_secondary_container),
            }
        };

        let bg_color = rgba(bg_u32);
        let fg_color = rgba(fg_u32);

        let horizontal_padding = PADDING_MEDIUM;
        let spacing = (SPACING_EXTRA_SMALL / 2.0).floor();
        let height = 36.0;

        let outer_radius = height / 2.0;
        let inner_radius = ROUNDING_MEDIUM / 2.0;
        let expand_inner_radius = if self.expanded {
            outer_radius
        } else {
            inner_radius
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(spacing))
            // Left Action Segment
            .child(
                div()
                    .id("split_btn_action")
                    .h(px(height))
                    .min_w(px(self.min_left_width))
                    .px(px(horizontal_padding))
                    .bg(bg_color)
                    .rounded_l(px(outer_radius))
                    .rounded_r(px(inner_radius))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .gap(px(SPACING_SMALL))
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.92))
                    .active(|s| s.opacity(0.88))
                    .child(
                        MaterialIcon::new(icon)
                            .text_size(px(ICON_SIZE_SMALL))
                            .text_color(fg_color),
                    )
                    .child(
                        StyledText::new(text)
                            .text_color(fg_color)
                            .font_weight(FontWeight::MEDIUM),
                    ),
            )
            // Right Dropdown Segment
            .child(
                div()
                    .id("split_btn_expand")
                    .size(px(height))
                    .bg(bg_color)
                    .rounded_l(px(expand_inner_radius))
                    .rounded_r(px(outer_radius))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.92))
                    .active(|s| s.opacity(0.88))
                    .child(
                        MaterialIcon::new("expand_more")
                            .text_size(px(ICON_SIZE_SMALL))
                            .text_color(fg_color),
                    ),
            )
    }
}
