//! Detailed Network Configuration Sections
//! Parity with `shell/modules/nexus/common/NetworkList.qml`, `EthernetSection.qml`, and `Ipv4ConfigSection.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::network::AccessPoint;
use gpui::{div, prelude::*, px, rgb, rgba, CursorStyle, Div, Styled};

/// Ranked and Filtered Wi-Fi Network Access Point List
#[derive(Clone, Debug, Default)]
pub struct NetworkList {
    pub filter: String,
    pub limit: usize,
    pub connecting_ssid: Option<String>,
}

impl NetworkList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rank_and_sort<'a>(&self, aps: &'a [AccessPoint]) -> Vec<&'a AccessPoint> {
        let mut list: Vec<&'a AccessPoint> = aps
            .iter()
            .filter(|ap| {
                if self.filter.is_empty() {
                    true
                } else {
                    ap.ssid.to_lowercase().contains(&self.filter.to_lowercase())
                }
            })
            .collect();

        // Sort: active/in_use first, then connecting, then by signal strength desc
        let connecting = self.connecting_ssid.as_deref().unwrap_or("");
        list.sort_by(|a, b| {
            let rank_a = if a.in_use {
                0
            } else if a.ssid == connecting {
                1
            } else {
                2
            };
            let rank_b = if b.in_use {
                0
            } else if b.ssid == connecting {
                1
            } else {
                2
            };

            rank_a.cmp(&rank_b).then_with(|| b.signal.cmp(&a.signal))
        });

        if self.limit > 0 && list.len() > self.limit {
            list.truncate(self.limit);
        }

        list
    }

    pub fn render(&self, aps: &[AccessPoint], wifi_enabled: bool, colours: &CaelestiaColours) -> Div {
        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .bg(rgba((colours.surface_container << 8) | 0xFF))
            .rounded(px(ROUNDING_MEDIUM))
            .p(px(PADDING_MEDIUM))
            .gap(px(SPACING_SMALL));

        if !wifi_enabled {
            return root.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .p(px(8.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .text_color(rgb(colours.on_surface_variant))
                            .child("signal_wifi_off"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(colours.on_surface_variant))
                            .child("Wi-Fi is currently disabled"),
                    ),
            );
        }

        let sorted = self.rank_and_sort(aps);

        if sorted.is_empty() {
            root = root.child(
                div()
                    .p(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .child("No Wi-Fi networks found"),
            );
        } else {
            for ap in sorted {
                let is_connecting = self.connecting_ssid.as_deref() == Some(&ap.ssid);
                let wifi_icon = if ap.signal >= 75 {
                    "signal_wifi_4_bar"
                } else if ap.signal >= 50 {
                    "network_wifi_3_bar"
                } else if ap.signal >= 25 {
                    "network_wifi_2_bar"
                } else {
                    "network_wifi_1_bar"
                };

                let row = div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .p(px(8.0))
                    .rounded(px(ROUNDING_SMALL))
                    .bg(if ap.in_use {
                        rgba((colours.primary_container << 8) | 0xFF)
                    } else {
                        rgba((colours.surface_container_high << 8) | 0xFF)
                    })
                    .cursor(CursorStyle::PointingHand)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(if ap.in_use {
                                        rgb(colours.primary)
                                    } else {
                                        rgb(colours.on_surface_variant)
                                    })
                                    .child(wifi_icon),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(if ap.in_use {
                                                rgb(colours.on_primary_container)
                                            } else {
                                                rgb(colours.on_surface)
                                            })
                                            .child(ap.ssid.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgb(colours.on_surface_variant))
                                            .child(format!(
                                                "{}% signal • {}",
                                                ap.signal,
                                                if ap.security.is_empty() {
                                                    "Open".to_string()
                                                } else {
                                                    ap.security.clone()
                                                }
                                            )),
                                    ),
                            ),
                    )
                    .child(
                        if ap.in_use {
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(colours.primary))
                                .child("Connected")
                        } else if is_connecting {
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(colours.secondary))
                                .child("Connecting...")
                        } else {
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(colours.on_surface_variant))
                                .child(if ap.security.is_empty() { "" } else { "lock" })
                        },
                    );
                root = root.child(row);
            }
        }

        root
    }
}

/// Ethernet Interface Details Section
#[derive(Clone, Debug, Default)]
pub struct EthernetSection {
    pub interface_name: String,
    pub carrier_up: bool,
    pub ip_address: String,
    pub speed_mbps: u32,
    pub duplex: String,
}

