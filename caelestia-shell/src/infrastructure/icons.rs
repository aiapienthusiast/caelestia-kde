//! Freedesktop icon resolver and fallback mapper
//! Ported with 100% parity from `shell/utils/Icons.qml`.

pub struct IconResolver;

impl IconResolver {
    /// Return standard fallback icon if icon is missing or empty
    pub fn resolve(icon_name: &str, fallback: &str) -> String {
        let trimmed = icon_name.trim();
        if trimmed.is_empty() {
            fallback.to_string()
        } else {
            trimmed.to_string()
        }
    }

    /// Map common application class names to icon theme names
    pub fn map_class_to_icon(wm_class: &str) -> &'static str {
        let s = wm_class.to_lowercase();
        if s.contains("terminal") || s.contains("konsole") || s.contains("alacritty") || s.contains("kitty") || s.contains("foot") || s.contains("wezterm") {
            "utilities-terminal"
        } else if s.contains("firefox") || s.contains("chromium") || s.contains("chrome") || s.contains("brave") {
            "web-browser"
        } else if s.contains("dolphin") || s.contains("nautilus") || s.contains("thunar") {
            "system-file-manager"
        } else if s.contains("code") || s.contains("vscodium") || s.contains("kate") || s.contains("kwrite") {
            "accessories-text-editor"
        } else if s.contains("spotify") || s.contains("vlc") || s.contains("mpv") {
            "multimedia-audio-player"
        } else if s.contains("discord") || s.contains("telegram") || s.contains("slack") {
            "internet-chat"
        } else if s.contains("steam") || s.contains("heroic") || s.contains("lutris") {
            "applications-games"
        } else if s.contains("gimp") || s.contains("inkscape") || s.contains("krita") {
            "applications-graphics"
        } else {
            "application-x-executable"
        }
    }

    /// Provide standard icon name dictionary for weather states
    pub fn weather_icon(condition_code: u32, is_night: bool) -> &'static str {
        match condition_code {
            1000 => if is_night { "weather-clear-night" } else { "weather-clear" },
            1003 => if is_night { "weather-few-clouds-night" } else { "weather-few-clouds" },
            1006 | 1009 => "weather-overcast",
            1030 | 1135 => "weather-fog",
            1063 | 1180..=1201 => "weather-showers",
            1066 | 1210..=1225 => "weather-snow",
            1087 | 1273..=1282 => "weather-storm",
            _ => "weather-none-available",
        }
    }
}
