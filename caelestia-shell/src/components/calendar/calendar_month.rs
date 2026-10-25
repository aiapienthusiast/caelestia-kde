//! Month at a glance calendar matching `shell/components/CalendarMonth.qml`.
//! Includes month navigation, weekday headers, and 42-day grid with indicators.

use std::collections::HashSet;
use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
use gpui::prelude::*;
use gpui::*;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

/// Information for a single day tile in the 42-day calendar grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarDayCell {
    pub date: NaiveDate,
    pub day_number: u32,
    pub is_in_month: bool,
    pub is_today: bool,
    pub is_marked: bool,
    pub is_weekend: bool,
    pub has_events: bool,
}

/// Calendar month component state and logic.
#[derive(Debug, Clone, PartialEq)]
pub struct CalendarMonthState {
    pub view_date: NaiveDate,
    pub highlighted_date: Option<NaiveDate>,
    pub selectable: bool,
    pub scale_offset: f32,
    pub font_scale: f32,
    pub events: HashSet<NaiveDate>,
}

impl Default for CalendarMonthState {
    fn default() -> Self {
        let today = chrono::Local::now().date_naive();
        Self {
            view_date: today,
            highlighted_date: None,
            selectable: false,
            scale_offset: 1.0,
            font_scale: 1.0,
            events: HashSet::new(),
        }
    }
}

impl CalendarMonthState {
    pub fn new(view_date: NaiveDate) -> Self {
        Self {
            view_date,
            ..Default::default()
        }
    }

    pub fn with_highlighted_date(mut self, date: Option<NaiveDate>) -> Self {
        self.highlighted_date = date;
        self
    }

    pub fn with_selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    pub fn with_events(mut self, events: HashSet<NaiveDate>) -> Self {
        self.events = events;
        self
    }

    pub fn curr_month(&self) -> u32 {
        self.view_date.month()
    }

    pub fn curr_year(&self) -> i32 {
        self.view_date.year()
    }

    pub fn marked_date(&self) -> NaiveDate {
        self.highlighted_date.unwrap_or_else(|| chrono::Local::now().date_naive())
    }

    pub fn is_same_day(&self, a: NaiveDate, b: NaiveDate) -> bool {
        a == b
    }

    pub fn is_marked(&self, date: NaiveDate) -> bool {
        self.is_same_day(date, self.marked_date())
    }

    pub fn is_today(&self, date: NaiveDate) -> bool {
        self.is_same_day(date, chrono::Local::now().date_naive())
    }

    pub fn is_current_month(&self) -> bool {
        let now = chrono::Local::now().date_naive();
        self.view_date.year() == now.year() && self.view_date.month() == now.month()
    }

    /// Shifts the viewed month by `delta` (+1 or -1).
    pub fn shift_month(&mut self, delta: i32) {
        if delta > 0 {
            if let Some(next) = self.view_date.checked_add_months(Months::new(delta as u32)) {
                self.view_date = next;
            }
        } else if delta < 0 {
            if let Some(prev) = self.view_date.checked_sub_months(Months::new((-delta) as u32)) {
                self.view_date = prev;
            }
        }
    }

    /// Resets the viewed month to today.
    pub fn go_to_today(&mut self) {
        self.view_date = chrono::Local::now().date_naive();
    }

    /// Selects a date if selectable.
    pub fn select_day(&mut self, date: NaiveDate) {
        if self.selectable {
            self.highlighted_date = Some(date);
        }
    }

    /// Computes the 42 cells (6 rows x 7 columns) for the month view.
    pub fn generate_grid_cells(&self) -> Vec<CalendarDayCell> {
        let first_of_month = NaiveDate::from_ymd_opt(self.curr_year(), self.curr_month(), 1)
            .unwrap_or(self.view_date);

        // Days from Monday (0) to Sunday (6)
        let first_weekday_idx = first_of_month.weekday().num_days_from_monday();

        // Start from Monday on or before first_of_month
        let start_date = first_of_month
            .checked_sub_days(Days::new(first_weekday_idx as u64))
            .unwrap_or(first_of_month);

        let mut cells = Vec::with_capacity(42);
        for i in 0..42 {
            let current = start_date.checked_add_days(Days::new(i)).unwrap_or(start_date);
            let is_in_month = current.month() == self.curr_month();
            let is_weekend = current.weekday() == Weekday::Sat || current.weekday() == Weekday::Sun;

            cells.push(CalendarDayCell {
                date: current,
                day_number: current.day(),
                is_in_month,
                is_today: self.is_today(current),
                is_marked: self.is_marked(current),
                is_weekend,
                has_events: self.events.contains(&current),
            });
        }

        cells
    }

