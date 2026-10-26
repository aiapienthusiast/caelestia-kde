//! Internationalization & Translations Catalog Service
//! 100% parity with C++ `Caelestia::Translations`.

use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub code: String,
    pub name: String,
    pub native_name: String,
}

pub struct TranslationsService {
    active_language: Mutex<String>,
    catalogs: Mutex<HashMap<String, HashMap<String, String>>>,
    available: Vec<LanguageInfo>,
}

impl Default for TranslationsService {
    fn default() -> Self {
        Self::new()
    }
}

impl TranslationsService {
    pub fn new() -> Self {
        let available = vec![
            LanguageInfo {
                code: "en".to_string(),
                name: "English".to_string(),
                native_name: "English".to_string(),
            },
            LanguageInfo {
                code: "es".to_string(),
                name: "Spanish".to_string(),
                native_name: "Español".to_string(),
            },
            LanguageInfo {
                code: "fr".to_string(),
                name: "French".to_string(),
                native_name: "Français".to_string(),
            },
            LanguageInfo {
                code: "de".to_string(),
                name: "German".to_string(),
                native_name: "Deutsch".to_string(),
            },
            LanguageInfo {
                code: "ja".to_string(),
                name: "Japanese".to_string(),
                native_name: "日本語".to_string(),
            },
            LanguageInfo {
                code: "zh_CN".to_string(),
                name: "Chinese (Simplified)".to_string(),
                native_name: "简体中文".to_string(),
            },
            LanguageInfo {
                code: "ru".to_string(),
                name: "Russian".to_string(),
                native_name: "Русский".to_string(),
            },
            LanguageInfo {
                code: "pt_BR".to_string(),
                name: "Portuguese (Brazil)".to_string(),
                native_name: "Português (Brasil)".to_string(),
            },
        ];

        let mut catalogs = HashMap::new();
        // Seed standard translations
        let mut es = HashMap::new();
        es.insert("Good Morning".to_string(), "Buenos días".to_string());
        es.insert("Good Afternoon".to_string(), "Buenas tardes".to_string());
        es.insert("Good Evening".to_string(), "Buenas noches".to_string());
        es.insert("Settings".to_string(), "Configuración".to_string());
        catalogs.insert("es".to_string(), es);

        Self {
            active_language: Mutex::new("en".to_string()),
            catalogs: Mutex::new(catalogs),
            available,
        }
    }

    pub fn active_language(&self) -> String {
        self.active_language.lock().unwrap().clone()
    }

    pub fn set_language(&self, code: &str) {
        if let Ok(mut lang) = self.active_language.lock() {
            *lang = code.to_string();
        }
    }

    pub fn available_languages(&self) -> &[LanguageInfo] {
        &self.available
    }

    pub fn native_name_for(&self, code: &str) -> Option<&str> {
        self.available
            .iter()
            .find(|l| l.code.eq_ignore_ascii_case(code))
            .map(|l| l.native_name.as_str())
    }

    pub fn name_for(&self, code: &str) -> Option<&str> {
        self.available
            .iter()
            .find(|l| l.code.eq_ignore_ascii_case(code))
            .map(|l| l.name.as_str())
    }

    /// Translate string with fallback to key
    pub fn tr(&self, key: &str) -> String {
        let lang = self.active_language();
        if lang == "en" {
            return key.to_string();
        }

        if let Ok(catalogs) = self.catalogs.lock() {
            if let Some(cat) = catalogs.get(&lang) {
                if let Some(val) = cat.get(key) {
                    return val.clone();
                }
            }
        }

        key.to_string()
    }

    /// Register catalog translations for language code
    pub fn add_translations(&self, code: &str, entries: HashMap<String, String>) {
        if let Ok(mut catalogs) = self.catalogs.lock() {
            catalogs.entry(code.to_string()).or_default().extend(entries);
        }
    }
}
