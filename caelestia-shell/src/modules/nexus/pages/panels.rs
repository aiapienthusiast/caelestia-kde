//! Panels settings page.
//! Parity with `shell/modules/nexus/pages/PanelsPage.qml`.

use crate::components::controls::switch::StyledSwitch;
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::config::ShellConfig;
use crate::infrastructure::theme::CaelestiaColours;
use crate::modules::nexus::action::NexusAction;
use crate::modules::nexus::window::NexusWindow;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, MouseButton,
    SharedString, Styled,
};

pub struct PanelsPage;

impl PanelsPage {
    pub fn render(
        _config: &ShellConfig,
        c: &CaelestiaColours,
        cx: &mut gpui::Context<NexusWindow>,
    ) -> Div {
        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .p(px(PADDING_LARGE))
            .gap(px(SPACING_LARGE))
            .overflow_hidden()
            .child(
                div()
                    .text_size(px(TypographyScale::HeadlineMedium.font_size()))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(c.on_surface))
                    .child("Panels & Docks"),
            )
            .child(Self::render_toggle_row(
                "Taskbar",
                "Per-monitor status bar, system tray, and media controls",
                true,
                NexusAction::ToggleTaskbar(false),
                c,
                cx,
            ))
            .child(Self::render_toggle_row(
                "Dashboard",
                "Full-screen overview with calendar, music player, and stats",
                true,
                NexusAction::ToggleDashboard(false),
                c,
                cx,
            ))
            .child(Self::render_toggle_row(
                "App Launcher",
                "Fuzzy search, categories, and recent application grid",
                true,
                NexusAction::ToggleLauncher(false),
                c,
                cx,
            ))
            .child(Self::render_toggle_row(
                "Dock Mode",
                "Transform taskbar into an autohiding macOS/GNOME style dock",
                false,
                NexusAction::ToggleDockMode(true),
                c,
                cx,
            ))
            .child(Self::render_toggle_row(
                "Sidebar Quick Toggles",
                "Slide-out control center with volume, brightness, and network pills",
                true,
                NexusAction::ToggleSidebarToggles(false),
                c,
                cx,
            ))
    }

    /// Stateless preview for tests or headless rendering
    pub fn render_preview(c: &CaelestiaColours) -> Div {
        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .p(px(PADDING_LARGE))
            .gap(px(SPACING_LARGE))
            .overflow_hidden()
            .child(
                div()
                    .text_size(px(TypographyScale::HeadlineMedium.font_size()))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(c.on_surface))
                    .child("Panels & Docks"),
            )
            .child(Self::render_static_row(
                "Taskbar",
                "Per-monitor status bar, system tray, and media controls",
                true,
                c,
            ))
            .child(Self::render_static_row(
                "Dashboard",
                "Full-screen overview with calendar, music player, and stats",
                true,
                c,
            ))
            .child(Self::render_static_row(
                "App Launcher",
                "Fuzzy search, categories, and recent application grid",
                true,
                c,
            ))
            .child(Self::render_static_row(
                "Dock Mode",
                "Transform taskbar into an autohiding macOS/GNOME style dock",
                false,
                c,
            ))
            .child(Self::render_static_row(
                "Sidebar Quick Toggles",
                "Slide-out control center with volume, brightness, and network pills",
                true,
                c,
            ))
    }

    fn render_static_row(
        title: impl Into<SharedString>,
        subtitle: impl Into<SharedString>,
        checked: bool,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .w_full()
            .h(px(64.0))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .px(px(PADDING_LARGE))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_NORMAL))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(c.on_surface))
                            .child(title.into()),
                    )
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_SMALL))
                            .text_color(rgba(c.on_surface_variant))
                            .child(subtitle.into()),
                    ),
            )
            .child(StyledSwitch::render(checked, false, false, false, c))
    }

    fn render_toggle_row(
        title: impl Into<SharedString>,
        subtitle: impl Into<SharedString>,
        checked: bool,
        action: NexusAction,
        c: &CaelestiaColours,
        cx: &mut gpui::Context<NexusWindow>,
    ) -> Div {
        div()
            .w_full()
            .h(px(64.0))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .px(px(PADDING_LARGE))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_NORMAL))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(c.on_surface))
                            .child(title.into()),
                    )
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_SMALL))
                            .text_color(rgba(c.on_surface_variant))
                            .child(subtitle.into()),
                    ),
            )
            .child(
                div()
                    .cursor(CursorStyle::PointingHand)
                    .on_mouse_down(MouseButton::Left, cx.listener(move |nexus, _, _, cx| {
                        nexus.dispatch(action.clone(), cx);
                    }))
                    .child(StyledSwitch::render(checked, false, false, false, c)),
            )
    }
}
