//! About page for Caelestia Shell.
//! Parity with `shell/modules/nexus/pages/AboutPage.qml`.

use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::config::ShellConfig;
use crate::infrastructure::theme::CaelestiaColours;
use crate::modules::nexus::window::NexusWindow;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString, Styled};

pub struct AboutPage;

impl AboutPage {
    pub fn render(
        _config: &ShellConfig,
        c: &CaelestiaColours,
        _cx: &mut gpui::Context<NexusWindow>,
    ) -> Div {
        Self::render_preview(c)
    }

    pub fn render_preview(c: &CaelestiaColours) -> Div {
        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .p(px(PADDING_LARGE))
            .gap(px(SPACING_LARGE))
            .overflow_hidden()
            // Hero Mascot & Title
            .child(
                div()
                    .w_full()
                    .p(px(PADDING_LARGE))
                    .rounded(px(ROUNDING_LARGE))
                    .bg(rgba(c.surface_container))
                    .flex()
                    .items_center()
                    .gap(px(SPACING_LARGE))
                    .child(
                        div()
                            .text_size(px(48.0))
                            .child("🦖"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(TypographyScale::HeadlineLarge.font_size()))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgba(c.primary))
                                    .child("Caelestia Shell"),
                            )
                            .child(
                                div()
                                    .text_size(px(FONT_SIZE_NORMAL))
                                    .text_color(rgba(c.on_surface_variant))
                                    .child("GPU-accelerated desktop experience for KDE Plasma 6 Wayland"),
                            ),
                    ),
            )
            // System Details
            .child(Self::render_spec_row("Shell Version", "1.0.0-beta.25 (Rust Edition)", c))
            .child(Self::render_spec_row("Rendering Engine", "GPUI with Vulkan / WGPU rasterizer", c))
            .child(Self::render_spec_row("Compositor Protocol", "Wayland Layer-Shell + KWin ext_background_effect", c))
            .child(Self::render_spec_row("Theme Engine", "Material 3 Dynamic Chroma Palette", c))
    }

    fn render_spec_row(
        label: impl Into<SharedString>,
        value: impl Into<SharedString>,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .w_full()
            .h(px(52.0))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .px(px(PADDING_LARGE))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_size(px(FONT_SIZE_NORMAL))
                    .text_color(rgba(c.on_surface_variant))
                    .child(label.into()),
            )
            .child(
                div()
                    .text_size(px(FONT_SIZE_NORMAL))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(c.on_surface))
                    .child(value.into()),
            )
    }
}