impl EthernetSection {
    pub fn new(interface: &str, carrier: bool, ip: &str, speed: u32, duplex: &str) -> Self {
        Self {
            interface_name: interface.to_string(),
            carrier_up: carrier,
            ip_address: ip.to_string(),
            speed_mbps: speed,
            duplex: duplex.to_string(),
        }
    }

    pub fn render(&self, colours: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .bg(rgba((colours.surface_container << 8) | 0xFF))
            .rounded(px(ROUNDING_MEDIUM))
            .gap(px(SPACING_SMALL))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .text_color(if self.carrier_up {
                                        rgb(colours.primary)
                                    } else {
                                        rgb(colours.on_surface_variant)
                                    })
                                    .child("settings_ethernet"),
                            )
                            .child(
                                div()
                                    .text_size(px(FONT_SIZE_TITLE_SMALL))
                                    .text_color(rgb(colours.on_surface))
                                    .child(format!("Ethernet ({})", self.interface_name)),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(if self.carrier_up {
                                rgb(colours.primary)
                            } else {
                                rgb(colours.error)
                            })
                            .child(if self.carrier_up { "Cable Connected" } else { "Disconnected" }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p(px(8.0))
                    .bg(rgba((colours.surface_container_high << 8) | 0xFF))
                    .rounded(px(ROUNDING_SMALL))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(colours.on_surface_variant))
                            .child("IP Address"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(colours.on_surface))
                            .child(if self.ip_address.is_empty() {
                                "None".to_string()
                            } else {
                                self.ip_address.clone()
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p(px(8.0))
                    .bg(rgba((colours.surface_container_high << 8) | 0xFF))
                    .rounded(px(ROUNDING_SMALL))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(colours.on_surface_variant))
                            .child("Link Speed"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(colours.on_surface))
                            .child(format!("{} Mbps ({})", self.speed_mbps, self.duplex)),
                    ),
            )
    }
}

/// IPv4 Configuration Section (DHCP vs Static)
#[derive(Clone, Debug)]
pub struct Ipv4ConfigSection {
    pub is_dhcp: bool,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub dns: Vec<String>,
}

impl Default for Ipv4ConfigSection {
    fn default() -> Self {
        Self {
            is_dhcp: true,
            ip: "192.168.1.100".to_string(),
            netmask: "255.255.255.0".to_string(),
            gateway: "192.168.1.1".to_string(),
            dns: vec!["1.1.1.1".to_string(), "8.8.8.8".to_string()],
        }
    }
}

impl Ipv4ConfigSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle_mode(&mut self) {
        self.is_dhcp = !self.is_dhcp;
    }

    pub fn render(&self, colours: &CaelestiaColours) -> Div {
        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .bg(rgba((colours.surface_container << 8) | 0xFF))
            .rounded(px(ROUNDING_MEDIUM))
            .gap(px(SPACING_SMALL));

        // Header with DHCP toggle
        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .child(
                div()
                    .text_size(px(FONT_SIZE_TITLE_SMALL))
                    .text_color(rgb(colours.on_surface))
                    .child("IPv4 Protocol Configuration"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(colours.on_surface_variant))
                            .child(if self.is_dhcp { "Automatic (DHCP)" } else { "Manual (Static)" }),
                    )
                    .child(
                        div()
                            .cursor(CursorStyle::PointingHand)
                            .p(px(4.0))
                            .rounded(px(ROUNDING_EXTRA_SMALL))
                            .bg(rgba((colours.primary << 8) | 0x33))
                            .text_size(px(11.0))
                            .text_color(rgb(colours.primary))
                            .child("Toggle"),
                    ),
            );

        root = root.child(header);

        if !self.is_dhcp {
            let static_fields = div()
                .flex()
                .flex_col()
                .w_full()
                .gap(px(6.0))
                .child(self.render_field("IP Address", &self.ip, colours))
                .child(self.render_field("Netmask", &self.netmask, colours))
                .child(self.render_field("Gateway", &self.gateway, colours))
                .child(self.render_field("DNS Servers", &self.dns.join(", "), colours));
            root = root.child(static_fields);
        }

        root
    }

    fn render_field(&self, label: &str, val: &str, colours: &CaelestiaColours) -> Div {
        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .p(px(8.0))
            .bg(rgba((colours.surface_container_high << 8) | 0xFF))
            .rounded(px(ROUNDING_SMALL))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .child(label.to_string()),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_surface))
                    .child(val.to_string()),
            )
    }
}
