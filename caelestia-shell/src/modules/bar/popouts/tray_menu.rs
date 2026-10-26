//! TrayMenu Popout
//! Parity with `shell/modules/bar/popouts/TrayMenu.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug)]
pub struct TrayMenuItem {
    pub id: i32,
    pub label: String,
    pub icon: Option<String>,
    pub is_separator: bool,
    pub is_checkable: bool,
    pub is_checked: bool,
    pub is_enabled: bool,
    pub has_submenu: bool,
}

#[derive(Clone, Debug)]
pub struct TrayMenuState {
    pub title: String,
    pub items: Vec<TrayMenuItem>,
}

impl Default for TrayMenuState {
    fn default() -> Self {
        Self {
            title: "Application Menu".into(),
            items: vec![
                TrayMenuItem {
                    id: 1,
                    label: "Open Window".into(),
                    icon: Some("launch".into()),
                    is_separator: false,
                    is_checkable: false,
                    is_checked: false,
                    is_enabled: true,
                    has_submenu: false,
                },
                TrayMenuItem {
                    id: 2,
                    label: "Mute Audio".into(),
                    icon: Some("volume_off".into()),
                    is_separator: false,
                    is_checkable: true,
                    is_checked: false,
                    is_enabled: true,
                    has_submenu: false,
                },
                TrayMenuItem {
                    id: 3,
                    label: "".into(),
                    icon: None,
                    is_separator: true,
                    is_checkable: false,
                    is_checked: false,
                    is_enabled: true,
                    has_submenu: false,
                },
                TrayMenuItem {
                    id: 4,
                    label: "Settings".into(),
                    icon: Some("settings".into()),
                    is_separator: false,
                    is_checkable: false,
                    is_checked: false,
                    is_enabled: true,
                    has_submenu: true,
                },
                TrayMenuItem {
                    id: 5,
                    label: "Quit".into(),
                    icon: Some("exit_to_app".into()),
                    is_separator: false,
                    is_checkable: false,
                    is_checked: false,
                    is_enabled: true,
                    has_submenu: false,
                },
            ],
        }
    }
}

pub struct TrayMenuPopout;

impl TrayMenuPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &TrayMenuState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &TrayMenuState,
        c: &CaelestiaColours,
    ) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .w(px(220.0))
            .p(px(PADDING_EXTRA_SMALL))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .gap(px(2.0))
            .children(state.items.iter().map(|item| {
                if item.is_separator {
                    return div()
                        .w_full()
                        .h(px(1.0))
                        .my(px(PADDING_EXTRA_SMALL))
                        .bg(rgba(c.outline_variant));
                }

                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(PADDING_MEDIUM))
                    .py(px(PADDING_SMALL))
                    .rounded(px(ROUNDING_SMALL))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(c.surface_container_highest)))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(SPACING_SMALL))
                            .child(if let Some(ref icon) = item.icon {
                                MaterialIcon::new(icon.clone())
                                    .text_size(px(ICON_SIZE_SMALL))
                                    .text_color(rgba(c.on_surface_variant))
                            } else if item.is_checkable {
                                MaterialIcon::new(if item.is_checked {
                                    "check_box"
                                } else {
                                    "check_box_outline_blank"
                                })
                                .text_size(px(ICON_SIZE_SMALL))
                                .text_color(if item.is_checked {
                                    rgba(c.primary)
                                } else {
                                    rgba(c.outline)
                                })
                            } else {
                                MaterialIcon::new("chevron_right")
                                    .text_size(px(ICON_SIZE_SMALL))
                                    .text_color(gpui::transparent_black())
                            })
                            .child(
                                StyledText::new(item.label.clone())
                                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                    .font_weight(FontWeight::NORMAL)
                                    .text_color(if item.is_enabled {
                                        rgba(c.on_surface)
                                    } else {
                                        rgba(c.outline)
                                    }),
                            ),
                    )
                    .child(if item.has_submenu {
                        div().child(
                            MaterialIcon::new("chevron_right")
                                .text_size(px(ICON_SIZE_SMALL))
                                .text_color(rgba(c.outline)),
                        )
                    } else {
                        div()
                    })
            }));

        PopoutWrapper::render(&state.title, "menu", center_x, content, c)
    }
}