    /// Returns human-readable Month and Year title (e.g. "October 2026").
    pub fn month_year_title(&self) -> String {
        self.view_date.format("%B %Y").to_string()
    }
}

/// GPUI Element rendering the full month calendar.
#[derive(IntoElement)]
pub struct CalendarMonth {
    state: CalendarMonthState,
}

impl CalendarMonth {
    pub fn new(state: CalendarMonthState) -> Self {
        Self { state }
    }
}

impl RenderOnce for CalendarMonth {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colours = cx.try_global::<CaelestiaColours>().cloned().unwrap_or_default();
        let primary = rgba(colours.primary);
        let on_primary = rgba(colours.on_primary);
        let on_surface = rgba(colours.on_surface);
        let on_surface_variant = rgba(colours.on_surface_variant);
        let tertiary = rgba(colours.secondary);

        let scale = self.state.scale_offset;
        let cells = self.state.generate_grid_cells();

        let weekdays = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

        div()
            .flex()
            .flex_col()
            .gap(px(SPACING_EXTRA_SMALL * scale))
            .child(
                // Month Navigation Row
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(px(SPACING_EXTRA_SMALL * scale))
                    .child(
                        // Prev button
                        div()
                            .id("cal-prev-month")
                            .px(px(PADDING_SMALL * scale))
                            .py(px(PADDING_EXTRA_SMALL * scale))
                            .rounded(px(ROUNDING_FULL))
                            .cursor_pointer()
                            .child(StyledText::new("<").text_size(px(FONT_SIZE_NORMAL)).text_color(primary))
                    )
                    .child(
                        // Title / Today button
                        div()
                            .id("cal-month-title")
                            .px(px(PADDING_LARGE * scale))
                            .py(px(PADDING_EXTRA_SMALL * scale))
                            .rounded(px(ROUNDING_FULL))
                            .cursor_pointer()
                            .child(
                                StyledText::new(self.state.month_year_title())
                                    .text_size(px(FONT_SIZE_NORMAL))
                                    .text_color(primary)
                            )
                    )
                    .child(
                        // Next button
                        div()
                            .id("cal-next-month")
                            .px(px(PADDING_SMALL * scale))
                            .py(px(PADDING_EXTRA_SMALL * scale))
                            .rounded(px(ROUNDING_FULL))
                            .cursor_pointer()
                            .child(StyledText::new(">").text_size(px(FONT_SIZE_NORMAL)).text_color(primary))
                    )
            )
            .child(
                // Weekday Header Row
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .children(weekdays.iter().enumerate().map(|(idx, name)| {
                        let is_weekend = idx >= 5;
                        let color = if is_weekend {
                            tertiary
                        } else {
                            on_surface
                        };
                        div()
                            .flex_1()
                            .flex()
                            .justify_center()
                            .child(StyledText::new(*name).text_size(px(FONT_SIZE_SMALL)).text_color(color))
                    }))
            )
            .child(
                // 42-day Month Grid (6 rows of 7 days)
                div()
                    .flex()
                    .flex_col()
                    .gap(px(3.0 * scale))
                    .children((0..6).map(|row| {
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .gap(px(3.0 * scale))
                            .children((0..7).map(|col| {
                                let cell = cells[row * 7 + col];
                                let text_color = if cell.is_marked {
                                    on_primary
                                } else if cell.is_today {
                                    primary
                                } else if cell.is_weekend {
                                    tertiary
                                } else {
                                    on_surface_variant
                                };

                                let bg_color = if cell.is_marked {
                                    primary
                                } else {
                                    Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
                                };

                                let cell_opacity = if cell.is_today || cell.is_in_month {
                                    1.0
                                } else {
                                    0.4
                                };

                                div()
                                    .id(SharedString::from(format!("cal-day-{}", cell.date)))
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .py(px(PADDING_EXTRA_SMALL * scale))
                                    .rounded(px(ROUNDING_FULL))
                                    .bg(bg_color)
                                    .opacity(cell_opacity)
                                    .cursor_pointer()
                                    .child(
                                        StyledText::new(cell.day_number.to_string())
                                            .text_size(px(FONT_SIZE_SMALL))
                                            .text_color(text_color)
                                    )
                                    .when(cell.has_events && !cell.is_marked, |d| {
                                        d.child(
                                            div()
                                                .w(px(3.0 * scale))
                                                .h(px(3.0 * scale))
                                                .rounded(px(ROUNDING_FULL))
                                                .bg(primary)
                                        )
                                    })
                            }))
                    }))
            )
    }
}
