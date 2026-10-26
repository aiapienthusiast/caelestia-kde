//! Emoji Database & Search Engine Service
//! 100% parity with C++ `Caelestia::EmojiDb`.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmojiEntry {
    pub ch: String,
    pub name: String,
    pub category: String,
    pub keywords: Vec<String>,
}

pub struct EmojiDbService {
    emojis: Vec<EmojiEntry>,
    frequencies: Mutex<HashMap<String, u32>>,
    freq_path: PathBuf,
}

impl Default for EmojiDbService {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        let freq_path = PathBuf::from(home).join(".local/state/caelestia/emoji_freq.json");
        Self::new(freq_path)
    }
}

impl EmojiDbService {
    pub fn new(freq_path: PathBuf) -> Self {
        let mut service = Self {
            emojis: Self::seed_emojis(),
            frequencies: Mutex::new(HashMap::new()),
            freq_path,
        };
        service.load_frequencies();
        service
    }

    pub fn count(&self) -> usize {
        self.emojis.len()
    }

    /// Search emojis with prefix matching, keyword matching, and frequency boosting
    pub fn search(&self, query: &str, limit: usize, favorites: &[String]) -> Vec<EmojiEntry> {
        let trimmed = query.trim().to_lowercase();
        let freqs = self.frequencies.lock().unwrap();

        let mut matches: Vec<(&EmojiEntry, i32)> = Vec::new();

        for entry in &self.emojis {
            let mut score = 0;

            if favorites.iter().any(|f| f == &entry.ch) {
                score += 1000;
            }

            if let Some(freq) = freqs.get(&entry.ch) {
                score += (*freq as i32) * 10;
            }

            if trimmed.is_empty() {
                matches.push((entry, score));
                continue;
            }

            let name_lower = entry.name.to_lowercase();
            if name_lower.starts_with(&trimmed) {
                score += 500;
            } else if name_lower.contains(&trimmed) {
                score += 200;
            } else if entry.keywords.iter().any(|k| k.to_lowercase().contains(&trimmed)) {
                score += 100;
            } else {
                continue;
            }

            matches.push((entry, score));
        }

        matches.sort_by(|a, b| b.1.cmp(&a.1));
        matches.into_iter().take(limit).map(|(e, _)| e.clone()).collect()
    }

    /// Record usage of an emoji to update frequency ranking
    pub fn record_usage(&self, ch: &str) {
        if let Ok(mut freqs) = self.frequencies.lock() {
            *freqs.entry(ch.to_string()).or_insert(0) += 1;
        }
        self.save_frequencies();
    }

    fn load_frequencies(&mut self) {
        if self.freq_path.exists() {
            if let Ok(data) = fs::read_to_string(&self.freq_path) {
                if let Ok(parsed) = serde_json::from_str(&data) {
                    self.frequencies = Mutex::new(parsed);
                }
            }
        }
    }

    fn save_frequencies(&self) {
        if let Some(parent) = self.freq_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(freqs) = self.frequencies.lock() {
            if let Ok(json) = serde_json::to_string(&*freqs) {
                let _ = fs::write(&self.freq_path, json);
            }
        }
    }

    fn seed_emojis() -> Vec<EmojiEntry> {
        vec![
            EmojiEntry {
                ch: "😀".to_string(),
                name: "Grinning Face".to_string(),
                category: "Smileys".to_string(),
                keywords: vec!["smile".to_string(), "happy".to_string(), "joy".to_string()],
            },
            EmojiEntry {
                ch: "😂".to_string(),
                name: "Face with Tears of Joy".to_string(),
                category: "Smileys".to_string(),
                keywords: vec!["laugh".to_string(), "crying".to_string(), "lol".to_string()],
            },
            EmojiEntry {
                ch: "😍".to_string(),
                name: "Smiling Face with Heart-Eyes".to_string(),
                category: "Smileys".to_string(),
                keywords: vec!["love".to_string(), "crush".to_string(), "heart".to_string()],
            },
            EmojiEntry {
                ch: "🔥".to_string(),
                name: "Fire".to_string(),
                category: "Symbols".to_string(),
                keywords: vec!["flame".to_string(), "hot".to_string(), "lit".to_string()],
            },
            EmojiEntry {
                ch: "🎉".to_string(),
                name: "Party Popper".to_string(),
                category: "Objects".to_string(),
                keywords: vec!["celebration".to_string(), "party".to_string(), "congrats".to_string()],
            },
            EmojiEntry {
                ch: "👍".to_string(),
                name: "Thumbs Up".to_string(),
                category: "People".to_string(),
                keywords: vec!["approve".to_string(), "yes".to_string(), "ok".to_string()],
            },
            EmojiEntry {
                ch: "✨".to_string(),
                name: "Sparkles".to_string(),
                category: "Symbols".to_string(),
                keywords: vec!["shiny".to_string(), "magic".to_string(), "clean".to_string()],
            },
            EmojiEntry {
                ch: "❤️".to_string(),
                name: "Red Heart".to_string(),
                category: "Symbols".to_string(),
                keywords: vec!["love".to_string(), "like".to_string()],
            },
            EmojiEntry {
                ch: "🦖".to_string(),
                name: "T-Rex Dinosaur".to_string(),
                category: "Animals".to_string(),
                keywords: vec!["dino".to_string(), "jurassic".to_string(), "mascot".to_string()],
            },
            EmojiEntry {
                ch: "🚀".to_string(),
                name: "Rocket".to_string(),
                category: "Travel".to_string(),
                keywords: vec!["space".to_string(), "launch".to_string(), "fast".to_string()],
            },
        ]
    }
}
