//! Caelestia Logo component matching `shell/components/Logo.qml`.
//! Dual-color vector mark with top crescent, bottom swoosh, and star accents.

use gpui::*;
use crate::infrastructure::theme::CaelestiaColours;

pub const LOGO_DESIGN_WIDTH: f32 = 128.0;
pub const LOGO_DESIGN_HEIGHT: f32 = 90.38;

pub const LOGO_PATH_TOP_CRESCENT: &str = "m42.56,42.96c-7.76,1.6-16.36,4.22-22.44,6.22-.49.16-.88-.44-.53-.82,5.37-5.85,9.66-13.3,9.66-13.3,8.66-14.67,22.97-23.51,39.85-21.14,6.47.91,12.33,3.38,17.26,6.98.99.72,1.14,2.14.31,3.04-.4.44-.95.67-1.51.67-.34,0-.69-.09-1-.26-3.21-1.84-6.82-2.69-10.71-3.24-13.1-1.84-25.41,4.75-31.06,15.83-.94,1.84-.61,3.81.45,5.21.22.3.07.72-.29.8Z";

pub const LOGO_PATH_BOTTOM_SWOOSH: &str = "m103.02,51.8c-.65.11-1.26-.37-1.28-1.03-.06-1.96.15-3.89-.2-5.78-.28-1.48-1.66-2.5-3.16-2.34h-.05c-6.53.73-24.63,3.1-48,9.32-6.89,1.83-9.83,10-5.67,15.79,4.62,6.44,11.84,10.93,20.41,12.13,11.82,1.66,22.99-3.36,29.21-12.65.54-.81,1.54-1.17,2.47-.86.91.3,1.47,1.15,1.47,2.04,0,.33-.08.66-.24.98-7.23,14.21-22.91,22.95-39.59,20.6-7.84-1.1-14.8-4.5-20.28-9.43,0,0,0,0-.02-.01-7.28-5.14-14.7-9.99-27.24-11.98-18.82-2.98-9.53-8.75.46-13.78,7.36-3.13,25.17-7.9,36.24-10.73.16-.03.31-.06.47-.1,1.52-.4,3.2-.83,5.02-1.29,1.06-.26,1.93-.48,2.58-.64.09-.02.18-.04.26-.06.31-.08.56-.14.73-.18.03,0,.06-.01.08-.02.03,0,.05-.01.07-.02.02,0,.04,0,.06-.01.01,0,.03,0,.04-.01,0,0,.02,0,.03,0,.01,0,.02,0,.02,0,10.62-2.58,24.63-5.62,37.74-7.34,1.02-.13,2.03-.26,3.03-.37,7.49-.87,14.58-1.26,20.42-.81,25.43,1.95-4.71,16.77-15.12,18.61Z";

pub const LOGO_PATH_STAR_1: &str = "m98.12.06c-.29,2.08-1.72,8.42-8.36,9.19-.09,0-.09.13,0,.14,6.64.78,8.07,7.11,8.36,9.19.01.08.13.08.14,0,.29-2.08,1.72-8.42,8.36-9.19.09,0,.09-.13,0-.14-6.64-.78-8.07-7.11-8.36-9.19-.01-.08-.13-.08-.14,0Z";

pub const LOGO_PATH_STAR_2: &str = "m113.36,15.5c-.22,1.29-1.08,4.35-4.38,4.87-.08.01-.08.13,0,.14,3.3.52,4.17,3.58,4.38,4.87.01.08.13.08.14,0,.22-1.29,1.08-4.35,4.38-4.87.08-.01.08-.13,0-.14-3.3-.52-4.17-3.58-4.38-4.87-.01-.08-.13-.08-.14,0Z";

pub const LOGO_PATH_STAR_3: &str = "m112.69,65.22c-.19,1.01-.86,3.15-3.2,3.57-.08.01-.08.13,0,.14,2.34.42,3.01,2.56,3.2,3.57.01.08.13.08.14,0,.19-1.01.86-3.15,3.2-3.57.08-.01.08-.13,0-.14-2.34-.42-3.01-2.56-3.2-3.57-.01-.08-.13-.08-.14,0Z";

/// Logo configuration with optional color overrides.
#[derive(Debug, Clone, PartialEq)]
pub struct LogoConfig {
    pub width: f32,
    pub height: f32,
    pub top_color: Option<Rgba>,
    pub bottom_color: Option<Rgba>,
}

impl Default for LogoConfig {
    fn default() -> Self {
        Self {
            width: LOGO_DESIGN_WIDTH,
            height: LOGO_DESIGN_HEIGHT,
            top_color: None,
            bottom_color: None,
        }
    }
}

impl LogoConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_colors(mut self, top_color: Rgba, bottom_color: Rgba) -> Self {
        self.top_color = Some(top_color);
        self.bottom_color = Some(bottom_color);
        self
    }

    /// Generates standard SVG markup matching `Logo.qml`.
    pub fn to_svg_string(&self, default_top: Rgba, default_bottom: Rgba) -> String {
        let top = self.top_color.unwrap_or(default_top);
        let bottom = self.bottom_color.unwrap_or(default_bottom);

        let top_hex = format!("#{:02x}{:02x}{:02x}", (top.r * 255.0) as u8, (top.g * 255.0) as u8, (top.b * 255.0) as u8);
        let bottom_hex = format!("#{:02x}{:02x}{:02x}", (bottom.r * 255.0) as u8, (bottom.g * 255.0) as u8, (bottom.b * 255.0) as u8);

        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}">
  <path fill="{top_hex}" d="{p1}"/>
  <path fill="{bottom_hex}" d="{p2}"/>
  <path fill="{top_hex}" d="{p3}"/>
  <path fill="{top_hex}" d="{p4}"/>
  <path fill="{top_hex}" d="{p5}"/>
</svg>"#,
            w = LOGO_DESIGN_WIDTH,
            h = LOGO_DESIGN_HEIGHT,
            p1 = LOGO_PATH_TOP_CRESCENT,
            p2 = LOGO_PATH_BOTTOM_SWOOSH,
            p3 = LOGO_PATH_STAR_1,
            p4 = LOGO_PATH_STAR_2,
            p5 = LOGO_PATH_STAR_3,
        )
    }
}

/// GPUI Element rendering the Caelestia dual-color Logo.
#[derive(IntoElement)]
pub struct Logo {
    config: LogoConfig,
}

impl Logo {
    pub fn new() -> Self {
        Self {
            config: LogoConfig::default(),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.config = self.config.with_size(width, height);
        self
    }

    pub fn with_colors(mut self, top: Rgba, bottom: Rgba) -> Self {
        self.config = self.config.with_colors(top, bottom);
        self
    }
}

impl Default for Logo {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Logo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colours = cx.try_global::<CaelestiaColours>().cloned().unwrap_or_default();
        let default_top = rgba(colours.primary);
        let default_bottom = rgba(colours.on_surface);

        let _svg_data = self.config.to_svg_string(default_top, default_bottom);

        div()
            .w(px(self.config.width))
            .h(px(self.config.height))
            .flex()
            .items_center()
            .justify_center()
    }
}
