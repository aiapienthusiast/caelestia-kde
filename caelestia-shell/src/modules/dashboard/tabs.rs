//! Dashboard Tabs Navigation Bar
//! Parity with `shell/modules/dashboard/Tabs.qml`.

use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledText;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, MouseButton, Styled};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardTab {
    Dashboard = 0,
    Media = 1,
    Performance = 2,
    Weather = 3,
    Notes = 4,
    Terminal = 5,
    Wallhaven = 6,
}

impl DashboardTab {
    pub const ALL: [DashboardTab; 7] = [
        DashboardTab::Dashboard,
        DashboardTab::Media,
        DashboardTab::Performance,
        DashboardTab::Weather,
        DashboardTab::Notes,
        DashboardTab::Terminal,
        DashboardTab::Wallhaven,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            DashboardTab::Dashboard => "Dashboard",
            DashboardTab::Media => "Media",
            DashboardTab::Performance => "Performance",
            DashboardTab::Weather => "Weather",
            DashboardTab::Notes => "Notes",
            DashboardTab::Terminal => "Terminal",
            DashboardTab::Wallhaven => "Wallhaven",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            DashboardTab::Dashboard => "dashboard",
            DashboardTab::Media => "queue_music",
            DashboardTab::Performance => "speed",
            DashboardTab::Weather => "cloud",
            DashboardTab::Notes => "edit_note",
            DashboardTab::Terminal => "terminal",
            DashboardTab::Wallhaven => "wallpaper",
        }
    }
}

pub struct DashboardTabs;

impl DashboardTabs {
    pub fn render<V: 'static>(
        current_tab: DashboardTab,
        colours: &CaelestiaColours,
        cx: &mut gpui::Context<V>,
        on_select: impl Fn(&mut V, DashboardTab, &mut gpui::Context<V>) + 'static + Copy,
    ) -> Div {
        let mut row = div()
            .flex()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .p(px(8.0))
            .rounded_full()
            .bg(rgba(colours.surface_container));

        for tab in DashboardTab::ALL {
            let is_selected = tab == current_tab;
            let bg_color = if is_selected {
                rgba(colours.primary)
            } else {
                rgba(0x00000000)
            };
            let text_color = if is_selected {
                colours.on_primary
            } else {
                colours.on_surface_variant
            };

            row = row.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded_full()
                    .bg(bg_color)
                    .cursor(CursorStyle::PointingHand)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |view, _, _, cx| {
                            on_select(view, tab, cx);
                        }),
                    )
                    .child(
                        MaterialIcon::new(tab.icon())
                            .text_color(rgba(text_color))
                            .text_size(px(18.0)),
                    )
                    .child(
                        StyledText::new(tab.label())
                            .text_color(rgba(text_color))
                            .text_size(px(14.0))
                            .font_weight(if is_selected {
                                gpui::FontWeight::SEMIBOLD
                            } else {
                                gpui::FontWeight::NORMAL
                            }),
                    ),
            );
        }

        row
    }
}
