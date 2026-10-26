//! Shortcuts Popout
//! Parity with `shell/modules/bar/popouts/Shortcuts.qml` and `Toggles.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::icon_button::IconButton;
use crate::components::controls::list_row::ListRow;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString, Styled};

#[derive(Clone, Debug)]
pub struct QuickToggleItem {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct ShortcutsPopoutState {
    pub quick_toggles: Vec<QuickToggleItem>,
    pub current_page: usize,
    pub total_pages: usize,
}

impl Default for ShortcutsPopoutState {
    fn default() -> Self {
        Self {
            quick_toggles: vec![
                QuickToggleItem {
                    id: "wifi".into(),
                    name: "Wi-Fi".into(),
                    icon: "wifi".into(),
                    active: true,
                },
                QuickToggleItem {
                    id: "bluetooth".into(),
                    name: "Bluetooth".into(),
                    icon: "bluetooth".into(),
                    active: true,
                },
                QuickToggleItem {
                    id: "mic".into(),
                    name: "Microphone".into(),
                    icon: "mic".into(),
                    active: false,
                },
                QuickToggleItem {
                    id: "dnd".into(),
                    name: "Do Not Disturb".into(),
                    icon: "notifications_off".into(),
                    active: false,
                },
                QuickToggleItem {
                    id: "nightlight".into(),
                    name: "Night Light".into(),
                    icon: "bedtime".into(),
                    active: false,
                },
                QuickToggleItem {
                    id: "vpn".into(),
                    name: "VPN".into(),
                    icon: "vpn_key".into(),
                    active: false,
                },
                QuickToggleItem {
                    id: "gamemode".into(),
                    name: "Game Mode".into(),
                    icon: "gamepad".into(),
                    active: false,
                },
                QuickToggleItem {
                    id: "hotspot".into(),
                    name: "Hotspot".into(),
                    icon: "wifi_tethering".into(),
                    active: false,
                },
            ],
            current_page: 0,
            total_pages: 1,
        }
    }
}

pub struct ShortcutsPopout;

impl ShortcutsPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &ShortcutsPopoutState::default(), c)
    }

    pub fn render_with_state(center_x: f32, state: &ShortcutsPopoutState, c: &CaelestiaColours) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .w(px(360.0))
            .gap(px(SPACING_MEDIUM))
            // Quick Toggles Card (Header + Grid)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(PADDING_MEDIUM))
                    .rounded(px(ROUNDING_LARGE))
                    .bg(rgba(c.surface_container))
                    .gap(px(SPACING_MEDIUM))
                    // Title Bar
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                StyledText::new("Quick Toggles")
                                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgba(c.on_surface)),
                            )
                            .child(
                                IconButton::new("tune")
                                    .with_colours(c.clone())
                                    .into_element(),
                            ),
                    )
                    // Grid of 2 columns
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(SPACING_SMALL))
                            .children(state.quick_toggles.iter().map(|item| {
                                let bg_col = if item.active {
                                    rgba(c.secondary_container)
                                } else {
                                    rgba(c.surface_container_highest)
                                };
                                let fg_col = if item.active {
                                    rgba(c.on_secondary_container)
                                } else {
                                    rgba(c.on_surface_variant)
                                };

                                div()
                                    .id(SharedString::from(item.id.clone()))
                                    .w(px(160.0))
                                    .h(px(48.0))
                                    .px(px(PADDING_MEDIUM))
                                    .rounded(px(ROUNDING_LARGE))
                                    .bg(bg_col)
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(SPACING_SMALL))
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.9))
                                    .child(
                                        MaterialIcon::new(item.icon.clone())
                                            .text_size(px(ICON_SIZE_MEDIUM))
                                            .text_color(fg_col),
                                    )
                                    .child(
                                        StyledText::new(item.name.clone())
                                            .text_size(px(FONT_SIZE_BODY_SMALL))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(fg_col),
                                    )
                            })),
                    ),
            )
            // Keyboard Shortcuts Reference Card
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(PADDING_MEDIUM))
                    .rounded(px(ROUNDING_LARGE))
                    .bg(rgba(c.surface_container))
                    .gap(px(SPACING_SMALL))
                    .child(
                        StyledText::new("Keybindings")
                            .text_size(px(FONT_SIZE_BODY_MEDIUM))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(c.on_surface)),
                    )
                    .child(ListRow::render(
                        "Application Launcher",
                        Some("Super + Space"),
                        Some("apps"),
                        None,
                        false,
                        false,
                        false,
                        c,
                    ))
                    .child(ListRow::render(
                        "Overview Workspace",
                        Some("Super + Tab"),
                        Some("view_carousel"),
                        None,
                        false,
                        false,
                        false,
                        c,
                    ))
                    .child(ListRow::render(
                        "Terminal Console",
                        Some("Super + Return"),
                        Some("terminal"),
                        None,
                        false,
                        false,
                        false,
                        c,
                    )),
            );

        PopoutWrapper::render("Quick Toggles & Shortcuts", "keyboard", center_x, content, c)
    }
}
