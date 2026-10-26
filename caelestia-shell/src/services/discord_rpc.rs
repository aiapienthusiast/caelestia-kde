//! Discord Rich Presence IPC Client Service
//! 100% parity with C++ `Caelestia::DiscordIpc` and `shell/services/DiscordRPC.qml`.

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub const DISCORD_DEFAULT_CLIENT_ID: &'static str = "1324831818295574578";

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscordActivity {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
}

pub struct DiscordRpcService {
    client_id: Mutex<String>,
    stream: Mutex<Option<UnixStream>>,
    connected: Mutex<bool>,
}

impl Default for DiscordRpcService {
    fn default() -> Self {
        Self::new(DISCORD_DEFAULT_CLIENT_ID)
    }
}

impl DiscordRpcService {
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: Mutex::new(client_id.to_string()),
            stream: Mutex::new(None),
            connected: Mutex::new(false),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected.lock().map(|c| *c).unwrap_or(false)
    }

    /// Attempts connection to Discord IPC socket across discord-ipc-0 .. discord-ipc-9
    pub fn connect(&self) -> Result<(), String> {
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());

        for i in 0..10 {
            let socket_path = PathBuf::from(&runtime_dir).join(format!("discord-ipc-{}", i));
            if socket_path.exists() {
                if let Ok(mut stream) = UnixStream::connect(&socket_path) {
                    // Send Handshake frame (opcode 0)
                    let client_id = self.client_id.lock().unwrap().clone();
                    let handshake = json!({
                        "v": 1,
                        "client_id": client_id
                    });

                    if Self::send_frame(&mut stream, 0, &handshake).is_ok() {
                        // Read response
                        let mut header = [0u8; 8];
                        if stream.read_exact(&mut header).is_ok() {
                            let len = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as usize;
                            let mut buf = vec![0u8; len];
                            let _ = stream.read_exact(&mut buf);

                            if let Ok(mut s) = self.stream.lock() {
                                *s = Some(stream);
                            }
                            if let Ok(mut c) = self.connected.lock() {
                                *c = true;
                            }
                            return Ok(());
                        }
                    }
                }
            }
        }

        Err("No active Discord IPC socket found".to_string())
    }

    /// Disconnects from Discord IPC socket
    pub fn disconnect(&self) {
        if let Ok(mut s) = self.stream.lock() {
            *s = None;
        }
        if let Ok(mut c) = self.connected.lock() {
            *c = false;
        }
    }

    /// Updates Discord Rich Presence activity
    pub fn send_activity(&self, activity: &DiscordActivity) -> Result<(), String> {
        let mut stream_guard = self.stream.lock().map_err(|e| e.to_string())?;
        if let Some(ref mut stream) = *stream_guard {
            let mut timestamps = json!({});
            if let Some(start) = activity.start_timestamp {
                timestamps["start"] = json!(start);
            }
            if let Some(end) = activity.end_timestamp {
                timestamps["end"] = json!(end);
            }

            let mut assets = json!({});
            if let Some(ref img) = activity.large_image {
                assets["large_image"] = json!(img);
            }
            if let Some(ref txt) = activity.large_text {
                assets["large_text"] = json!(txt);
            }
            if let Some(ref img) = activity.small_image {
                assets["small_image"] = json!(img);
            }
            if let Some(ref txt) = activity.small_text {
                assets["small_text"] = json!(txt);
            }

            let mut act = json!({
                "assets": assets,
            });
            if let Some(ref d) = activity.details {
                act["details"] = json!(d);
            }
            if let Some(ref s) = activity.state {
                act["state"] = json!(s);
            }
            if activity.start_timestamp.is_some() || activity.end_timestamp.is_some() {
                act["timestamps"] = timestamps;
            }

            let payload = json!({
                "cmd": "SET_ACTIVITY",
                "args": {
                    "pid": std::process::id(),
                    "activity": act
                },
                "nonce": format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis())
            });

            Self::send_frame(stream, 1, &payload).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Not connected to Discord".to_string())
        }
    }

    /// Clears active Rich Presence
    pub fn clear_activity(&self) -> Result<(), String> {
        let empty = DiscordActivity::default();
        self.send_activity(&empty)
    }

    fn send_frame(stream: &mut UnixStream, opcode: u32, payload: &serde_json::Value) -> std::io::Result<()> {
        let json_bytes = serde_json::to_vec(payload)?;
        let len = json_bytes.len() as u32;

        let mut header = [0u8; 8];
        header[0..4].copy_from_slice(&opcode.to_le_bytes());
        header[4..8].copy_from_slice(&len.to_le_bytes());

        stream.write_all(&header)?;
        stream.write_all(&json_bytes)?;
        stream.flush()
    }
}
