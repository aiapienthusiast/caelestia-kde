//! Design tokens for Caelestia Shell, matching Caelestia M3 specifications
//! directly ported from `shell/plugin/src/Caelestia/Config/tokens.hpp`.

#![allow(dead_code)]

// --- Rounding Tokens ---
pub const ROUNDING_EXTRA_SMALL: f32 = 4.0;
pub const ROUNDING_SMALL: f32 = 8.0;
pub const ROUNDING_MEDIUM: f32 = 12.0;
pub const ROUNDING_LARGE: f32 = 16.0;
pub const ROUNDING_LARGE_INCREASED: f32 = 20.0;
pub const ROUNDING_EXTRA_LARGE: f32 = 28.0;
pub const ROUNDING_EXTRA_LARGE_INCREASED: f32 = 32.0;
pub const ROUNDING_EXTRA_EXTRA_LARGE: f32 = 48.0;
pub const ROUNDING_NORMAL: f32 = 24.0;
pub const ROUNDING_FULL: f32 = 9999.0;

// --- Spacing Tokens ---
pub const SPACING_EXTRA_SMALL: f32 = 4.0;
pub const SPACING_SMALL: f32 = 8.0;
pub const SPACING_MEDIUM: f32 = 12.0;
pub const SPACING_LARGE: f32 = 16.0;
pub const SPACING_LARGE_INCREASED: f32 = 20.0;
pub const SPACING_EXTRA_LARGE: f32 = 28.0;
pub const SPACING_EXTRA_LARGE_INCREASED: f32 = 32.0;
pub const SPACING_EXTRA_EXTRA_LARGE: f32 = 48.0;
pub const SPACING_NORMAL: f32 = 12.0;

// --- Padding Tokens ---
pub const PADDING_EXTRA_SMALL: f32 = 4.0;
pub const PADDING_SMALL: f32 = 8.0;
pub const PADDING_MEDIUM: f32 = 12.0;
pub const PADDING_LARGE: f32 = 16.0;
pub const PADDING_LARGE_INCREASED: f32 = 20.0;
pub const PADDING_EXTRA_LARGE: f32 = 28.0;
pub const PADDING_EXTRA_LARGE_INCREASED: f32 = 32.0;
pub const PADDING_EXTRA_EXTRA_LARGE: f32 = 48.0;
pub const PADDING_NORMAL: f32 = 16.0;

// --- Font Size Tokens ---
pub const FONT_SIZE_SMALL: f32 = 11.0;
pub const FONT_SIZE_SMALLER: f32 = 12.0;
pub const FONT_SIZE_NORMAL: f32 = 13.0;
pub const FONT_SIZE_LARGER: f32 = 15.0;
pub const FONT_SIZE_LARGE: f32 = 18.0;
pub const FONT_SIZE_EXTRA_LARGE: f32 = 28.0;
pub const FONT_SIZE_MONO_SMALL: f32 = 12.0;
pub const FONT_SIZE_MONO_MEDIUM: f32 = 14.0;
pub const FONT_SIZE_MONO_LARGE: f32 = 16.0;

// Material 3 Typography Scale aliases
pub const FONT_SIZE_BODY_SMALL: f32 = 12.0;
pub const FONT_SIZE_BODY_MEDIUM: f32 = 14.0;
pub const FONT_SIZE_BODY_LARGE: f32 = 16.0;
pub const FONT_SIZE_LABEL_SMALL: f32 = 11.0;
pub const FONT_SIZE_LABEL_MEDIUM: f32 = 12.0;
pub const FONT_SIZE_LABEL_LARGE: f32 = 14.0;
pub const FONT_SIZE_TITLE_SMALL: f32 = 14.0;
pub const FONT_SIZE_TITLE_MEDIUM: f32 = 16.0;
pub const FONT_SIZE_TITLE_LARGE: f32 = 22.0;
pub const FONT_SIZE_HEADLINE_SMALL: f32 = 24.0;
pub const FONT_SIZE_HEADLINE_MEDIUM: f32 = 28.0;
pub const FONT_SIZE_HEADLINE_LARGE: f32 = 32.0;

// Icon sizes
pub const ICON_SIZE_SMALL: f32 = 16.0;
pub const ICON_SIZE_MEDIUM: f32 = 22.0;
pub const ICON_SIZE_LARGE: f32 = 28.0;
pub const ICON_SIZE_EXTRA_LARGE: f32 = 36.0;

// --- Animation Duration Tokens (in milliseconds) ---
pub const ANIM_DURATION_SMALL: u64 = 200;
pub const ANIM_DURATION_NORMAL: u64 = 400;
pub const ANIM_DURATION_LARGE: u64 = 600;
pub const ANIM_DURATION_EXTRA_LARGE: u64 = 1000;
pub const ANIM_DURATION_EXPRESSIVE_FAST_SPATIAL: u64 = 350;
pub const ANIM_DURATION_EXPRESSIVE_DEFAULT_SPATIAL: u64 = 500;
pub const ANIM_DURATION_EXPRESSIVE_SLOW_SPATIAL: u64 = 650;
pub const ANIM_DURATION_EXPRESSIVE_FAST_EFFECTS: u64 = 150;
pub const ANIM_DURATION_EXPRESSIVE_DEFAULT_EFFECTS: u64 = 200;
pub const ANIM_DURATION_EXPRESSIVE_SLOW_EFFECTS: u64 = 300;

// --- Component Geometry Dimensions ---
pub const DRAWER_WIDTH: f32 = 116.0;
pub const HITBOX_WIDTH: f32 = 4.0;

pub const BUTTON_WIDTH: f32 = 76.0;
pub const BUTTON_HEIGHT: f32 = 76.0;

// Controls Dimensions
pub const CHECKBOX_SIZE: f32 = 24.0;
pub const RADIO_SIZE: f32 = 20.0;
pub const RADIO_INNER_SIZE: f32 = 8.0;
pub const SWITCH_TRACK_WIDTH: f32 = 52.0;
pub const SWITCH_TRACK_HEIGHT: f32 = 32.0;
pub const SWITCH_THUMB_SIZE: f32 = 24.0;
pub const SLIDER_HEIGHT: f32 = 12.0;
pub const PROGRESS_BAR_HEIGHT: f32 = 4.0;
