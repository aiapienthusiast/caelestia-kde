//! Media and Entertainment API Bridge
//! Parity with `shell/services/api/MediaApi.qml`.

use crate::services::audio::{AudioService, AudioState};
use crate::services::discord_rpc::{DiscordActivity, DiscordRpcService};
use crate::services::lyrics::{LyricLine, LyricsService};
use crate::services::mpris::{MprisService, TrackMetadata};
use std::sync::{Arc, Mutex, OnceLock};

/// Media API exposing audio volume, MPRIS player states, lyrics, and Discord RPC
#[derive(Clone)]
pub struct MediaApi {
    pub audio: Arc<Mutex<AudioService>>,
    pub mpris: Arc<Mutex<MprisService>>,
    pub lyrics: Arc<Mutex<LyricsService>>,
    pub discord: Arc<Mutex<DiscordRpcService>>,
}

impl Default for MediaApi {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaApi {
    pub fn new() -> Self {
        Self {
            audio: Arc::new(Mutex::new(AudioService::new())),
            mpris: Arc::new(Mutex::new(MprisService::new())),
            lyrics: Arc::new(Mutex::new(LyricsService::new())),
            discord: Arc::new(Mutex::new(DiscordRpcService::default())),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<MediaApi> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Read master audio output volume (0.0 to 1.0)
    pub fn volume(&self) -> f32 {
        self.audio.lock().map_or(1.0, |a| a.state.sink_volume)
    }

    /// Set master audio output volume
    pub fn set_volume(&self, volume: f32) {
        if let Ok(mut a) = self.audio.lock() {
            a.set_volume(volume);
        }
    }

    /// Toggle master audio output mute
    pub fn toggle_mute(&self) {
        if let Ok(mut a) = self.audio.lock() {
            a.toggle_mute();
        }
    }

    /// Audio output and input devices
    pub fn audio_state(&self) -> AudioState {
        self.audio.lock().map_or_else(
            |_| AudioState::default(),
            |a| a.state.clone(),
        )
    }

    /// Active track metadata
    pub fn current_track(&self) -> TrackMetadata {
        self.mpris.lock().map_or_else(
            |_| TrackMetadata::default(),
            |m| m.current_track.clone(),
        )
    }

    /// Send playback command to active MPRIS player
    pub fn play_pause(&self) {
        MprisService::play_pause();
    }

    /// Next track command
    pub fn next(&self) {
        MprisService::next();
    }

    /// Previous track command
    pub fn previous(&self) {
        MprisService::previous();
    }

    /// Synchronized lyric lines for active track
    pub fn lyrics(&self) -> Vec<LyricLine> {
        self.lyrics.lock().map_or(Vec::new(), |l| l.lines())
    }

    /// Update Discord Rich Presence activity
    pub fn update_discord(&self, details: &str, state: &str) {
        if let Ok(d) = self.discord.lock() {
            let activity = DiscordActivity {
                details: Some(details.to_string()),
                state: Some(state.to_string()),
                ..Default::default()
            };
            let _ = d.send_activity(&activity);
        }
    }
}
