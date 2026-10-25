//! Weather Popout
//! Parity with `shell/modules/bar/popouts/Weather.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::components::widgets::weather_cards::{HourlyForecastEntry, HourStripView};
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct WeatherPopout;

impl WeatherPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let forecast_entries = vec![
            HourlyForecastEntry::new("12:00", 12, 22.0, 0),
            HourlyForecastEntry::new("13:00", 13, 23.5, 1),
            HourlyForecastEntry::new("14:00", 14, 24.0, 1),
            HourlyForecastEntry::new("15:00", 15, 23.0, 2),
            HourlyForecastEntry::new("16:00", 16, 21.5, 3),
            HourlyForecastEntry::new("17:00", 17, 19.0, 51),
        ];

        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(ListRow::render(
                "22°C • Clear Sky",
                Some("Humidity: 45% • Wind: 12 km/h"),
                Some("wb_sunny"),
                None,
                false,
                false,
                false,
                c,
            ))
            .child(HourStripView::new(forecast_entries))
            .child(ListRow::render(
                "Tomorrow",
                Some("24°C • Mostly Sunny"),
                Some("partly_cloudy_day"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Weather Forecast", "wb_sunny", center_x, content, c)
    }
}
