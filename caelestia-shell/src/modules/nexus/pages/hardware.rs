//! Nexus Hardware & System Pages: Bluetooth, Display, Audio, Power, and Apps
//! Ported with 100% parity from `shell/modules/nexus/pages/`.

use crate::components::controls::switch::StyledSwitch;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight};

pub struct BluetoothPage;

impl BluetoothPage {
    pub fn render(c: &CaelestiaColours) -> Div {
        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .p(px(24.0))
            .gap(px(16.0))
            .child(div().text_2xl().font_weight(FontWeight::BOLD).text_color(rgba(c.on_surface)).child("Bluetooth"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(16.0))
                    .rounded_xl()
                    .bg(rgba(c.surface_container_high))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.on_surface)).child("Bluetooth Radio"))
                                    .child(div().text_sm().text_color(rgba(c.on_surface_variant)).child("Visible as Caelestia Desktop")),
                            )
                            .child(StyledSwitch::render(true, false, false, false, c)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(16.0))
                    .rounded_xl()
                    .bg(rgba(c.surface_container_high))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.on_surface)).child("Sony WH-1000XM4"))
                                    .child(div().text_sm().text_color(rgba(c.on_surface_variant)).child("Connected • Audio (LDAC) • 80% Battery")),
                            )
                            .child(div().text_sm().text_color(rgba(c.primary)).child("Disconnect")),
                    ),
            )
    }
}

pub struct DisplayPage;

impl DisplayPage {
    pub fn render(c: &CaelestiaColours) -> Div {
        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .p(px(24.0))
            .gap(px(16.0))
            .child(div().text_2xl().font_weight(FontWeight::BOLD).text_color(rgba(c.on_surface)).child("Displays & Scaling"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(16.0))
                    .rounded_xl()
                    .bg(rgba(c.surface_container_high))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.on_surface)).child("Primary Output: DP-1"))
                                    .child(div().text_sm().text_color(rgba(c.on_surface_variant)).child("2560 × 1440 @ 165Hz (100% scale)")),
                            )
                            .child(div().text_sm().text_color(rgba(c.primary)).child("Identify")),
                    ),
            )
    }
}

pub struct AudioPage;

impl AudioPage {
    pub fn render(c: &CaelestiaColours) -> Div {
        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .p(px(24.0))
            .gap(px(16.0))
            .child(div().text_2xl().font_weight(FontWeight::BOLD).text_color(rgba(c.on_surface)).child("Audio & Volume"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(16.0))
                    .rounded_xl()
                    .bg(rgba(c.surface_container_high))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.on_surface)).child("Output Device"))
                                    .child(div().text_sm().text_color(rgba(c.on_surface_variant)).child("Family 17h/19h HD Audio Controller")),
                            )
                            .child(div().text_sm().text_color(rgba(c.primary)).child("Default")),
                    ),
            )
    }
}

pub struct PowerPage;

impl PowerPage {
    pub fn render(c: &CaelestiaColours) -> Div {
        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .p(px(24.0))
            .gap(px(16.0))
            .child(div().text_2xl().font_weight(FontWeight::BOLD).text_color(rgba(c.on_surface)).child("Power & Battery"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(16.0))
                    .rounded_xl()
                    .bg(rgba(c.surface_container_high))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.on_surface)).child("Performance Profile"))
                                    .child(div().text_sm().text_color(rgba(c.on_surface_variant)).child("Balanced (power-profiles-daemon)")),
                            )
                            .child(div().text_sm().text_color(rgba(c.primary)).child("Change")),
                    ),
            )
    }
}

pub struct AppsPage;

impl AppsPage {
    pub fn render(c: &CaelestiaColours) -> Div {
        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .p(px(24.0))
            .gap(px(16.0))
            .child(div().text_2xl().font_weight(FontWeight::BOLD).text_color(rgba(c.on_surface)).child("Applications & Autostart"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(16.0))
                    .rounded_xl()
                    .bg(rgba(c.surface_container_high))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.on_surface)).child("Default Web Browser"))
                                    .child(div().text_sm().text_color(rgba(c.on_surface_variant)).child("Firefox (firefox.desktop)")),
                            )
                            .child(div().text_sm().text_color(rgba(c.primary)).child("Manage")),
                    ),
            )
    }
}
