//! Audio Service
//! Parity with `shell/services/Audio.qml` and `audiodevicebackend.cpp`.
//! Manages PipeWire audio sinks, sources, volume levels, and mute states via wpctl.

use std::process::Command;

#[derive(Clone, Debug, PartialEq)]
pub struct AudioDevice {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub volume: f32,
    pub muted: bool,
    pub is_default: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioState {
    pub sink_volume: f32,
    pub sink_muted: bool,
    pub sink_name: String,
    pub source_volume: f32,
    pub source_muted: bool,
    pub source_name: String,
    pub sinks: Vec<AudioDevice>,
    pub sources: Vec<AudioDevice>,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            sink_volume: 1.0,
            sink_muted: false,
            sink_name: "Default Speaker".to_string(),
            source_volume: 1.0,
            source_muted: false,
            source_name: "Default Microphone".to_string(),
            sinks: Vec::new(),
            sources: Vec::new(),
        }
    }
}

pub struct AudioService {
    pub state: AudioState,
}

impl AudioService {
    pub fn new() -> Self {
        let mut service = Self {
            state: AudioState::default(),
        };
        if !crate::infrastructure::is_test_environment() {
            service.poll();
        }
        service
    }

    /// Refresh sink/source volume, mute states, and devices
    pub fn poll(&mut self) {
        if crate::infrastructure::is_test_environment() {
            return;
        }
        if let Ok(output) = Command::new("wpctl")
            .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                if let Some((vol, muted)) = Self::parse_volume_output(&text) {
                    self.state.sink_volume = vol;
                    self.state.sink_muted = muted;
                }
            }
        }

        if let Ok(output) = Command::new("wpctl")
            .args(["get-volume", "@DEFAULT_AUDIO_SOURCE@"])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                if let Some((vol, muted)) = Self::parse_volume_output(&text) {
                    self.state.source_volume = vol;
                    self.state.source_muted = muted;
                }
            }
        }

        if let Ok(output) = Command::new("wpctl").arg("status").output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                let (sinks, sources) = Self::parse_wpctl_status(&text);
                self.state.sinks = sinks;
                self.state.sources = sources;
            }
        }
    }

    /// Parse wpctl get-volume output, e.g. "Volume: 0.85" or "Volume: 1.00 [MUTED]"
    pub fn parse_volume_output(output: &str) -> Option<(f32, bool)> {
        let trimmed = output.trim();
        if !trimmed.starts_with("Volume:") {
            return None;
        }

        let parts: Vec<&str> = trimmed["Volume:".len()..].split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let vol = parts[0].parse::<f32>().ok()?;
        let muted = parts.iter().any(|&p| p == "[MUTED]");
        Some((vol, muted))
    }

    /// Parse wpctl status sinks and sources
    pub fn parse_wpctl_status(output: &str) -> (Vec<AudioDevice>, Vec<AudioDevice>) {
        let mut sinks = Vec::new();
        let mut sources = Vec::new();
        let mut in_sinks = false;
        let mut in_sources = false;

        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("├─ Sinks:") || trimmed.starts_with("Sinks:") {
                in_sinks = true;
                in_sources = false;
                continue;
            } else if trimmed.starts_with("├─ Sources:") || trimmed.starts_with("Sources:") {
                in_sinks = false;
                in_sources = true;
                continue;
            } else if trimmed.starts_with("├─ Filters:")
                || trimmed.starts_with("└─ Streams:")
                || trimmed.starts_with("Video")
                || trimmed.starts_with("Settings")
            {
                in_sinks = false;
                in_sources = false;
                continue;
            }

            if in_sinks || in_sources {
                if let Some(dev) = Self::parse_device_line(line) {
                    if in_sinks {
                        sinks.push(dev);
                    } else {
                        sources.push(dev);
                    }
                }
            }
        }

        (sinks, sources)
    }

    fn parse_device_line(line: &str) -> Option<AudioDevice> {
        let trimmed = line.trim_start_matches(|c| c == ' ' || c == '│' || c == '├' || c == '└' || c == '─');
        let trimmed = trimmed.trim();
        if trimmed.is_empty() || trimmed.starts_with('-') {
            return None;
        }

        let is_default = trimmed.starts_with('*');
        let rest = if is_default {
            trimmed.trim_start_matches('*').trim()
        } else {
            trimmed
        };

        // Format: "57. Alder Lake PCH-P High Definition Audio Controller Speaker [vol: 1.20]"
        let dot_pos = rest.find('.')?;
        let id = rest[..dot_pos].trim().parse::<u32>().ok()?;
        let after_id = rest[dot_pos + 1..].trim();

        let (desc, vol, muted) = if let Some(bracket_start) = after_id.find('[') {
            let desc = after_id[..bracket_start].trim().to_string();
            let bracket_content = &after_id[bracket_start..];
            let muted = bracket_content.contains("MUTED");
            let vol = if let Some(vol_pos) = bracket_content.find("vol:") {
                let num_str = bracket_content[vol_pos + 4..]
                    .trim_matches(|c: char| c == ' ' || c == ']')
                    .split_whitespace()
                    .next()
                    .unwrap_or("1.0");
                num_str.parse::<f32>().unwrap_or(1.0)
            } else {
                1.0
            };
            (desc, vol, muted)
        } else {
            (after_id.to_string(), 1.0, false)
        };

        Some(AudioDevice {
            id,
            name: desc.clone(),
            description: desc,
            volume: vol,
            muted,
            is_default,
        })
    }

    /// Returns command specification for setting audio volume via wpctl
    pub fn command_for_set_volume(target: &str, volume: f32) -> (&'static str, Vec<String>) {
        (
            "wpctl",
            vec![
                "set-volume".to_string(),
                target.to_string(),
                format!("{:.2}", volume.clamp(0.0, 1.5)),
            ],
        )
    }

    /// Returns command specification for toggling or setting mute via wpctl
    pub fn command_for_set_mute(target: &str, toggle: bool) -> (&'static str, Vec<String>) {
        (
            "wpctl",
            vec![
                "set-mute".to_string(),
                target.to_string(),
                if toggle { "toggle".to_string() } else { "1".to_string() },
            ],
        )
    }

    /// Returns command specification for setting default device via wpctl
    pub fn command_for_set_default(id: u32) -> (&'static str, Vec<String>) {
        (
            "wpctl",
            vec!["set-default".to_string(), id.to_string()],
        )
    }

    /// Set volume as ratio (0.0 to 1.5, clamped to 1.5 max)
    pub fn set_volume(&mut self, volume: f32) {
        let clamped = volume.clamp(0.0, 1.5);
        self.state.sink_volume = clamped;
        if !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_set_volume("@DEFAULT_AUDIO_SINK@", clamped);
            let _ = Command::new(prog).args(args).spawn();
        }
    }

    /// Step volume by delta, e.g. 0.05 (+5%) or -0.05 (-5%)
    pub fn step_volume(&mut self, delta: f32) {
        self.set_volume(self.state.sink_volume + delta);
    }

    /// Toggle mute for default sink
    pub fn toggle_mute(&mut self) {
        self.state.sink_muted = !self.state.sink_muted;
        if !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_set_mute("@DEFAULT_AUDIO_SINK@", true);
            let _ = Command::new(prog).args(args).spawn();
        }
    }

    /// Set source (mic) volume
    pub fn set_source_volume(&mut self, volume: f32) {
        let clamped = volume.clamp(0.0, 1.0);
        self.state.source_volume = clamped;
        if !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_set_volume("@DEFAULT_AUDIO_SOURCE@", clamped);
            let _ = Command::new(prog).args(args).spawn();
        }
    }

    /// Toggle mute for default microphone
    pub fn toggle_source_mute(&mut self) {
        self.state.source_muted = !self.state.source_muted;
        if !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_set_mute("@DEFAULT_AUDIO_SOURCE@", true);
            let _ = Command::new(prog).args(args).spawn();
        }
    }

    /// Set default sink by device ID
    pub fn set_default_sink(&mut self, id: u32) {
        if !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_set_default(id);
            let _ = Command::new(prog).args(args).spawn();
            self.poll();
        }
    }

    /// Set default source by device ID
    pub fn set_default_source(&mut self, id: u32) {
        if !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_set_default(id);
            let _ = Command::new(prog).args(args).spawn();
            self.poll();
        }
    }
}
