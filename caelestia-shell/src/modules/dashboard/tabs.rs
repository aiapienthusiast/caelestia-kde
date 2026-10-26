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
    pub const DEFAULT_TABS: [DashboardTab; 5] = [
        DashboardTab::Dashboard,
        DashboardTab::Media,
        DashboardTab::Performance,
        DashboardTab::Weather,
        DashboardTab::Notes,
    ];

    pub fn render<V: 'static>(
        current_tab: DashboardTab,
        colours: &CaelestiaColours,
        cx: &mut gpui::Context<V>,
        on_select: impl Fn(&mut V, DashboardTab, &mut gpui::Context<V>) + 'static + Copy,
    ) -> Div {
        div()
            .flex()
            .flex_col()
            .w_full()
            .border_b_1()
            .border_color(rgba(colours.outline_variant))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_around()
                    .w_full()
                    .pt(px(8.0))
                    .children(Self::DEFAULT_TABS.iter().map(|&tab| {
                        let is_selected = tab == current_tab;
                        let text_color = if is_selected {
                            colours.primary
                        } else {
                            colours.on_surface_variant
                        };

                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(4.0))
                            .px(px(20.0))
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
                                    .text_size(px(20.0)),
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
                            )
                            .child(
                                div()
                                    .h(px(3.0))
                                    .w(px(56.0))
                                    .rounded_full()
                                    .bg(if is_selected {
                                        rgba(colours.primary)
                                    } else {
                                        rgba(0x00000000)
                                    }),
                            )
                    })),
            )
    }
}
