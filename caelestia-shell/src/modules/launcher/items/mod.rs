//! Launcher search result item delegates
//! Ported with 100% parity from `shell/modules/launcher/items/`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalcResultItem {
    pub expression: String,
    pub result: String,
    pub copy_payload: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmojiResultItem {
    pub emoji: String,
    pub name: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardResultItem {
    pub id: u64,
    pub preview_text: String,
    pub is_image: bool,
    pub timestamp_str: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowResultItem {
    pub window_id: u32,
    pub title: String,
    pub app_id: String,
    pub workspace_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemeResultItem {
    pub scheme_name: String,
    pub is_dark: bool,
    pub primary_hex: String,
    pub secondary_hex: String,
    pub surface_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariantResultItem {
    pub variant_name: String, // e.g. "TonalSpot", "Content", "Expressive", "FruitSalad"
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionLauncherItem {
    pub action_id: String,
    pub title: String,
    pub description: String,
    pub icon_name: String,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LauncherSearchResult {
    App(super::entry::DesktopEntry),
    Calc(CalcResultItem),
    Emoji(EmojiResultItem),
    Clipboard(ClipboardResultItem),
    Window(WindowResultItem),
    Scheme(SchemeResultItem),
    Variant(VariantResultItem),
    Action(ActionLauncherItem),
}

impl LauncherSearchResult {
    pub fn title(&self) -> &str {
        match self {
            Self::App(app) => &app.name,
            Self::Calc(calc) => &calc.result,
            Self::Emoji(emoji) => &emoji.name,
            Self::Clipboard(clip) => &clip.preview_text,
            Self::Window(win) => &win.title,
            Self::Scheme(sch) => &sch.scheme_name,
            Self::Variant(var) => &var.variant_name,
            Self::Action(act) => &act.title,
        }
    }

    pub fn subtitle(&self) -> &str {
        match self {
            Self::App(app) => &app.comment,
            Self::Calc(calc) => &calc.expression,
            Self::Emoji(emoji) => &emoji.emoji,
            Self::Clipboard(clip) => &clip.timestamp_str,
            Self::Window(win) => &win.app_id,
            Self::Scheme(_) => "Material 3 Theme",
            Self::Variant(var) => &var.description,
            Self::Action(act) => &act.description,
        }
    }
}
