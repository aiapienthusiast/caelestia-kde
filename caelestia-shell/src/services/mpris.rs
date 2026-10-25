//! MPRIS Media Player Service
//! Parity with `shell/services/Players.qml` and `lyrics.cpp`.
//! Tracks active MPRIS media players, metadata, art, and playback states.

use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub art_url: String,
    pub status: PlaybackStatus,
    pub position_secs: f64,
    pub duration_secs: f64,
}

impl Default for TrackMetadata {
    fn default() -> Self {
        Self {
            title: "No Media Playing".to_string(),
            artist: "Caelestia Shell".to_string(),
            album: "".to_string(),
            art_url: "".to_string(),
            status: PlaybackStatus::Stopped,
            position_secs: 0.0,
            duration_secs: 0.0,
        }
    }
}

pub struct MprisService {
    pub current_track: TrackMetadata,
}

impl Default for MprisService {
    fn default() -> Self {
        Self::new()
    }
}

impl MprisService {
    pub fn new() -> Self {
        let mut service = Self {
            current_track: TrackMetadata::default(),
        };
        service.poll_playerctl();
        service
    }

    /// Read active player metadata using playerctl CLI if available
    pub fn poll_playerctl(&mut self) {
        let title_out = Command::new("playerctl")
            .args(["metadata", "xesam:title"])
            .output();
        let artist_out = Command::new("playerctl")
            .args(["metadata", "xesam:artist"])
            .output();
        let status_out = Command::new("playerctl").args(["status"]).output();
        let position_out = Command::new("playerctl").args(["position"]).output();

        if let Ok(title) = title_out {
            let t = String::from_utf8_lossy(&title.stdout).trim().to_string();
            if !t.is_empty() {
                self.current_track.title = t;
            }
        }

        if let Ok(artist) = artist_out {
            let a = String::from_utf8_lossy(&artist.stdout).trim().to_string();
            if !a.is_empty() {
                self.current_track.artist = a;
            }
        }

        if let Ok(status) = status_out {
            let s = String::from_utf8_lossy(&status.stdout).trim().to_lowercase();
            self.current_track.status = match s.as_str() {
                "playing" => PlaybackStatus::Playing,
                "paused" => PlaybackStatus::Paused,
                _ => PlaybackStatus::Stopped,
            };
        }

        if let Ok(pos) = position_out {
            let p_str = String::from_utf8_lossy(&pos.stdout).trim().to_string();
            if let Ok(p) = p_str.parse::<f64>() {
                self.current_track.position_secs = p;
            }
        }
    }

    pub fn play_pause() {
        let _ = Command::new("playerctl").args(["play-pause"]).spawn();
    }

    pub fn next() {
        let _ = Command::new("playerctl").args(["next"]).spawn();
    }

    pub fn previous() {
        let _ = Command::new("playerctl").args(["previous"]).spawn();
    }

    /// Progress ratio (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.current_track.duration_secs > 0.0 {
            (self.current_track.position_secs / self.current_track.duration_secs) as f32
        } else {
            0.0
        }
    }

    /// Formatted time (e.g. "1:45 / 3:12")
    pub fn formatted_time(&self) -> String {
        let pos = self.current_track.position_secs as u64;
        let dur = self.current_track.duration_secs as u64;
        format!(
            "{}:{:02} / {}:{:02}",
            pos / 60,
            pos % 60,
            dur / 60,
            dur % 60
        )
    }
}
