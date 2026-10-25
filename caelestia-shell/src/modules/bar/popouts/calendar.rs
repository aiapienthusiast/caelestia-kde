//! Calendar Popout
//! Parity with `shell/modules/bar/popouts/CalendarPopout.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::calendar::{CalendarMonth, CalendarMonthState};
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, Styled};

pub struct CalendarPopout;

impl CalendarPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let calendar = CalendarMonth::new(CalendarMonthState::default());

        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(calendar)
            .child(ListRow::render(
                "No upcoming events today",
                Some("Calendar synced"),
                Some("event_available"),
                None,
                false,
                false,
                false,
                c,
            ));

        PopoutWrapper::render("Calendar", "calendar_month", center_x, content, c)
    }
}
