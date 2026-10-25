//! NavPane component for Nexus Control Center.
//! Parity with `shell/modules/nexus/NavPane.qml` and `NavLocations.qml`.

use super::drawer::NexusDrawer;
use super::state::{NexusCategory, NexusState};
use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, MouseButton, MouseDownEvent, Styled};

pub struct NavPane;

impl NavPane {
    /// Renders the left navigation pane of Nexus.
    pub fn render(
        state: &NexusState,
        c: &CaelestiaColours,
        cx: &mut gpui::Context<NexusDrawer>,
    ) -> Div {
        let filtered = state.filtered_categories();

        div()
            .w(px(280.0))
            .h_full()
            .flex()
            .flex_col()
            .gap(px(SPACING_LARGE))
            .p(px(PADDING_LARGE))
            .border_r_1()
            .border_color(rgba(c.outline_variant))
            // Search Bar
            .child(
                div()
                    .w_full()
                    .h(px(42.0))
                    .rounded(px(ROUNDING_FULL))
                    .bg(rgba(c.surface_container_lowest))
                    .border_1()
                    .border_color(rgba(c.outline_variant))
                    .px(px(PADDING_LARGE))
                    .flex()
                    .items_center()
                    .gap(px(SPACING_SMALL))
                    .child(MaterialIcon::with_size("🔍", IconSize::Small, c.on_surface_variant))
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_NORMAL))
                            .text_color(rgba(c.on_surface_variant))
                            .child(if state.search_query.is_empty() {
                                "Search settings".to_string()
                            } else {
                                state.search_query.clone()
                            }),
                    ),
            )
            // Categories List
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(px(SPACING_EXTRA_SMALL))
                    .overflow_hidden()
                    .children(filtered.iter().map(|&(idx, cat)| {
                        Self::render_category_item(idx, cat, state, c, cx)
                    })),
            )
    }

    fn render_category_item(
        idx: usize,
        cat: &'static NexusCategory,
        state: &NexusState,
        c: &CaelestiaColours,
        cx: &mut gpui::Context<NexusDrawer>,
    ) -> Div {
        let is_selected = idx == state.selected_index;
        let is_hovered = state.hovered_category == Some(idx);

        // Connected corner rounding matching NavLocations.qml:60-64
        let radius = if is_selected {
            ROUNDING_EXTRA_LARGE_INCREASED
        } else {
            ROUNDING_MEDIUM
        };

        let bg_color = if is_selected {
            c.secondary_container
        } else if is_hovered {
            c.surface_container_high
        } else {
            0x00000000 // transparent
        };

        let label_color = if is_selected {
            c.on_secondary_container
        } else {
            c.on_surface
        };

        let desc_color = if is_selected {
            c.on_secondary_container
        } else {
            c.on_surface_variant
        };

        let icon_char = match cat.key {
            "appearance" => "🎨",
            "desktop" => "🖥️",
            "panels" => "📑",
            "network" => "📶",
            "bluetooth" => "ᛒ",
            "display" => "🖵",
            "audio" => "🔊",
            "power" => "🔋",
            "apps" => "📱",
            "services" => "✨",
            "about" => "ℹ️",
            _ => "⚙️",
        };

        div()
            .relative()
            .w_full()
            .h(px(52.0))
            .rounded(px(radius))
            .bg(rgba(bg_color))
            .px(px(PADDING_MEDIUM))
            .flex()
            .items_center()
            .gap(px(SPACING_NORMAL))
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |nexus, _event: &MouseDownEvent, _window, cx| {
                    nexus.state.select_category(idx);
                    cx.notify();
                }),
            )
            .child(StateLayer::overlay(
                is_hovered,
                false,
                false,
                radius,
                c.primary,
            ))
            .child(
                div()
                    .text_size(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon_char),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(TypographyScale::LabelLarge.font_size()))
                            .font_weight(if is_selected {
                                FontWeight::BOLD
                            } else {
                                FontWeight::MEDIUM
                            })
                            .text_color(rgba(label_color))
                            .child(cat.label),
                    )
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_SMALL))
                            .text_color(rgba(desc_color))
                            .child(cat.description),
                    ),
            )
    }
}
