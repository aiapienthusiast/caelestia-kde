//! PeripheralBattery Popout
//! Parity with `shell/modules/bar/popouts/PeripheralBattery.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug)]
pub struct PeripheralDevice {
    pub name: String,
    pub device_type: String,
    pub percentage: u8,
    pub is_charging: bool,
}

#[derive(Clone, Debug)]
pub struct PeripheralBatteryState {
    pub devices: Vec<PeripheralDevice>,
}

impl Default for PeripheralBatteryState {
    fn default() -> Self {
        Self {
            devices: vec![
                PeripheralDevice {
                    name: "Wireless Mouse".into(),
                    device_type: "mouse".into(),
                    percentage: 85,
                    is_charging: false,
                },
                PeripheralDevice {
                    name: "Mechanical Keyboard".into(),
                    device_type: "keyboard".into(),
                    percentage: 92,
                    is_charging: false,
                },
                PeripheralDevice {
                    name: "Bluetooth Headset".into(),
                    device_type: "headphones".into(),
                    percentage: 60,
                    is_charging: true,
                },
            ],
        }
    }
}

pub struct PeripheralBatteryPopout;

impl PeripheralBatteryPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &PeripheralBatteryState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &PeripheralBatteryState,
        c: &CaelestiaColours,
    ) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .w(px(300.0))
            .p(px(PADDING_MEDIUM))
            .rounded(px(ROUNDING_LARGE))
            .bg(rgba(c.surface_container))
            .gap(px(SPACING_SMALL))
            .children(if state.devices.is_empty() {
                vec![div().p(px(PADDING_SMALL)).child(
                    StyledText::new("No connected peripherals")
                        .text_size(px(FONT_SIZE_BODY_SMALL))
                        .text_color(rgba(c.on_surface_variant)),
                )]
            } else {
                state
                    .devices
                    .iter()
                    .map(|dev| {
                        let icon_name = match dev.device_type.as_str() {
                            "mouse" => "mouse",
                            "keyboard" => "keyboard",
                            "headphones" => "headphones",
                            "gamepad" => "sports_esports",
                            "stylus" => "stylus",
                            "speaker" => "speaker",
                            "smartphone" => "smartphone",
                            _ => "battery_full",
                        };

                        let battery_col = if dev.percentage <= 20 {
                            c.error
                        } else if dev.percentage <= 50 {
                            c.secondary
                        } else {
                            c.primary
                        };

                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p(px(PADDING_SMALL))
                            .rounded(px(ROUNDING_MEDIUM))
                            .bg(rgba(c.surface_container_low))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(SPACING_SMALL))
                                    .child(
                                        MaterialIcon::new(icon_name)
                                            .text_size(px(ICON_SIZE_MEDIUM))
                                            .text_color(rgba(c.on_surface)),
                                    )
                                    .child(
                                        StyledText::new(dev.name.clone())
                                            .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgba(c.on_surface)),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(SPACING_EXTRA_SMALL))
                                    .child(
                                        StyledText::new(format!("{}%", dev.percentage))
                                            .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgba(battery_col)),
                                    )
                                    .child(if dev.is_charging {
                                        div().child(
                                            MaterialIcon::new("bolt")
                                                .text_size(px(ICON_SIZE_SMALL))
                                                .text_color(rgba(c.primary)),
                                        )
                                    } else {
                                        div()
                                    }),
                            )
                    })
                    .collect()
            });

        PopoutWrapper::render("Peripherals", "devices", center_x, content, c)
    }
}
