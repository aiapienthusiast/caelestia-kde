//! Bluetooth Popout
//! 100% Parity with `shell/modules/bar/popouts/Bluetooth.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::icon_text_button::IconTextButton;
use crate::components::controls::toggle_row::ToggleRow;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, Styled};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BluetoothDeviceItem {
    pub mac: String,
    pub name: String,
    pub icon: String,
    pub connected: bool,
    pub paired: bool,
    pub battery: Option<u8>,
}

impl BluetoothDeviceItem {
    pub fn new(mac: impl Into<String>, name: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            mac: mac.into(),
            name: name.into(),
            icon: icon.into(),
            connected: false,
            paired: true,
            battery: None,
        }
    }

    pub fn connected(mut self, connected: bool) -> Self {
        self.connected = connected;
        self
    }

    pub fn battery(mut self, battery: u8) -> Self {
        self.battery = Some(battery);
        self
    }
}

/// State of bluetooth adapter and remote devices (port of `Quickshell.Bluetooth`).
#[derive(Clone, Debug, PartialEq)]
pub struct BluetoothPopoutState {
    pub enabled: bool,
    pub discovering: bool,
    pub devices: Vec<BluetoothDeviceItem>,
}

impl Default for BluetoothPopoutState {
    fn default() -> Self {
        Self {
            enabled: true,
            discovering: false,
            devices: vec![
                BluetoothDeviceItem::new("00:1B:66:81:49:12", "Sony WH-1000XM5", "headphones")
                    .connected(true)
                    .battery(80),
                BluetoothDeviceItem::new("CC:98:8B:11:02:44", "MX Master 3S", "mouse")
                    .connected(true)
                    .battery(95),
                BluetoothDeviceItem::new("AA:22:33:44:55:66", "Keychron K2 Pro", "keyboard")
                    .connected(false),
            ],
        }
    }
}

pub struct BluetoothPopout;

impl BluetoothPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &BluetoothPopoutState::default(), c)
    }

    /// Renders the bluetooth popout with full parity to Bluetooth.qml.
    pub fn render_with_state(center_x: f32, state: &BluetoothPopoutState, c: &CaelestiaColours) -> Div {
        let width_px = 300.0;

        let mut content = div()
            .flex()
            .flex_col()
            .w(px(width_px))
            .gap(px(SPACING_SMALL));

        // 1. Header: "Bluetooth"
        content = content.child(
            StyledText::new("Bluetooth")
                .text_size(px(FONT_SIZE_NORMAL))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.on_surface)),
        );

        // 2. Main Card
        let mut card = div()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL));

        // Enabled Toggle
        card = card.child(ToggleRow::render(
            "Enabled",
            None::<&str>,
            None,
            state.enabled,
            false,
            false,
            false,
            c,
        ));

        // Discovering Toggle
        card = card.child(ToggleRow::render(
            "Discovering",
            None::<&str>,
            None,
            state.discovering,
            false,
            false,
            !state.enabled,
            c,
        ));

        // Device Count Subtitle
        let dev_count = state.devices.len();
        let connected_count = state.devices.iter().filter(|d| d.connected).count();
        let mut count_msg = format!("{} device{} available", dev_count, if dev_count == 1 { "" } else { "s" });
        if connected_count > 0 {
            count_msg.push_str(&format!(" ({} connected)", connected_count));
        }

        card = card.child(
            div()
                .pt(px(SPACING_SMALL))
                .child(
                    StyledText::new(count_msg)
                        .text_size(px(FONT_SIZE_SMALL))
                        .text_color(rgba(c.on_surface_variant)),
                ),
        );

        // Sorted Devices List: connected first, then paired, then name
        let mut sorted_devices = state.devices.clone();
        sorted_devices.sort_by(|a, b| {
            b.connected
                .cmp(&a.connected)
                .then_with(|| b.paired.cmp(&a.paired))
                .then_with(|| a.name.cmp(&b.name))
        });

        for dev in &sorted_devices {
            let mut status_line = if dev.connected {
                "Connected".to_string()
            } else if dev.paired {
                "Paired".to_string()
            } else {
                "Available".to_string()
            };

            if let Some(bat) = dev.battery {
                status_line.push_str(&format!(" • {}%", bat));
            }

            let row = div()
                .w_full()
                .p(px(PADDING_SMALL))
                .rounded(px(ROUNDING_SMALL))
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .cursor(CursorStyle::PointingHand)
                .hover(|s| s.bg(rgba(c.surface_container_high)))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(SPACING_SMALL))
                        .child(
                            MaterialIcon::new(dev.icon.clone())
                                .text_size(px(ICON_SIZE_SMALL))
                                .text_color(rgba(if dev.connected { c.primary } else { c.on_surface_variant })),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    StyledText::new(dev.name.clone())
                                        .text_size(px(FONT_SIZE_NORMAL))
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgba(c.on_surface)),
                                )
                                .child(
                                    StyledText::new(status_line)
                                        .text_size(px(FONT_SIZE_SMALL))
                                        .text_color(rgba(c.on_surface_variant)),
                                ),
                        ),
                );

            card = card.child(row);
        }

        content = content.child(card);

        // 3. Open Settings Button
        let mut btn_style = ButtonStyle::default();
        btn_style.button_type = ButtonType::Tonal;

        content = content.child(
            div()
                .mt(px(SPACING_SMALL))
                .child(IconTextButton::render("settings", "Open settings", &btn_style, c)),
        );

        PopoutWrapper::render("Bluetooth", "bluetooth", center_x, content, c)
    }
}
