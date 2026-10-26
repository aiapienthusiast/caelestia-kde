//! Greeter Widget
//! Parity with `shell/modules/bar/components/Greeter.qml`.

use chrono::{Local, Timelike};
use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, Styled};

pub struct GreeterWidget;

impl GreeterWidget {
    /// Generates time-of-day greeting matching Greeter.qml:38-59.
    pub fn greeting(username: &str) -> String {
        let hour = Local::now().hour();
        let formatted_user = if username.is_empty() {
            "User".to_string()
        } else {
            let mut c = username.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        };

        let msg = if hour >= 6 && hour < 12 {
            "Good Morning"
        } else if hour >= 12 && hour < 18 {
            "Good Afternoon"
        } else if hour >= 18 && hour < 22 {
            "Good Evening"
        } else {
            "Good Night"
        };

        format!("{}, {}!", msg, formatted_user)
    }

    /// Renders user avatar and greeting badge pill.
    pub fn render(username: &str, c: &CaelestiaColours) -> Div {
        Self::render_with_state(username, true, false, false, c)
    }

    /// Renders with explicit layout and hover/pressed states.
    pub fn render_with_state(
        username: &str,
        is_horizontal: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let greeting_text = Self::greeting(username);
        let padding_h = PADDING_MEDIUM;
        let padding_v = PADDING_EXTRA_SMALL;

        let container = div()
            .relative()
            .flex()
            .items_center()
            .px(px(padding_h))
            .py(px(padding_v))
            .rounded_full()
            .bg(rgba(c.surface_container))
            .cursor(CursorStyle::PointingHand)
            .child(StateLayer::overlay(
                is_hovered,
                is_pressed,
                false,
                ROUNDING_FULL,
                c.primary,
            ));

        let content = if is_horizontal {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(SPACING_SMALL))
                .child(
                    div()
                        .w(px(24.0))
                        .h(px(24.0))
                        .rounded_full()
                        .bg(rgba(c.primary))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            MaterialIcon::with_size("person", IconSize::Small, c.on_primary),
                        ),
                )
                .child(
                    StyledText::new(greeting_text)
                        .text_size(px(FONT_SIZE_BODY_MEDIUM))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(c.on_surface)),
                )
        } else {
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(SPACING_EXTRA_SMALL))
                .child(
                    div()
                        .w(px(24.0))
                        .h(px(24.0))
                        .rounded_full()
                        .bg(rgba(c.primary))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            MaterialIcon::with_size("person", IconSize::Small, c.on_primary),
                        ),
                )
        };

        container.child(content)
    }
}

