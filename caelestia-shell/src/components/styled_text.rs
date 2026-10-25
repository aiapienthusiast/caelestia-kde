//! StyledText
//! Parity with `shell/components/StyledText.qml`.

use crate::domain::tokens::*;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString, Styled};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypographyScale {
    DisplayLarge,
    HeadlineLarge,
    HeadlineMedium,
    TitleLarge,
    TitleMedium,
    TitleSmall,
    BodyLarge,
    BodyMedium,
    BodySmall,
    LabelLarge,
    LabelMedium,
    LabelSmall,
}

impl TypographyScale {
    pub fn font_size(self) -> f32 {
        match self {
            Self::DisplayLarge => 57.0,
            Self::HeadlineLarge => 32.0,
            Self::HeadlineMedium => 28.0,
            Self::TitleLarge => 22.0,
            Self::TitleMedium => 16.0,
            Self::TitleSmall => 14.0,
            Self::BodyLarge => 16.0,
            Self::BodyMedium => 14.0,
            Self::BodySmall => FONT_SIZE_SMALLER, // 12.0
            Self::LabelLarge => 14.0,
            Self::LabelMedium => FONT_SIZE_SMALLER, // 12.0
            Self::LabelSmall => FONT_SIZE_SMALL,   // 11.0
        }
    }

    pub fn default_weight(self) -> FontWeight {
        match self {
            Self::DisplayLarge | Self::HeadlineLarge | Self::HeadlineMedium => FontWeight::NORMAL,
            Self::TitleLarge | Self::TitleMedium | Self::TitleSmall => FontWeight::MEDIUM,
            Self::BodyLarge | Self::BodyMedium | Self::BodySmall => FontWeight::NORMAL,
            Self::LabelLarge | Self::LabelMedium | Self::LabelSmall => FontWeight::MEDIUM,
        }
    }
}

pub struct StyledText;

impl StyledText {
    /// Renders text with M3 typography scale and color.
    pub fn new(text: impl Into<SharedString>) -> Div {
        div()
            .text_size(px(TypographyScale::BodySmall.font_size()))
            .font_weight(TypographyScale::BodySmall.default_weight())
            .child(text.into())
    }

    /// Renders text with a specific M3 typography scale.
    pub fn with_scale(text: impl Into<SharedString>, scale: TypographyScale) -> Div {
        div()
            .text_size(px(scale.font_size()))
            .font_weight(scale.default_weight())
            .child(text.into())
    }

    /// Renders text with custom font size, weight, and color.
    pub fn custom(
        text: impl Into<SharedString>,
        size_px: f32,
        weight: FontWeight,
        color: u32,
    ) -> Div {
        div()
            .text_size(px(size_px))
            .font_weight(weight)
            .text_color(rgba(color))
            .child(text.into())
    }
}

pub trait StyledTextExt {
    fn color(self, color: u32) -> Self;
    fn font_size(self, size: f32) -> Self;
    fn icon_size(self, size: f32) -> Self;
}

impl StyledTextExt for Div {
    fn color(self, color: u32) -> Self {
        self.text_color(rgba(color))
    }
    fn font_size(self, size: f32) -> Self {
        self.text_size(px(size))
    }
    fn icon_size(self, size: f32) -> Self {
        self.text_size(px(size))
    }
}

