//! KWin Compositor Effects & Scripts Deployment Manager
//! 100% native Rust parity with `shell/kwin-effects/` deployment scripts.
//! Manages installation, state queries, and D-Bus reloading of custom KWin effects
//! (`caelestia-blur`, `magic-lamp`) in `~/.local/share/kwin/effects/`.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KWinEffectInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub is_installed: bool,
    pub is_enabled: bool,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct KWinEffectsManager;

impl KWinEffectsManager {
    pub fn new() -> Self {
        Self
    }

    /// Returns the target directory for user KWin effects: ~/.local/share/kwin/effects/
    pub fn effects_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".local/share/kwin/effects")
    }

    /// Inspects an effect directory and extracts metadata.json
    pub fn inspect_effect_dir(path: &Path) -> Option<KWinEffectInfo> {
        let metadata_path = path.join("metadata.json");
        let content = fs::read_to_string(metadata_path).ok()?;
        let val: serde_json::Value = serde_json::from_str(&content).ok()?;

        let kplugin = val.get("KPlugin");
        let id = kplugin
            .and_then(|k| k.get("Id"))
            .and_then(|i| i.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                path.file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            });
        let name = kplugin
            .and_then(|k| k.get("Name"))
            .and_then(|n| n.as_str())
            .unwrap_or(&id)
            .to_string();
        let description = kplugin
            .and_then(|k| k.get("Description"))
            .and_then(|d| d.as_str())
            .unwrap_or("")
            .to_string();
        let author = kplugin
            .and_then(|k| k.get("Authors"))
            .and_then(|a| a.get(0))
            .and_then(|a| a.get("Name"))
            .and_then(|n| n.as_str())
            .unwrap_or("Caelestia")
            .to_string();

        let is_installed = path.starts_with(Self::effects_dir());
        let is_enabled = Self::is_effect_enabled(&id);

        Some(KWinEffectInfo {
            id,
            name,
            description,
            author,
            is_installed,
            is_enabled,
            path: path.to_path_buf(),
        })
    }

    /// Queries `kreadconfig6` to check if an effect is active in `kwinrc`
    pub fn is_effect_enabled(effect_id: &str) -> bool {
        let output = Command::new("kreadconfig6")
            .args([
                "--file",
                "kwinrc",
                "--group",
                "Plugins",
                "--key",
                &format!("{}Enabled", effect_id),
            ])
            .output();

        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_lowercase();
            s == "true"
        } else {
            false
        }
    }

    /// Sets effect enabled state in kwinrc via `kwriteconfig6` and notifies KWin via D-Bus
    pub fn set_effect_enabled(effect_id: &str, enabled: bool) -> std::io::Result<bool> {
        let _ = Command::new("kwriteconfig6")
            .args([
                "--file",
                "kwinrc",
                "--group",
                "Plugins",
                "--key",
                &format!("{}Enabled", effect_id),
                if enabled { "true" } else { "false" },
            ])
            .status();

        Self::reconfigure_kwin();
        Ok(true)
    }

    /// Triggers KWin compositor effect reconfiguration via D-Bus
    pub fn reconfigure_kwin() {
        let _ = Command::new("qdbus6")
            .args([
                "org.kde.KWin",
                "/KWin",
                "org.kde.KWin.reconfigure",
            ])
            .status();
    }

    /// Installs a bundled effect directory into ~/.local/share/kwin/effects/
    pub fn install_effect(src_dir: &Path) -> std::io::Result<PathBuf> {
        let id = src_dir
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid dir name"))?;

        let dest = Self::effects_dir().join(id);
        fs::create_dir_all(&dest)?;
        copy_dir_recursive(src_dir, &dest)?;
        Ok(dest)
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
