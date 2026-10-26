//! Greeter Popout
//! Parity with `shell/modules/bar/popouts/Greeter.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug)]
pub struct GreeterPopoutState {
    pub username: String,
    pub real_name: String,
    pub host: String,
    pub avatar_url: Option<String>,
}

impl Default for GreeterPopoutState {
    fn default() -> Self {
        Self {
            username: "ladybug-me".into(),
            real_name: "Ladybug".into(),
            host: "Arch Linux • Plasma 6 Wayland".into(),
            avatar_url: None,
        }
    }
}

pub struct GreeterPopout;

impl GreeterPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &GreeterPopoutState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &GreeterPopoutState,
        c: &CaelestiaColours,
    ) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .gap(px(SPACING_SMALL))
            // Profile Hero Card
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .p(px(PADDING_MEDIUM))
                    .rounded(px(ROUNDING_LARGE))
                    .bg(rgba(c.surface_container))
                    .gap(px(SPACING_MEDIUM))
                    .child(
                        div()
                            .size(px(52.0))
                            .rounded_full()
                            .bg(rgba(c.primary_container))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                MaterialIcon::new("account_circle")
                                    .text_size(px(ICON_SIZE_LARGE))
                                    .text_color(rgba(c.on_primary_container)),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                StyledText::new(state.real_name.clone())
                                    .text_size(px(FONT_SIZE_BODY_LARGE))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgba(c.on_surface)),
                            )
                            .child(
                                StyledText::new(format!("@{}", state.username))
                                    .text_size(px(FONT_SIZE_BODY_SMALL))
                                    .text_color(rgba(c.outline)),
                            )
                            .child(
                                StyledText::new(state.host.clone())
                                    .text_size(px(FONT_SIZE_LABEL_SMALL))
                                    .text_color(rgba(c.on_surface_variant)),
                            ),
                    ),
            )
            // Session Controls
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(SPACING_EXTRA_SMALL))
                    .child(ListRow::render(
                        "Lock Screen",
                        Some("Lock current session"),
                        Some("lock"),
                        None,
                        false,
                        false,
                        false,
                        c,
                    ))
                    .child(ListRow::render(
                        "Suspend System",
                        Some("Sleep to low-power state"),
                        Some("bedtime"),
                        None,
                        false,
                        false,
                        false,
                        c,
                    ))
                    .child(ListRow::render(
                        "Log Out",
                        Some("End active desktop session"),
                        Some("logout"),
                        None,
                        false,
                        false,
                        false,
                        c,
                    )),
            );

        PopoutWrapper::render("User Session", "account_circle", center_x, content, c)
    }
}
