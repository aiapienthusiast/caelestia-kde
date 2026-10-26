//! DockContext Popout
//! Parity with `shell/modules/bar/popouts/DockContext.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::icon_text_button::IconTextButton;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

#[derive(Clone, Debug)]
pub struct DockContextState {
    pub app_id: String,
    pub app_name: String,
    pub is_pinned: bool,
    pub can_close: bool,
}

impl Default for DockContextState {
    fn default() -> Self {
        Self {
            app_id: "org.kde.dolphin".into(),
            app_name: "File Manager".into(),
            is_pinned: true,
            can_close: true,
        }
    }
}

pub struct DockContextPopout;

impl DockContextPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &DockContextState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &DockContextState,
        c: &CaelestiaColours,
    ) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .w(px(220.0))
            .gap(px(SPACING_SMALL))
            // Action Rows Card
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(PADDING_EXTRA_SMALL))
                    .rounded(px(ROUNDING_MEDIUM))
                    .bg(rgba(c.surface_container))
                    .gap(px(2.0))
                    .child(ListRow::render(
                        if state.is_pinned {
                            "Unpin from dock"
                        } else {
                            "Pin to dock"
                        },
                        None::<&str>,
                        Some(if state.is_pinned {
                            "keep_off"
                        } else {
                            "push_pin"
                        }),
                        None,
                        false,
                        false,
                        false,
                        c,
                    ))
                    .child(ListRow::render(
                        "Open new window",
                        None::<&str>,
                        Some("add_to_photos"),
                        None,
                        false,
                        false,
                        false,
                        c,
                    ))
                    .child(ListRow::render(
                        "Minimize Window",
                        None::<&str>,
                        Some("minimize"),
                        None,
                        false,
                        false,
                        false,
                        c,
                    )),
            )
            // Close Action Button
            .child(
                IconTextButton::new("close", "Close")
                    .with_colours(c.clone())
                    .into_element(),
            );

        PopoutWrapper::render("Application Options", "apps", center_x, content, c)
    }
}
