//! Synced & Plain Lyrics Service
//! 100% parity with C++ `Caelestia::Lyrics`, `LyricCandidate`, and `LyricLine`.

use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LyricLine {
    pub time_secs: f64,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LyricCandidate {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration_secs: f64,
    pub is_synced: bool,
    pub source: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LyricsBackend {
    LrcLib,
    NetEase,
    LocalFile,
}

pub struct LyricsService {
    lines: Mutex<Vec<LyricLine>>,
    candidates: Mutex<Vec<LyricCandidate>>,
    track_title: Mutex<String>,
    track_artist: Mutex<String>,
    offset_secs: Mutex<f64>,
    loading: Mutex<bool>,
}

impl Default for LyricsService {
    fn default() -> Self {
        Self::new()
    }
}

impl LyricsService {
    pub fn new() -> Self {
        Self {
            lines: Mutex::new(Vec::new()),
            candidates: Mutex::new(Vec::new()),
            track_title: Mutex::new(String::new()),
            track_artist: Mutex::new(String::new()),
            offset_secs: Mutex::new(0.0),
            loading: Mutex::new(false),
        }
    }

    pub fn lines(&self) -> Vec<LyricLine> {
        self.lines.lock().map(|l| l.clone()).unwrap_or_default()
    }

    pub fn track_title(&self) -> String {
        self.track_title.lock().map(|t| t.clone()).unwrap_or_default()
    }

    pub fn track_artist(&self) -> String {
        self.track_artist.lock().map(|a| a.clone()).unwrap_or_default()
    }

    pub fn offset(&self) -> f64 {
        self.offset_secs.lock().map(|o| *o).unwrap_or(0.0)
    }

    pub fn set_offset(&self, offset_secs: f64) {
        if let Ok(mut o) = self.offset_secs.lock() {
            *o = offset_secs;
        }
    }

    pub fn is_loading(&self) -> bool {
        self.loading.lock().map(|l| *l).unwrap_or(false)
    }

    pub fn set_loading(&self, loading: bool) {
        if let Ok(mut l) = self.loading.lock() {
            *l = loading;
        }
    }

    pub fn set_lines(&self, lines: Vec<LyricLine>) {
        if let Ok(mut l) = self.lines.lock() {
            *l = lines;
        }
    }

    pub fn has_lyrics(&self) -> bool {
        self.lines.lock().map(|l| !l.is_empty()).unwrap_or(false)
    }

    /// Finds index of active lyric line at given playback timestamp
    pub fn index_for_time(&self, time_secs: f64) -> usize {
        let offset = self.offset();
        let effective_time = (time_secs + offset).max(0.0);

        if let Ok(lines) = self.lines.lock() {
            if lines.is_empty() {
                return 0;
            }
            let mut active_idx = 0;
            for (idx, line) in lines.iter().enumerate() {
                if line.time_secs <= effective_time {
                    active_idx = idx;
                } else {
                    break;
                }
            }
            active_idx
        } else {
            0
        }
    }

    /// Timestamp for given line index
    pub fn time_for_index(&self, index: usize) -> f64 {
        if let Ok(lines) = self.lines.lock() {
            lines.get(index).map(|l| l.time_secs).unwrap_or(0.0)
        } else {
            0.0
        }
    }

    /// Parses standard LRC synchronized string format into sorted `LyricLine` vector
    pub fn parse_lrc(lrc_content: &str) -> Vec<LyricLine> {
        let mut lines = Vec::new();

        for line in lrc_content.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with('[') {
                continue;
            }

            // Find all timestamps in line (e.g. [01:23.45][02:34.56] Lyric)
            let mut text_start = 0;
            let mut timestamps = Vec::new();

            let mut cursor = 0;
            while let Some(start_bracket) = trimmed[cursor..].find('[') {
                let abs_start = cursor + start_bracket;
                if let Some(end_bracket) = trimmed[abs_start..].find(']') {
                    let abs_end = abs_start + end_bracket;
                    let tag = &trimmed[abs_start + 1..abs_end];
                    if let Some(secs) = Self::parse_timestamp_tag(tag) {
                        timestamps.push(secs);
                        cursor = abs_end + 1;
                        text_start = cursor;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            let text = trimmed[text_start..].trim().to_string();
            for time_secs in timestamps {
                lines.push(LyricLine {
                    time_secs,
                    text: text.clone(),
                });
            }
        }

        // Sort chronologically
        lines.sort_by(|a, b| a.time_secs.partial_cmp(&b.time_secs).unwrap_or(std::cmp::Ordering::Equal));
        lines
    }

    fn parse_timestamp_tag(tag: &str) -> Option<f64> {
        let parts: Vec<&str> = tag.split(':').collect();
        if parts.len() == 2 {
            let minutes: f64 = parts[0].parse().ok()?;
            let seconds: f64 = parts[1].parse().ok()?;
            Some(minutes * 60.0 + seconds)
        } else {
            None
        }
    }

    /// Load lyrics directly from LRC text
    pub fn load_lrc_string(&self, title: &str, artist: &str, lrc: &str) {
        let parsed = Self::parse_lrc(lrc);
        if let Ok(mut lines) = self.lines.lock() {
            *lines = parsed;
        }
        if let Ok(mut t) = self.track_title.lock() {
            *t = title.to_string();
        }
        if let Ok(mut a) = self.track_artist.lock() {
            *a = artist.to_string();
        }
    }

    /// Clear currently loaded track & lyrics
    pub fn clear(&self) {
        if let Ok(mut l) = self.lines.lock() {
            l.clear();
        }
        if let Ok(mut c) = self.candidates.lock() {
            c.clear();
        }
        if let Ok(mut t) = self.track_title.lock() {
            t.clear();
        }
        if let Ok(mut a) = self.track_artist.lock() {
            a.clear();
        }
    }
}
