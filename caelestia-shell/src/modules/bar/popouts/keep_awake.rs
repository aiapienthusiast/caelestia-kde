//! KeepAwake Popout
//! 100% Parity with `shell/modules/bar/popouts/KeepAwake.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::icon_text_button::IconTextButton;
use crate::components::controls::toggle_row::ToggleRow;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InhibitPreset {
    #[default]
    Indefinite,
    Minutes15,
    Minutes30,
    Hour1,
    Hours2,
}

impl InhibitPreset {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Indefinite => "Indefinite",
            Self::Minutes15 => "15m",
            Self::Minutes30 => "30m",
            Self::Hour1 => "1h",
            Self::Hours2 => "2h",
        }
    }
}

/// State for the KeepAwake popout (port of `IdleInhibit.qml`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeepAwakePopoutState {
    pub enabled: bool,
    pub active_preset: InhibitPreset,
    pub elapsed_seconds: u64,
}

impl Default for KeepAwakePopoutState {
    fn default() -> Self {
        Self {
            enabled: false,
            active_preset: InhibitPreset::Indefinite,
            elapsed_seconds: 0,
        }
    }
}

pub struct KeepAwakePopout;

impl KeepAwakePopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &KeepAwakePopoutState::default(), c)
    }

    /// Renders the KeepAwake popout with full parity to KeepAwake.qml.
    pub fn render_with_state(center_x: f32, state: &KeepAwakePopoutState, c: &CaelestiaColours) -> Div {
        let width_px = 320.0;

        let mut content = div()
            .flex()
            .flex_col()
            .w(px(width_px))
            .gap(px(SPACING_SMALL));

        // 1. Header: "Keep Awake"
        content = content.child(
            StyledText::new("Keep Awake")
                .text_size(px(FONT_SIZE_NORMAL))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.on_surface)),
        );

        // 2. Main Card with Inhibit Toggle & Duration Presets
        let mut card = div()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL));

        let subtitle = if state.enabled {
            "Display sleep and auto-suspend are blocked"
        } else {
            "Standard power saving schedule active"
        };

        card = card.child(ToggleRow::render(
            "Prevent Sleep",
            Some(subtitle),
            Some("coffee"),
            state.enabled,
            false,
            false,
            false,
            c,
        ));

        // Presets row: 15m, 30m, 1h, 2h, Indefinite
        if state.enabled {
            card = card.child(
                div()
                    .pt(px(SPACING_SMALL))
                    .child(
                        StyledText::new("Duration preset")
                            .text_size(px(FONT_SIZE_SMALL))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(c.on_surface_variant)),
                    ),
            );

            let presets = [
                InhibitPreset::Minutes15,
                InhibitPreset::Minutes30,
                InhibitPreset::Hour1,
                InhibitPreset::Hours2,
                InhibitPreset::Indefinite,
            ];

            let mut presets_row = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(SPACING_EXTRA_SMALL));

            for p in presets {
                let is_active = state.active_preset == p;
                let mut btn_style = ButtonStyle::default();
                btn_style.button_type = if is_active {
                    ButtonType::Filled
                } else {
                    ButtonType::Tonal
                };

                presets_row = presets_row.child(IconTextButton::render(
                    "timer",
                    p.label(),
                    &btn_style,
                    c,
                ));
            }

            card = card.child(presets_row);
        }

        content = content.child(card);

        PopoutWrapper::render("Keep Awake", "coffee", center_x, content, c)
    }
}
