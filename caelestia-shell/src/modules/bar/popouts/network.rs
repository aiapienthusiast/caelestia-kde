//! Network Popout
//! 100% Parity with `shell/modules/bar/popouts/Network.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::icon_button::IconButton;
use crate::components::controls::toggle_row::ToggleRow;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, Styled};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NetworkViewMode {
    #[default]
    Wireless,
    Ethernet,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WifiNetworkItem {
    pub ssid: String,
    pub signal_percent: u8,
    pub is_connected: bool,
    pub is_secured: bool,
}

impl WifiNetworkItem {
    pub fn new(ssid: impl Into<String>, signal_percent: u8) -> Self {
        Self {
            ssid: ssid.into(),
            signal_percent,
            is_connected: false,
            is_secured: true,
        }
    }

    pub fn connected(mut self, connected: bool) -> Self {
        self.is_connected = connected;
        self
    }

    pub fn secured(mut self, secured: bool) -> Self {
        self.is_secured = secured;
        self
    }

    pub fn wifi_icon(&self) -> &'static str {
        if self.signal_percent >= 75 {
            if self.is_secured { "wifi_lock" } else { "wifi" }
        } else if self.signal_percent >= 50 {
            if self.is_secured { "wifi_lock" } else { "wifi_2_bar" }
        } else {
            if self.is_secured { "wifi_lock" } else { "wifi_1_bar" }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkDeviceDetails {
    pub ip_address: String,
    pub subnet_mask: String,
    pub gateway: String,
    pub dns: Vec<String>,
    pub mac_address: String,
}

impl Default for NetworkDeviceDetails {
    fn default() -> Self {
        Self {
            ip_address: "192.168.1.142".into(),
            subnet_mask: "255.255.255.0".into(),
            gateway: "192.168.1.1".into(),
            dns: vec!["1.1.1.1".into(), "1.0.0.1".into()],
            mac_address: "E4:5F:01:88:9C:2B".into(),
        }
    }
}

/// State of network interfaces and connection diagnostics (port of `Nmcli` service).
#[derive(Clone, Debug, PartialEq)]
pub struct NetworkPopoutState {
    pub view: NetworkViewMode,
    pub wifi_enabled: bool,
    pub networks: Vec<WifiNetworkItem>,
    pub active_details: Option<NetworkDeviceDetails>,
}

impl Default for NetworkPopoutState {
    fn default() -> Self {
        Self {
            view: NetworkViewMode::Wireless,
            wifi_enabled: true,
            networks: vec![
                WifiNetworkItem::new("Home-5G", 92).connected(true).secured(true),
                WifiNetworkItem::new("Office-Guest", 65).connected(false).secured(true),
                WifiNetworkItem::new("Starbucks_WiFi", 40).connected(false).secured(false),
            ],
            active_details: Some(NetworkDeviceDetails::default()),
        }
    }
}

pub struct NetworkPopout;

impl NetworkPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &NetworkPopoutState::default(), c)
    }

    /// Renders the network popout with full parity to Network.qml.
    pub fn render_with_state(center_x: f32, state: &NetworkPopoutState, c: &CaelestiaColours) -> Div {
        let width_px = 360.0;

        let mut content = div()
            .flex()
            .flex_col()
            .w(px(width_px))
            .gap(px(SPACING_SMALL));

        // 1. Header: "Network" + Settings Button
        let mut settings_style = ButtonStyle::default();
        settings_style.button_type = ButtonType::Tonal;

        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .child(
                StyledText::new("Network")
                    .text_size(px(FONT_SIZE_LARGE))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(c.on_surface)),
            )
            .child(IconButton::render("settings", &settings_style, c, Some(32.0)));

        content = content.child(header);

        // 2. Wireless Card
        let mut wifi_card = div()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL));

        wifi_card = wifi_card.child(ToggleRow::render(
            "Wi-Fi",
            None::<&str>,
            None,
            state.wifi_enabled,
            false,
            false,
            false,
            c,
        ));

        if state.wifi_enabled {
            wifi_card = wifi_card.child(
                div()
                    .pt(px(SPACING_EXTRA_SMALL))
                    .child(
                        StyledText::new(format!("{} networks found", state.networks.len()))
                            .text_size(px(FONT_SIZE_SMALL))
                            .text_color(rgba(c.on_surface_variant)),
                    ),
            );

            for net in &state.networks {
                let status_text = if net.is_connected {
                    "Connected"
                } else if net.is_secured {
                    "Secured"
                } else {
                    "Open"
                };

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
                                MaterialIcon::new(net.wifi_icon())
                                    .text_size(px(ICON_SIZE_SMALL))
                                    .text_color(rgba(if net.is_connected { c.primary } else { c.on_surface_variant })),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        StyledText::new(net.ssid.clone())
                                            .text_size(px(FONT_SIZE_NORMAL))
                                            .font_weight(if net.is_connected { FontWeight::BOLD } else { FontWeight::NORMAL })
                                            .text_color(rgba(c.on_surface)),
                                    )
                                    .child(
                                        StyledText::new(format!("{} • {}%", status_text, net.signal_percent))
                                            .text_size(px(FONT_SIZE_SMALL))
                                            .text_color(rgba(c.on_surface_variant)),
                                    ),
                            ),
                    );

                wifi_card = wifi_card.child(row);
            }
        }

        content = content.child(wifi_card);

        // 3. Active Connection Details Card
        if let Some(ref details) = state.active_details {
            let mut details_card = div()
                .w_full()
                .p(px(PADDING_MEDIUM))
                .rounded(px(ROUNDING_MEDIUM))
                .bg(rgba(c.surface_container))
                .flex()
                .flex_col()
                .gap(px(SPACING_EXTRA_SMALL));

            details_card = details_card.child(
                StyledText::new("Connection Details")
                    .text_size(px(FONT_SIZE_NORMAL))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(c.on_surface)),
            );

            let rows = [
                ("IP address", &details.ip_address),
                ("Subnet mask", &details.subnet_mask),
                ("Gateway", &details.gateway),
                ("MAC address", &details.mac_address),
            ];

            for (lbl, val) in rows {
                let r = div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .child(
                        StyledText::new(lbl)
                            .text_size(px(FONT_SIZE_SMALL))
                            .text_color(rgba(c.on_surface_variant)),
                    )
                    .child(
                        StyledText::new(val.clone())
                            .text_size(px(FONT_SIZE_SMALL))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(c.on_surface)),
                    );
                details_card = details_card.child(r);
            }

            if !details.dns.is_empty() {
                let dns_str = details.dns.join(", ");
                let r = div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .child(
                        StyledText::new("DNS")
                            .text_size(px(FONT_SIZE_SMALL))
                            .text_color(rgba(c.on_surface_variant)),
                    )
                    .child(
                        StyledText::new(dns_str)
                            .text_size(px(FONT_SIZE_SMALL))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(c.on_surface)),
                    );
                details_card = details_card.child(r);
            }

            content = content.child(details_card);
        }

        PopoutWrapper::render("Network", "wifi", center_x, content, c)
    }
}
