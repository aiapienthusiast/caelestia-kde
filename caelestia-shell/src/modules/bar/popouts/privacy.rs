//! Privacy Popout
//! 100% Parity with `shell/modules/bar/popouts/Privacy.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

/// State of active media capturing applications (port of `shell/services/Privacy.qml`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PrivacyState {
    pub microphone_apps: Vec<String>,
    pub video_apps: Vec<String>,
}

impl PrivacyState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mic_app(mut self, app: impl Into<String>) -> Self {
        self.microphone_apps.push(app.into());
        self
    }

    pub fn with_video_app(mut self, app: impl Into<String>) -> Self {
        self.video_apps.push(app.into());
        self
    }

    pub fn in_use(&self) -> bool {
        !self.microphone_apps.is_empty() || !self.video_apps.is_empty()
    }
}

pub struct PrivacyPopout;

impl PrivacyPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &PrivacyState::default(), c)
    }

    /// Renders the privacy popout with full parity to Privacy.qml.
    pub fn render_with_state(center_x: f32, state: &PrivacyState, c: &CaelestiaColours) -> Div {
        let mut content = div()
            .flex()
            .flex_col()
            .w(px(300.0))
            .gap(px(SPACING_SMALL));

        // Title: "Privacy"
        content = content.child(
            StyledText::new("Privacy")
                .text_size(px(FONT_SIZE_NORMAL))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.on_surface)),
        );

        let categories: [(&'static str, &'static str, &Vec<String>); 2] = [
            ("mic", "Microphone", &state.microphone_apps),
            ("videocam", "Video capture", &state.video_apps),
        ];

        let mut has_entries = false;

        for (icon, label, apps) in categories {
            if apps.is_empty() {
                continue;
            }
            has_entries = true;

            let mut card = div()
                .w_full()
                .p(px(PADDING_MEDIUM))
                .rounded(px(ROUNDING_MEDIUM))
                .bg(rgba(c.surface_container))
                .flex()
                .flex_col()
                .gap(px(SPACING_SMALL));

            // Category Header: [Icon] Label
            let header = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(SPACING_SMALL))
                .child(
                    MaterialIcon::new(icon)
                        .text_size(px(ICON_SIZE_SMALL))
                        .text_color(rgba(c.primary)),
                )
                .child(
                    StyledText::new(label)
                        .text_size(px(FONT_SIZE_NORMAL))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(c.on_surface)),
                );

            card = card.child(header);

            // Active Application Items
            for app_name in apps {
                card = card.child(
                    div()
                        .pl(px(ICON_SIZE_SMALL + SPACING_SMALL))
                        .child(
                            StyledText::new(app_name.clone())
                                .text_size(px(FONT_SIZE_SMALL))
                                .text_color(rgba(c.on_surface_variant)),
                        ),
                );
            }

            content = content.child(card);
        }

        // Idle state message: "Nothing is using the microphone or capturing the screen."
        if !has_entries {
            content = content.child(
                div()
                    .p(px(PADDING_MEDIUM))
                    .child(
                        StyledText::new("Nothing is using the microphone or capturing the screen.")
                            .text_size(px(FONT_SIZE_SMALL))
                            .text_color(rgba(c.on_surface_variant)),
                    ),
            );
        }

        PopoutWrapper::render("Privacy", "security", center_x, content, c)
    }
}
