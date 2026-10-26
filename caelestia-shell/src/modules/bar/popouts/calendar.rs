//! Calendar Popout
//! Parity with `shell/modules/bar/popouts/CalendarPopout.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::calendar::{CalendarMonth, CalendarMonthState};
use crate::components::controls::icon_button::IconButton;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug)]
pub struct CalendarAgendaEvent {
    pub id: String,
    pub title: String,
    pub time_str: String,
    pub color_u32: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct CalendarPopoutState {
    pub selected_label: String,
    pub events: Vec<CalendarAgendaEvent>,
    pub month_state: CalendarMonthState,
}

impl Default for CalendarPopoutState {
    fn default() -> Self {
        Self {
            selected_label: "Today".into(),
            events: vec![
                CalendarAgendaEvent {
                    id: "1".into(),
                    title: "KDE Plasma 6 Sync Meeting".into(),
                    time_str: "10:00 - 11:00".into(),
                    color_u32: None,
                },
                CalendarAgendaEvent {
                    id: "2".into(),
                    title: "GPUI Shell Performance Audit".into(),
                    time_str: "14:30 - 15:30".into(),
                    color_u32: None,
                },
            ],
            month_state: CalendarMonthState::default(),
        }
    }
}

pub struct CalendarPopout;

impl CalendarPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &CalendarPopoutState::default(), c)
    }

    pub fn render_with_state(
        center_x: f32,
        state: &CalendarPopoutState,
        c: &CaelestiaColours,
    ) -> Div {
        let calendar = CalendarMonth::new(state.month_state.clone());

        let content = div()
            .flex()
            .flex_col()
            .w(px(497.0))
            .p(px(PADDING_MEDIUM))
            .rounded(px(ROUNDING_EXTRA_LARGE))
            .bg(rgba(c.surface_container))
            .gap(px(SPACING_SMALL))
            // Calendar Month Grid
            .child(calendar)
            // Divider
            .child(
                div()
                    .w_full()
                    .h(px(1.0))
                    .bg(rgba(c.outline_variant)),
            )
            // Agenda Section Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(PADDING_SMALL))
                    .child(
                        StyledText::new(state.selected_label.clone())
                            .text_size(px(FONT_SIZE_BODY_MEDIUM))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgba(c.on_surface)),
                    )
                    .child(
                        IconButton::new("open_in_new")
                            .with_colours(c.clone())
                            .into_element(),
                    ),
            )
            // Events List or Empty State
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(SPACING_EXTRA_SMALL))
                    .children(if state.events.is_empty() {
                        vec![div()
                            .p(px(PADDING_SMALL))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(SPACING_SMALL))
                            .child(
                                MaterialIcon::new("event_available")
                                    .text_size(px(ICON_SIZE_SMALL))
                                    .text_color(rgba(c.on_surface_variant)),
                            )
                            .child(
                                StyledText::new("No events scheduled")
                                    .text_size(px(FONT_SIZE_BODY_SMALL))
                                    .text_color(rgba(c.on_surface_variant)),
                            )]
                    } else {
                        state
                            .events
                            .iter()
                            .map(|evt| {
                                let badge_col = evt.color_u32.unwrap_or(c.primary);
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .p(px(PADDING_SMALL))
                                    .rounded(px(ROUNDING_MEDIUM))
                                    .bg(rgba(c.surface_container_low))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(SPACING_SMALL))
                                            .child(
                                                div()
                                                    .w(px(4.0))
                                                    .h(px(16.0))
                                                    .rounded_full()
                                                    .bg(rgba(badge_col)),
                                            )
                                            .child(
                                                StyledText::new(evt.title.clone())
                                                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgba(c.on_surface)),
                                            ),
                                    )
                                    .child(
                                        StyledText::new(evt.time_str.clone())
                                            .text_size(px(FONT_SIZE_BODY_SMALL))
                                            .text_color(rgba(c.outline)),
                                    )
                            })
                            .collect()
                    }),
            );

        PopoutWrapper::render("Calendar", "calendar_month", center_x, content, c)
    }
}
