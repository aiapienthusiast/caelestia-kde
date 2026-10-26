//! AppearancePage
//! Parity with `shell/modules/nexus/pages/wallandstyle/AppearancePage.qml`.

use crate::components::controls::button_base::ButtonStyle;
use crate::components::controls::icon_text_button::IconTextButton;
use crate::components::controls::slider::StyledSlider;
use crate::components::controls::switch::StyledSwitch;
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::config::ShellConfig;
use crate::infrastructure::theme::CaelestiaColours;
use crate::modules::nexus::window::NexusWindow;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, MouseButton,
    MouseDownEvent, SharedString, Styled, Window,
};

pub struct AppearancePage;

impl AppearancePage {
    /// Renders the Appearance & Theme settings page with direct ShellConfig binding and live interactive mutations.
    pub fn render(
        config: &ShellConfig,
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
            // Header Bar
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(TypographyScale::HeadlineMedium.font_size()))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgba(c.on_surface))
                            .child("Theme & Effects"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(SPACING_MEDIUM))
                            .child(
                                div()
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|_nexus, _, _, _cx| {
                                            tracing::info!("Restarting Caelestia Shell...");
                                            let _ = std::process::Command::new(
                                                "/home/ladybug-me/caelestia-kde/shell/scripts/restart_shell.sh",
                                            )
                                            .spawn();
                                        }),
                                    )
                                    .child(IconTextButton::render(
                                        "🔄",
                                        "Restart Shell",
                                        &ButtonStyle::default(),
                                        c,
                                    )),
                            )
                            .child(
                                div()
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|nexus, _, _, cx| {
                                            nexus.reset_appearance_settings(cx);
                                        }),
                                    )
                                    .child(IconTextButton::render(
                                        "↩️",
                                        "Reset settings",
                                        &ButtonStyle {
                                            button_type:
                                                crate::components::controls::button_base::ButtonType::Tonal,
                                            ..Default::default()
                                        },
                                        c,
                                    )),
                            ),
                    ),
            )
            // Scrollable Content
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(px(SPACING_LARGE))
                    .overflow_hidden()
                    // Section 1: Display & Bezel Mode
                    .child(Self::render_toggle_row(
                        "Bezel mode (Pitch black)",
                        "Make the shell pitch black to blend with display bezels",
                        config.pitch_black(),
                        c,
                        cx.listener(|nexus, _, _, cx| {
                            nexus.toggle_pitch_black(cx);
                        }),
                    ))
                    // Section 2: Transparency
                    .child(Self::render_toggle_row(
                        "Transparency",
                        "Enable translucency effects across the shell",
                        config.transparency_enabled(),
                        c,
                        cx.listener(|nexus, _, _, cx| {
                            nexus.toggle_transparency(cx);
                        }),
                    ))
                    // Section 3: Blur
                    .child(Self::render_toggle_row(
                        "KWin Frosted Blur",
                        "Compositor dual-filter background blur for translucent surfaces",
                        config.blur_enabled(),
                        c,
                        cx.listener(|nexus, _, _, cx| {
                            nexus.toggle_blur(cx);
                        }),
                    ))
                    // Section 4: Base Opacity Slider
                    .child(Self::render_slider_row(
                        "Base opacity",
                        "Adjust transparency level of root surfaces",
                        config.transparency_base() as f32,
                        format!("{:.0}%", config.transparency_base() * 100.0),
                        c,
                        cx.listener(|nexus, _, _, cx| {
                            let current = nexus.config.transparency_base();
                            let next = if current >= 0.95 { 0.4 } else { (current + 0.15).min(1.0) };
                            nexus.set_base_opacity(next, cx);
                        }),
                    ))
                    // Section 5: Layers Opacity Slider
                    .child(Self::render_slider_row(
                        "Layers opacity",
                        "Opacity of stacked containers and cards",
                        config.transparency_layers() as f32,
                        format!("{:.0}%", config.transparency_layers() * 100.0),
                        c,
                        cx.listener(|nexus, _, _, cx| {
                            let current = nexus.config.transparency_layers();
                            let next = if current >= 0.95 { 0.5 } else { (current + 0.15).min(1.0) };
                            nexus.config.set_transparency_layers(next);
                            let _ = nexus.config.save();
                            cx.notify();
                        }),
                    ))
                    // Section 6: Corner Radius Scale
                    .child(Self::render_slider_row(
                        "Corner radius scale",
                        "Multiplies the shell's corner rounding tokens",
                        ((config.rounding_scale() - 0.5) / 1.5) as f32,
                        format!("{:.2}x", config.rounding_scale()),
                        c,
                        cx.listener(|nexus, _, _, cx| {
                            let current = nexus.config.rounding_scale();
                            let next = if current >= 1.9 { 0.75 } else { current + 0.25 };
                            nexus.set_corner_radius_scale(next, cx);
                        }),
                    ))
                    // Section 7: Border Thickness
                    .child(Self::render_slider_row(
                        "Border thickness",
                        "Shell outer border thickness in pixels (0 for borderless)",
                        (config.border_thickness() / 20.0) as f32,
                        format!("{} px", config.border_thickness().round() as i64),
                        c,
                        cx.listener(|nexus, _, _, cx| {
                            let current = nexus.config.border_thickness();
                            let next = if current >= 5.0 { 0.0 } else { current + 1.0 };
                            nexus.set_border_thickness(next, cx);
                        }),
                    )),
            )
    }

    fn render_toggle_row(
        title: impl Into<SharedString>,
        subtitle: impl Into<SharedString>,
        is_checked: bool,
        c: &CaelestiaColours,
        on_click: impl Fn(&MouseDownEvent, &mut Window, &mut gpui::App) + 'static,
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
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(MouseButton::Left, on_click)
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
            .child(StyledSwitch::render(
                is_checked,
                false,
                false,
                false,
                c,
            ))
    }

    fn render_slider_row(
        title: impl Into<SharedString>,
        subtitle: impl Into<SharedString>,
        value: f32,
        value_text: impl Into<SharedString>,
        c: &CaelestiaColours,
        on_click: impl Fn(&MouseDownEvent, &mut Window, &mut gpui::App) + 'static,
    ) -> Div {
        div()
            .w_full()
            .h(px(72.0))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .px(px(PADDING_LARGE))
            .flex()
            .items_center()
            .justify_between()
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(MouseButton::Left, on_click)
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
                    .flex()
                    .items_center()
                    .gap(px(SPACING_LARGE))
                    .child(StyledSlider::render(
                        value,
                        180.0,
                        false,
                        false,
                        false,
                        c,
                    ))
                    .child(
                        div()
                            .w(px(50.0))
                            .text_size(px(FONT_SIZE_SMALL))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgba(c.primary))
                            .child(value_text.into()),
                    ),
            )
    }
}
