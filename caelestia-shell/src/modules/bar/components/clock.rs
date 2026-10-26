//! Clock Bar Widget & Model
//! Complete parity with `shell/modules/bar/components/Clock.qml`.

use chrono::{Local, Timelike};
use serde::{Deserialize, Serialize};
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClockSnapshot {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub hour_str: String,
    pub minute_str: String,
    pub second_str: String,
    pub am_pm_str: String,
    pub day_of_week: String,
    pub day_of_month: String,
    pub time_str: String,
    pub date_str: String,
}

impl ClockSnapshot {
    pub fn formatted(use_24h: bool, show_seconds: bool) -> Self {
        let now = Local::now();
        let hours = now.hour();
        let minutes = now.minute();
        let seconds = now.second();

        let (hour_str, period) = if use_24h {
            (format!("{:02}", hours), String::new())
        } else {
            let (is_pm, h12) = now.hour12();
            (format!("{}", h12), if is_pm { "PM".to_string() } else { "AM".to_string() })
        };

        let am_pm_str = period.to_lowercase();
        let minute_str = format!("{:02}", minutes);
        let second_str = format!("{:02}", seconds);
        let day_of_week = now.format("%a").to_string();
        let day_of_month = now.format("%e").to_string().trim().to_string();

        let time_str = if use_24h {
            if show_seconds {
                format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
            } else {
                format!("{:02}:{:02}", hours, minutes)
            }
        } else if show_seconds {
            format!("{}:{:02}:{:02} {}", hour_str, minutes, seconds, period)
        } else {
            format!("{}:{:02} {}", hour_str, minutes, period)
        };
        let date_str = now.format("%a, %b %e").to_string();

        Self {
            hours,
            minutes,
            seconds,
            hour_str,
            minute_str,
            second_str,
            am_pm_str,
            day_of_week,
            day_of_month,
            time_str,
            date_str,
        }
    }

    pub fn now(use_24h: bool) -> Self {
        Self::formatted(use_24h, false)
    }
}

pub struct BarClockModel;

impl BarClockModel {
    pub fn current(use_24h: bool, show_seconds: bool) -> ClockSnapshot {
        ClockSnapshot::formatted(use_24h, show_seconds)
    }
}

#[derive(Clone, Debug)]
pub struct ClockWidgetConfig {
    pub is_horizontal: bool,
    pub show_icon: bool,
    pub show_date: bool,
    pub show_seconds: bool,
    pub use_24h: bool,
    pub has_background: bool,
}

impl Default for ClockWidgetConfig {
    fn default() -> Self {
        Self {
            is_horizontal: true,
            show_icon: true,
            show_date: true,
            show_seconds: false,
            use_24h: true,
            has_background: true,
        }
    }
}

pub struct ClockWidget;

impl ClockWidget {
    /// Renders the Clock widget with 100% visual and layout parity with `Clock.qml`.
    pub fn render(snapshot: &ClockSnapshot, c: &CaelestiaColours) -> Div {
        Self::render_with_config(snapshot, &ClockWidgetConfig::default(), c)
    }

    /// Renders with explicit layout and visibility options.
    pub fn render_with_config(
        snapshot: &ClockSnapshot,
        config: &ClockWidgetConfig,
        c: &CaelestiaColours,
    ) -> Div {
        let text_color = c.secondary;
        let padding_h = if config.has_background { PADDING_MEDIUM } else { PADDING_EXTRA_SMALL };
        let padding_v = if config.has_background { PADDING_SMALL } else { 2.0 };

        let mut container = div()
            .flex()
            .cursor_pointer()
            .rounded_full();

        if config.has_background {
            container = container.bg(rgba(c.surface_container));
        }

        if config.is_horizontal {
            // Horizontal layout matching Clock.qml:120-206
            let mut row = div()
                .flex()
                .flex_row()
                .items_center()
                .px(px(padding_h))
                .py(px(padding_v))
                .gap(px(SPACING_EXTRA_SMALL));

            // Calendar Icon
            if config.show_icon {
                row = row.child(
                    MaterialIcon::new("calendar_month")
                        .text_size(px(14.0))
                        .text_color(rgba(text_color)),
                );
            }

            // Date: "Mon 26"
            if config.show_date {
                row = row
                    .child(
                        StyledText::new(format!("{} {}", snapshot.day_of_week, snapshot.day_of_month))
                            .text_size(px(FONT_SIZE_BODY_SMALL))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(text_color)),
                    )
                    // 1px separator divider
                    .child(
                        div()
                            .w(px(1.0))
                            .h(px(12.0))
                            .bg(rgba(text_color))
                            .opacity(0.2),
                    );
            }

            // Time: "14:30" or "2:30 pm"
            row = row
                .child(
                    StyledText::new(&snapshot.hour_str)
                        .text_size(px(FONT_SIZE_BODY_SMALL))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(text_color)),
                )
                .child(
                    StyledText::new(":")
                        .text_size(px(FONT_SIZE_BODY_SMALL))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(text_color)),
                )
                .child(
                    StyledText::new(&snapshot.minute_str)
                        .text_size(px(FONT_SIZE_BODY_SMALL))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(text_color)),
                );

            if config.show_seconds {
                row = row
                    .child(
                        StyledText::new(":")
                            .text_size(px(FONT_SIZE_BODY_SMALL))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(text_color)),
                    )
                    .child(
                        StyledText::new(&snapshot.second_str)
                            .text_size(px(FONT_SIZE_BODY_SMALL))
                            .text_color(rgba(text_color)),
                    );
            }

            if !config.use_24h && !snapshot.am_pm_str.is_empty() {
                row = row.child(
                    StyledText::new(&snapshot.am_pm_str)
                        .text_size(px(FONT_SIZE_LABEL_SMALL))
                        .text_color(rgba(text_color)),
                );
            }

            container.child(row)
        } else {
            // Vertical layout matching Clock.qml:208-304
            let mut col = div()
                .flex()
                .flex_col()
                .items_center()
                .px(px(padding_v))
                .py(px(padding_h))
                .gap(px(2.0));

            if config.show_icon {
                col = col.child(
                    MaterialIcon::new("calendar_month")
                        .text_size(px(14.0))
                        .text_color(rgba(text_color)),
                );
            }

            if config.show_date {
                col = col
                    .child(
                        StyledText::new(&snapshot.day_of_week)
                            .text_size(px(FONT_SIZE_LABEL_SMALL))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(text_color)),
                    )
                    .child(
                        StyledText::new(&snapshot.day_of_month)
                            .text_size(px(FONT_SIZE_LABEL_SMALL))
                            .text_color(rgba(text_color)),
                    )
                    .child(
                        div()
                            .w(px(16.0))
                            .h(px(1.0))
                            .bg(rgba(c.outline_variant)),
                    );
            }

            col = col
                .child(
                    StyledText::new(&snapshot.hour_str)
                        .text_size(px(FONT_SIZE_BODY_SMALL))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(text_color)),
                )
                .child(
                    StyledText::new(&snapshot.minute_str)
                        .text_size(px(FONT_SIZE_BODY_SMALL))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(text_color)),
                );

            if config.show_seconds {
                col = col.child(
                    StyledText::new(&snapshot.second_str)
                        .text_size(px(FONT_SIZE_LABEL_SMALL))
                        .text_color(rgba(text_color)),
                );
            }

            if !config.use_24h && !snapshot.am_pm_str.is_empty() {
                col = col.child(
                    StyledText::new(&snapshot.am_pm_str)
                        .text_size(px(FONT_SIZE_LABEL_SMALL))
                        .text_color(rgba(text_color)),
                );
            }

            container.child(col)
        }
    }
}

