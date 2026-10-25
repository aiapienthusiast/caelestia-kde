use gpui::*;

use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::icon_button::IconButton;
use crate::infrastructure::theme::CaelestiaColours;

/// Setting Reset Button state and configuration options.
#[derive(Clone, Debug)]
pub struct SettingResetOptions {
    /// Current value as a string representation or numeric float.
    pub current_str: Option<String>,
    pub current_num: Option<f64>,
    /// Fallback default value.
    pub default_str: Option<String>,
    pub default_num: Option<f64>,
}

impl SettingResetOptions {
    pub fn new_numeric(current: f64, default_val: f64) -> Self {
        Self {
            current_str: None,
            current_num: Some(current),
            default_str: None,
            default_num: Some(default_val),
        }
    }

    pub fn new_string(current: impl Into<String>, default_val: impl Into<String>) -> Self {
        Self {
            current_str: Some(current.into()),
            current_num: None,
            default_str: Some(default_val.into()),
            default_num: None,
        }
    }

    /// Evaluates if the current value differs from the default fallback value.
    pub fn is_dirty(&self) -> bool {
        if let (Some(curr), Some(def)) = (self.current_num, self.default_num) {
            (curr - def).abs() > 0.005
        } else if let (Some(curr), Some(def)) = (&self.current_str, self.default_str.as_ref()) {
            curr != def
        } else {
            false
        }
    }
}

/// Reset button for settings options (port of `SettingResetButton.qml`).
///
/// Occupies its slot at the right edge of configuration rows so rows never shift;
/// dimmed and inert until the value differs from the default schema fallback.
#[derive(Clone)]
pub struct SettingResetButton {
    pub active: bool,
    pub dirty: bool,
    pub latched: bool,
    pub colours: Option<CaelestiaColours>,
}

impl SettingResetButton {
    pub fn new() -> Self {
        Self {
            active: true,
            dirty: false,
            latched: false,
            colours: None,
        }
    }

    pub fn with_options(mut self, options: Option<&SettingResetOptions>) -> Self {
        if let Some(opts) = options {
            self.active = true;
            self.dirty = opts.is_dirty();
            if self.dirty {
                self.latched = true;
            }
        } else {
            self.active = false;
            self.dirty = false;
        }
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        if dirty {
            self.latched = true;
        }
        self
    }

    pub fn with_colours(mut self, colours: CaelestiaColours) -> Self {
        self.colours = Some(colours);
        self
    }

    pub fn is_disabled(&self) -> bool {
        !(self.dirty || self.latched)
    }

    pub fn reset(&mut self) {
        if !self.active {
            return;
        }
        self.latched = false;
    }
}

impl Default for SettingResetButton {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for SettingResetButton {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        if !self.active {
            return div();
        }

        let disabled = self.is_disabled();
        let colours = self.colours.unwrap_or_default();

        let mut style = ButtonStyle::default();
        style.button_type = ButtonType::Text;
        style.is_disabled = disabled;

        IconButton::render("restart_alt", &style, &colours, Some(32.0))
    }
}
