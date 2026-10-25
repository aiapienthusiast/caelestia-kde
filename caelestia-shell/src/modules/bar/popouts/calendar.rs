//! Calendar Popout
//! Parity with `shell/modules/bar/popouts/CalendarPopout.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::list_row::ListRow;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, Styled};

pub struct CalendarPopout;

impl CalendarPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        let weekdays = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
        let mut grid = div().flex().flex_col().gap(px(4.0));

        // Weekday header
        let mut header_row = div().flex().flex_row().justify_between();
        for d in weekdays {
            header_row = header_row.child(
                div()
                    .w(px(36.0))
                    .text_align(gpui::TextAlign::Center)
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(c.on_surface_variant))
                    .child(d),
            );
        }
        grid = grid.child(header_row);

        // Days row sample
        let mut days_row = div().flex().flex_row().justify_between();
        for day in 21..=27 {
            let is_today = day == 25;
            let mut cell = div()
                .w(px(36.0))
                .h(px(36.0))
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(13.0))
                .cursor(CursorStyle::PointingHand);

            if is_today {
                cell = cell
                    .bg(rgba(c.primary))
                    .text_color(rgba(c.on_primary))
                    .font_weight(FontWeight::BOLD);
            } else {
                cell = cell.text_color(rgba(c.on_surface));
            }

            days_row = days_row.child(cell.child(day.to_string()));
        }
        grid = grid.child(days_row);

        let content = div()
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(grid)
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
