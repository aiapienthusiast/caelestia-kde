//! Categories Subsystem
//! Complete parity with `shell/modules/launcher/services/Categories.qml`.

use super::entry::DesktopEntry;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CategoryDef {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub xdg: &'static [&'static str],
}

pub const CATEGORY_DEFINITIONS: &[CategoryDef] = &[
    CategoryDef { id: "favorites", name: "Favorites", icon: "favorite", xdg: &[] },
    CategoryDef { id: "all", name: "All Applications", icon: "apps", xdg: &[] },
    CategoryDef { id: "audioVideo", name: "Audio & Video", icon: "movie", xdg: &["AudioVideo"] },
    CategoryDef { id: "audio", name: "Audio", icon: "music_note", xdg: &["Audio"] },
    CategoryDef { id: "video", name: "Video", icon: "videocam", xdg: &["Video"] },
    CategoryDef { id: "development", name: "Development", icon: "code", xdg: &["Development"] },
    CategoryDef { id: "education", name: "Education", icon: "school", xdg: &["Education"] },
    CategoryDef { id: "game", name: "Games", icon: "sports_esports", xdg: &["Game"] },
    CategoryDef { id: "graphics", name: "Graphics", icon: "palette", xdg: &["Graphics"] },
    CategoryDef { id: "network", name: "Network", icon: "public", xdg: &["Network"] },
    CategoryDef { id: "office", name: "Office", icon: "description", xdg: &["Office"] },
    CategoryDef { id: "science", name: "Science", icon: "science", xdg: &["Science"] },
    CategoryDef { id: "settings", name: "Settings", icon: "settings", xdg: &["Settings"] },
    CategoryDef { id: "system", name: "System", icon: "computer", xdg: &["System"] },
    CategoryDef { id: "utility", name: "Utilities", icon: "build", xdg: &["Utility"] },
    CategoryDef { id: "other", name: "Other", icon: "category", xdg: &[] },
];

pub struct Categories;

impl Categories {
    /// Curated category a desktop entry belongs to, or "other"
    pub fn category_for_app(entry: &DesktopEntry) -> &'static str {
        for def in CATEGORY_DEFINITIONS {
            if def.xdg.is_empty() {
                continue;
            }
            for xdg_cat in def.xdg {
                if entry.categories.iter().any(|c| c.eq_ignore_ascii_case(xdg_cat)) {
                    return def.id;
                }
            }
        }
        "other"
    }

    /// Return categories that have at least one application plus favorites and all
    pub fn visible_categories(entries: &[DesktopEntry]) -> Vec<&'static CategoryDef> {
        let mut result = Vec::new();
        for def in CATEGORY_DEFINITIONS {
            if def.id == "favorites" || def.id == "all" {
                result.push(def);
            } else if entries.iter().any(|e| Self::category_for_app(e) == def.id) {
                result.push(def);
            }
        }
        result
    }

    /// Filter entries for a specific category
    pub fn apps_for<'a>(
        category_id: &str,
        entries: &'a [DesktopEntry],
        favourite_ids: &[String],
    ) -> Vec<&'a DesktopEntry> {
        match category_id {
            "all" => entries.iter().collect(),
            "favorites" => entries
                .iter()
                .filter(|e| favourite_ids.iter().any(|fav| fav == &e.id || fav == &e.name))
                .collect(),
            "other" => entries
                .iter()
                .filter(|e| Self::category_for_app(e) == "other")
                .collect(),
            cat => entries
                .iter()
                .filter(|e| Self::category_for_app(e) == cat)
                .collect(),
        }
    }
}
