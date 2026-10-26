//! Weather Popout
//! Parity with `shell/modules/bar/popouts/Weather.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::components::widgets::weather_cards::{HourlyForecastEntry, HourStripView};
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug)]
pub struct WeatherPopoutState {
    pub location: String,
    pub current_temp: String,
    pub condition_text: String,
    pub condition_icon: String,
    pub high_temp: String,
    pub low_temp: String,
    pub humidity: String,
    pub feels_like: String,
    pub wind_speed: String,
    pub hourly_forecast: Vec<HourlyForecastEntry>,
}

impl Default for WeatherPopoutState {
    fn default() -> Self {
        Self {
            location: "Tokyo".into(),
            current_temp: "22°C".into(),
            condition_text: "Clear Sky".into(),
            condition_icon: "wb_sunny".into(),
            high_temp: "24°".into(),
            low_temp: "16°".into(),
            humidity: "48%".into(),
            feels_like: "23°C".into(),
            wind_speed: "12 km/h".into(),
            hourly_forecast: vec![
                HourlyForecastEntry::new("12:00", 12, 22.0, 0),
                HourlyForecastEntry::new("13:00", 13, 23.5, 1),
                HourlyForecastEntry::new("14:00", 14, 24.0, 1),
                HourlyForecastEntry::new("15:00", 15, 23.0, 2),
                HourlyForecastEntry::new("16:00", 16, 21.5, 3),
                HourlyForecastEntry::new("17:00", 17, 19.0, 51),
            ],
        }
    }
}

pub struct WeatherPopout;

impl WeatherPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &WeatherPopoutState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &WeatherPopoutState,
        c: &CaelestiaColours,
    ) -> Div {
        let content = div()
            .flex()
            .flex_col()
            .w(px(380.0))
            .gap(px(SPACING_SMALL))
            // Hero Weather Summary Card
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p(px(PADDING_MEDIUM))
                    .rounded(px(ROUNDING_LARGE))
                    .bg(rgba(c.surface_container))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                StyledText::new(state.location.clone())
                                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgba(c.on_surface_variant)),
                            )
                            .child(
                                StyledText::new(state.current_temp.clone())
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgba(c.on_surface)),
                            )
                            .child(
                                StyledText::new(format!(
                                    "{} • H:{} L:{}",
                                    state.condition_text, state.high_temp, state.low_temp
                                ))
                                .text_size(px(FONT_SIZE_BODY_SMALL))
                                .text_color(rgba(c.outline)),
                            ),
                    )
                    .child(
                        MaterialIcon::new(state.condition_icon.clone())
                            .text_size(px(48.0))
                            .text_color(rgba(c.primary)),
                    ),
            )
            // 3 Detail Cards Row
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(SPACING_SMALL))
                    // Humidity
                    .child(Self::render_detail_card(
                        "water_drop",
                        "Humidity",
                        &state.humidity,
                        c.secondary,
                        c,
                    ))
                    // Feels Like
                    .child(Self::render_detail_card(
                        "thermostat",
                        "Feels like",
                        &state.feels_like,
                        c.primary,
                        c,
                    ))
                    // Wind
                    .child(Self::render_detail_card(
                        "air",
                        "Wind",
                        &state.wind_speed,
                        c.secondary,
                        c,
                    )),
            )
            // Hourly Forecast Strip Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(SPACING_EXTRA_SMALL))
                    .child(
                        StyledText::new("Hourly forecast")
                            .text_size(px(FONT_SIZE_BODY_MEDIUM))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(c.on_surface)),
                    ),
            )
            // Hourly Forecast Strip Card
            .child(
                div()
                    .p(px(PADDING_SMALL))
                    .rounded(px(ROUNDING_LARGE))
                    .bg(rgba(c.surface_container))
                    .child(HourStripView::new(state.hourly_forecast.clone())),
            );

        PopoutWrapper::render("Weather", "wb_sunny", center_x, content, c)
    }

    fn render_detail_card(
        icon: &'static str,
        label: &'static str,
        value: &str,
        icon_colour_u32: u32,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .p(px(PADDING_SMALL))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .gap(px(2.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(SPACING_EXTRA_SMALL))
                    .child(
                        MaterialIcon::new(icon)
                            .text_size(px(ICON_SIZE_SMALL))
                            .text_color(rgba(icon_colour_u32)),
                    )
                    .child(
                        StyledText::new(label)
                            .text_size(px(FONT_SIZE_LABEL_SMALL))
                            .text_color(rgba(c.on_surface_variant)),
                    ),
            )
            .child(
                StyledText::new(value.to_string())
                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(c.on_surface)),
            )
    }
}
