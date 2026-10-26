//! WirelessPassword Popout
//! Parity with `shell/modules/bar/popouts/WirelessPassword.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::icon_text_button::IconTextButton;
use crate::components::controls::password_mask::AnimatedPasswordMask;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug)]
pub struct WirelessPasswordState {
    pub ssid: String,
    pub password_length: usize,
    pub is_connecting: bool,
    pub has_error: bool,
    pub error_message: Option<String>,
}

impl Default for WirelessPasswordState {
    fn default() -> Self {
        Self {
            ssid: "Home-5G".into(),
            password_length: 8,
            is_connecting: false,
            has_error: false,
            error_message: None,
        }
    }
}

pub struct WirelessPasswordPopout;

impl WirelessPasswordPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &WirelessPasswordState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &WirelessPasswordState,
        c: &CaelestiaColours,
    ) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .items_center()
            .w(px(380.0))
            .p(px(PADDING_LARGE))
            .rounded(px(ROUNDING_LARGE))
            .bg(rgba(c.surface_container))
            .gap(px(SPACING_MEDIUM))
            // Lock Hero Icon
            .child(
                div()
                    .size(px(56.0))
                    .rounded_full()
                    .bg(rgba(c.secondary_container))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        MaterialIcon::new("lock")
                            .text_size(px(ICON_SIZE_LARGE))
                            .text_color(rgba(c.on_secondary_container)),
                    ),
            )
            // Title & Network Name
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(SPACING_EXTRA_SMALL))
                    .child(
                        StyledText::new("Enter password")
                            .text_size(px(FONT_SIZE_BODY_LARGE))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(c.on_surface)),
                    )
                    .child(
                        StyledText::new(format!("Network: {}", state.ssid))
                            .text_size(px(FONT_SIZE_BODY_SMALL))
                            .text_color(rgba(c.outline)),
                    ),
            )
            // Error / Status Message
            .child(
                if state.has_error {
                    div().child(
                        StyledText::new(
                            state
                                .error_message
                                .clone()
                                .unwrap_or_else(|| "Connection failed. Please check your password and try again.".into()),
                        )
                        .text_size(px(FONT_SIZE_BODY_SMALL))
                        .text_color(rgba(c.error)),
                    )
                } else if state.is_connecting {
                    div().child(
                        StyledText::new("Connecting...")
                            .text_size(px(FONT_SIZE_BODY_SMALL))
                            .text_color(rgba(c.on_surface_variant)),
                    )
                } else {
                    div()
                },
            )
            // Password Mask Container
            .child(
                div()
                    .w_full()
                    .h(px(48.0))
                    .px(px(PADDING_MEDIUM))
                    .rounded(px(ROUNDING_MEDIUM))
                    .bg(rgba(c.surface_container_highest))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        AnimatedPasswordMask::new("*".repeat(state.password_length))
                            .with_colours(c.clone())
                            .into_element(),
                    ),
            )
            // Actions Row (Cancel / Connect)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_end()
                    .w_full()
                    .gap(px(SPACING_SMALL))
                    .child(
                        IconTextButton::new("close", "Cancel")
                            .with_colours(c.clone())
                            .into_element(),
                    )
                    .child(
                        IconTextButton::new("wifi", if state.is_connecting { "Connecting..." } else { "Connect" })
                            .disabled(state.is_connecting)
                            .with_colours(c.clone())
                            .into_element(),
                    ),
            );

        PopoutWrapper::render("Connect to Wi-Fi", "wifi_password", center_x, content, c)
    }
}
