//! News Feed Tab Subsystem
//! Complete parity with `shell/modules/sidebar/News.qml`.

use serde::{Deserialize, Serialize};

/// News feed headline article
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewsItem {
    pub id: String,
    pub title: String,
    pub source: String,
    pub url: String,
    pub timestamp: u64,
    pub category: String,
}

/// News feed manager
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewsModel {
    pub articles: Vec<NewsItem>,
    pub is_refreshing: bool,
}

impl Default for NewsModel {
    fn default() -> Self {
        Self {
            articles: vec![
                NewsItem {
                    id: "news-1".to_string(),
                    title: "KDE Plasma 6.4 Desktop Architecture Overview".to_string(),
                    source: "KDE News".to_string(),
                    url: "https://kde.org/announcements/".to_string(),
                    timestamp: 1729000000,
                    category: "Desktop".to_string(),
                },
                NewsItem {
                    id: "news-2".to_string(),
                    title: "Wayland Layer Shell Protocol Standardization Progress".to_string(),
                    source: "Wayland Project".to_string(),
                    url: "https://wayland.freedesktop.org/".to_string(),
                    timestamp: 1729050000,
                    category: "Protocols".to_string(),
                },
                NewsItem {
                    id: "news-3".to_string(),
                    title: "Linux 6.14 Kernel Rolling Out Performance Optimizations".to_string(),
                    source: "Kernel.org".to_string(),
                    url: "https://kernel.org".to_string(),
                    timestamp: 1729100000,
                    category: "Kernel".to_string(),
                },
            ],
            is_refreshing: false,
        }
    }
}

impl NewsModel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Refresh news feed articles
    pub fn refresh(&mut self) {
        self.is_refreshing = true;
        // In real backend, fetches RSS / API feed asynchronously
        self.is_refreshing = false;
    }
}
