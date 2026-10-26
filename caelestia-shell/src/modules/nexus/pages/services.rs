//! Nexus Background Services Configuration Page
//! Ported with 100% parity from `shell/modules/nexus/pages/services/`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesSettingsPageModel {
    pub gamemode_enabled: bool,
    pub gamemode_registered_apps: Vec<String>,
    pub quickshare_enabled: bool,
    pub quickshare_device_name: String,
    pub quickshare_visible_to_everyone: bool,
    pub focus_mode_active: bool,
    pub focus_duration_mins: u32,
    pub dnd_enabled: bool,
    pub toast_sound_effects: bool,
    pub idle_inhibitor_active: bool,
}

impl Default for ServicesSettingsPageModel {
    fn default() -> Self {
        Self {
            gamemode_enabled: true,
            gamemode_registered_apps: vec!["steam".to_string(), "heroic".to_string()],
            quickshare_enabled: true,
            quickshare_device_name: "Caelestia Desktop".to_string(),
            quickshare_visible_to_everyone: false,
            focus_mode_active: false,
            focus_duration_mins: 25,
            dnd_enabled: false,
            toast_sound_effects: true,
            idle_inhibitor_active: false,
        }
    }
}

impl ServicesSettingsPageModel {
    pub fn toggle_gamemode(&mut self) {
        self.gamemode_enabled = !self.gamemode_enabled;
    }

    pub fn toggle_quickshare(&mut self) {
        self.quickshare_enabled = !self.quickshare_enabled;
    }

    pub fn toggle_focus_mode(&mut self) {
        self.focus_mode_active = !self.focus_mode_active;
        if self.focus_mode_active {
            self.dnd_enabled = true;
        }
    }

    pub fn toggle_idle_inhibitor(&mut self) {
        self.idle_inhibitor_active = !self.idle_inhibitor_active;
    }
}

pub struct ServicesPage;

impl ServicesPage {
    pub fn render(c: &crate::infrastructure::theme::CaelestiaColours) -> gpui::Div {
        use gpui::{div, prelude::*, px, rgba, FontWeight};

        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .p(px(24.0))
            .gap(px(16.0))
            .overflow_hidden()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(c.on_surface))
                    .child("Services & Background Daemon"),
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
                                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.on_surface)).child("GameMode Daemon"))
                                    .child(div().text_sm().text_color(rgba(c.on_surface_variant)).child("Optimize CPU frequency and GPU clocks for games")),
                            )
                            .child(crate::components::controls::switch::StyledSwitch::render(true, false, false, false, c)),
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
                                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.on_surface)).child("QuickShare Nearby Service"))
                                    .child(div().text_sm().text_color(rgba(c.on_surface_variant)).child("Receive files wirelessly from Android and Linux devices")),
                            )
                            .child(crate::components::controls::switch::StyledSwitch::render(true, false, false, false, c)),
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
                                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgba(c.on_surface)).child("Focus Session / Do Not Disturb"))
                                    .child(div().text_sm().text_color(rgba(c.on_surface_variant)).child("Mute popups and notifications")),
                            )
                            .child(crate::components::controls::switch::StyledSwitch::render(false, false, false, false, c)),
                    ),
            )
    }
}

