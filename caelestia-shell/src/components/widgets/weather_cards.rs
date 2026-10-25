//! Weather visualization cards and hourly forecast strip
//! Parity with `shell/components/widgets/WeatherCards.qml`.

use gpui::*;

/// Model for an hourly forecast data point
#[derive(Clone, Debug, PartialEq)]
pub struct HourlyForecastEntry {
    pub timestamp: String,
    pub hour: u32,
    pub temp_c: f32,
    pub weather_code: u32,
    pub icon_name: String,
}

impl HourlyForecastEntry {
    pub fn new(timestamp: impl Into<String>, hour: u32, temp_c: f32, code: u32) -> Self {
        let icon_name = match code {
            0 => "weather_sunny",
            1..=3 => "weather_partly_cloudy",
            45 | 48 => "weather_fog",
            51..=67 => "weather_rainy",
            71..=77 => "weather_snowy",
            80..=82 => "weather_pouring",
            95..=99 => "weather_lightning",
            _ => "weather_cloudy",
        }
        .to_string();

        Self {
            timestamp: timestamp.into(),
            hour,
            temp_c,
            weather_code: code,
            icon_name,
        }
    }

    pub fn format_hour(&self, twelve_hour: bool) -> String {
        if twelve_hour {
            let suffix = if self.hour >= 12 { "PM" } else { "AM" };
            let hr = if self.hour % 12 == 0 { 12 } else { self.hour % 12 };
            format!("{}{}", hr, suffix)
        } else {
            format!("{:02}:00", self.hour)
        }
    }
}

/// Strip view rendering hourly forecast temperatures and icons
#[derive(IntoElement)]
pub struct HourStripView {
    entries: Vec<HourlyForecastEntry>,
    twelve_hour: bool,
}

impl HourStripView {
    pub fn new(entries: Vec<HourlyForecastEntry>) -> Self {
        Self {
            entries,
            twelve_hour: true,
        }
    }

    pub fn with_twelve_hour(mut self, twelve_hour: bool) -> Self {
        self.twelve_hour = twelve_hour;
        self
    }
}

impl RenderOnce for HourStripView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut row = div().flex().flex_row().gap_3().p_2().overflow_hidden();

        for entry in self.entries {
            let label = entry.format_hour(self.twelve_hour);
            let temp_str = format!("{:.0}°", entry.temp_c);

            row = row.child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .p_2()
                    .rounded_md()
                    .bg(rgb(0x2a2830))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xa09cb0))
                            .child(label),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .child(temp_str),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xd0bcff))
                            .child(entry.icon_name),
                    ),
            );
        }

        row
    }
}
