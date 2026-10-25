//! Desktop & Tiling settings page.
//! Parity with `shell/modules/nexus/pages/DesktopPage.qml` and `KrohnkitePage.qml`.

use crate::components::controls::switch::StyledSwitch;
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString, Styled};

pub struct DesktopPage;

impl DesktopPage {
    pub fn render(c: &CaelestiaColours) -> Div {
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
                    .child("Desktop & Window Tiling"),
            )
            .child(Self::render_row(
                "Krohnkite Window Tiling",
                "Dynamic spiral and bsp tiling for KWin Wayland",
                true,
                c,
            ))
            .child(Self::render_row(
                "Desktop Icons & Addons",
                "Show desktop files, widgets, and quick launcher icons",
                true,
                c,
            ))
            .child(Self::render_row(
                "Enhanced Context Menu",
                "Caelestia styled right-click actions on desktop background",
                true,
                c,
            ))
            .child(Self::render_row(
                "Virtual Workspaces Indicator",
                "Per-monitor active workspace strip",
                true,
                c,
            ))
    }

    fn render_row(
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
}
