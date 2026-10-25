//! Weather Popout
//! Parity with `shell/modules/bar/popouts/Weather.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct WeatherPopout;

impl WeatherPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
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
