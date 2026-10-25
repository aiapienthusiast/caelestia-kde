//! Weather Tab
//! Parity with `shell/modules/dashboard/WeatherTab.qml`.
//! Displays detailed weather observations, wind, humidity, and forecasts.

use crate::components::icon::MaterialIcon;
use crate::components::styled_text::{StyledText, StyledTextExt};
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::weather::WeatherData;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct WeatherTabView;

impl WeatherTabView {
    pub fn render(weather: &WeatherData, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .w_full()
            .h_full()
            .child(
                // Hero Weather Observation Card
                div()
                    .p(px(24.0))
                    .rounded(px(24.0))
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
                            .gap(px(24.0))
                            .child(
                                div()
                                    .w(px(80.0))
                                    .h(px(80.0))
                                    .rounded_full()
                                    .bg(rgba(c.secondary_container))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        MaterialIcon::new(&weather.icon)
                                            .color(c.on_secondary_container)
                                            .icon_size(48.0),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        StyledText::new(&weather.temp)
                                            .color(c.on_surface)
                                            .font_size(36.0)
                                            .font_weight(gpui::FontWeight::BOLD),
                                    )
                                    .child(
                                        StyledText::new(&weather.condition)
                                            .color(c.on_surface_variant)
                                            .font_size(16.0),
                                    )
                                    .child(
                                        StyledText::new(&weather.location)
                                            .color(c.primary)
                                            .font_size(13.0)
                                            .font_weight(gpui::FontWeight::MEDIUM),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(24.0))
                            .child(Self::render_stat_box("water_drop", "Humidity", &format!("{}%", weather.humidity), c))
                            .child(Self::render_stat_box("air", "Wind", &format!("{:.0} km/h", weather.wind_speed_kmh), c))
                            .child(Self::render_stat_box("thermostat", "Range", &format!("{} / {}", weather.high, weather.low), c)),
                    ),
            )
            .child(
                // 5-Day Forecast Row
                div()
                    .flex()
                    .gap(px(12.0))
                    .w_full()
                    .child(Self::forecast_day("Mon", "sunny", "25°", "17°", c))
                    .child(Self::forecast_day("Tue", "partly_cloudy_day", "24°", "16°", c))
                    .child(Self::forecast_day("Wed", "cloud", "22°", "15°", c))
                    .child(Self::forecast_day("Thu", "rainy", "20°", "14°", c))
                    .child(Self::forecast_day("Fri", "sunny", "26°", "18°", c)),
            )
    }

    fn render_stat_box(icon: &'static str, label: &'static str, value: &str, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(4.0))
            .p(px(12.0))
            .rounded(px(16.0))
            .bg(rgba(c.surface_container_high))
            .w(px(90.0))
            .child(MaterialIcon::new(icon).color(c.primary).icon_size(22.0))
            .child(
                StyledText::new(value)
                    .color(c.on_surface)
                    .font_size(13.0)
                    .font_weight(gpui::FontWeight::BOLD),
            )
            .child(
                StyledText::new(label)
                    .color(c.on_surface_variant)
                    .font_size(11.0),
            )
    }

    fn forecast_day(day: &'static str, icon: &'static str, high: &'static str, low: &'static str, c: &CaelestiaColours) -> Div {
        div()
            .flex_1()
            .p(px(16.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .flex()
            .flex_col()
            .items_center()
            .gap(px(8.0))
            .child(
                StyledText::new(day)
                    .color(c.on_surface_variant)
                    .font_size(13.0)
                    .font_weight(gpui::FontWeight::BOLD),
            )
            .child(MaterialIcon::new(icon).color(c.primary).icon_size(28.0))
            .child(
                StyledText::new(high)
                    .color(c.on_surface)
                    .font_size(14.0)
                    .font_weight(gpui::FontWeight::BOLD),
            )
            .child(
                StyledText::new(low)
                    .color(c.on_surface_variant)
                    .font_size(12.0),
            )
    }
}
