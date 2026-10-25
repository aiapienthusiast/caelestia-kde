//! Primary Dash Tab
//! Parity with `shell/modules/dashboard/Dash.qml`.
//! Contains User profile, SmallWeather, DateTime, and quick toggle cards.

use crate::components::icon::MaterialIcon;
use crate::components::styled_text::{StyledText, StyledTextExt};
use crate::infrastructure::paths::CaelestiaPaths;
use crate::infrastructure::sysinfo::SystemInfo;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::weather::WeatherData;
use chrono::Local;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct DashView;

impl DashView {
    pub fn render(
        weather: &WeatherData,
        sysinfo: &SystemInfo,
        paths: &CaelestiaPaths,
        colours: &CaelestiaColours,
    ) -> Div {
        let now = Local::now();
        let time_str = now.format("%H:%M").to_string();
        let date_str = now.format("%A, %B %d").to_string();
        let uptime_str = SystemInfo::uptime_formatted();

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .w_full()
            .h_full()
            .child(
                // Top Row: User Card + DateTime Card
                div()
                    .flex()
                    .gap(px(16.0))
                    .w_full()
                    .child(Self::render_user_card(sysinfo, &uptime_str, paths, colours))
                    .child(Self::render_datetime_card(&time_str, &date_str, colours)),
            )
            .child(
                // Bottom Row: Weather Card + Quick Actions
                div()
                    .flex()
                    .gap(px(16.0))
                    .w_full()
                    .child(Self::render_weather_card(weather, colours))
                    .child(Self::render_quick_actions(colours)),
            )
    }

    fn render_user_card(
        sysinfo: &SystemInfo,
        uptime: &str,
        _paths: &CaelestiaPaths,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .flex_1()
            .p(px(20.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .flex()
            .items_center()
            .gap(px(16.0))
            .child(
                // Avatar circle
                div()
                    .w(px(56.0))
                    .h(px(56.0))
                    .rounded_full()
                    .bg(rgba(c.primary))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        MaterialIcon::new("person")
                            .color(c.on_primary)
                            .icon_size(32.0),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        StyledText::new(&format!("Welcome back, {}", sysinfo.user))
                            .color(c.on_surface)
                            .font_size(18.0)
                            .font_weight(gpui::FontWeight::BOLD),
                    )
                    .child(
                        StyledText::new(&format!("{} • Uptime {}", sysinfo.hostname, uptime))
                            .color(c.on_surface_variant)
                            .font_size(13.0),
                    ),
            )
    }

    fn render_datetime_card(time_str: &str, date_str: &str, c: &CaelestiaColours) -> Div {
        div()
            .w(px(260.0))
            .p(px(20.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .flex()
            .flex_col()
            .justify_center()
            .child(
                StyledText::new(time_str)
                    .color(c.primary)
                    .font_size(32.0)
                    .font_weight(gpui::FontWeight::BOLD),
            )
            .child(
                StyledText::new(date_str)
                    .color(c.on_surface_variant)
                    .font_size(13.0),
            )
    }

    fn render_weather_card(weather: &WeatherData, c: &CaelestiaColours) -> Div {
        div()
            .flex_1()
            .p(px(20.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(16.0))
                    .child(
                        div()
                            .w(px(52.0))
                            .h(px(52.0))
                            .rounded_full()
                            .bg(rgba(c.secondary_container))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                MaterialIcon::new(&weather.icon)
                                    .color(c.on_secondary_container)
                                    .icon_size(30.0),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                StyledText::new(&weather.temp)
                                    .color(c.on_surface)
                                    .font_size(24.0)
                                    .font_weight(gpui::FontWeight::BOLD),
                            )
                            .child(
                                StyledText::new(&weather.condition)
                                    .color(c.on_surface_variant)
                                    .font_size(14.0),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_end()
                    .child(
                        StyledText::new(&format!("H: {}  L: {}", weather.high, weather.low))
                            .color(c.on_surface_variant)
                            .font_size(12.0),
                    )
                    .child(
                        StyledText::new(&format!("Humidity: {}%", weather.humidity))
                            .color(c.on_surface_variant)
                            .font_size(12.0),
                    ),
            )
    }

    fn render_quick_actions(c: &CaelestiaColours) -> Div {
        div()
            .w(px(260.0))
            .p(px(16.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .flex()
            .gap(px(12.0))
            .child(Self::quick_tile("wifi", "Wi-Fi", true, c))
            .child(Self::quick_tile("bluetooth", "Bluetooth", true, c))
            .child(Self::quick_tile("bedtime", "Night", false, c))
            .child(Self::quick_tile("notifications", "Notifs", true, c))
    }

    fn quick_tile(icon: &'static str, label: &'static str, active: bool, c: &CaelestiaColours) -> Div {
        let bg_color = if active {
            rgba(c.primary)
        } else {
            rgba(c.surface_container_high)
        };
        let icon_color = if active {
            c.on_primary
        } else {
            c.on_surface_variant
        };

        div()
            .flex_1()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(6.0))
            .py(px(12.0))
            .rounded(px(12.0))
            .bg(bg_color)
            .cursor(CursorStyle::PointingHand)
            .child(MaterialIcon::new(icon).color(icon_color).icon_size(20.0))
            .child(
                StyledText::new(label)
                    .color(icon_color)
                    .font_size(11.0)
                    .font_weight(gpui::FontWeight::MEDIUM),
            )
    }
}
