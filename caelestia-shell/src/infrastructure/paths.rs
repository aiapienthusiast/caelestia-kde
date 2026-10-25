//! Standardized paths for Caelestia KDE Shell
//! Parity with `shell/utils/Paths.qml`.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct CaelestiaPaths {
    pub home: PathBuf,
    pub config: PathBuf,
    pub data: PathBuf,
    pub state: PathBuf,
    pub cache: PathBuf,
    pub runtime_dir: PathBuf,
    pub pictures: PathBuf,
    pub videos: PathBuf,
    pub bin_dir: PathBuf,
}

impl Default for CaelestiaPaths {
    fn default() -> Self {
        Self::new()
    }
}

impl CaelestiaPaths {
    pub fn new() -> Self {
        let home = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/home/user"));

        let config = std::env::var("XDG_CONFIG_HOME")
            .map(|p| PathBuf::from(p).join("caelestia"))
            .unwrap_or_else(|_| home.join(".config/caelestia"));

        let data = std::env::var("XDG_DATA_HOME")
            .map(|p| PathBuf::from(p).join("caelestia"))
            .unwrap_or_else(|_| home.join(".local/share/caelestia"));

        let state = std::env::var("XDG_STATE_HOME")
            .map(|p| PathBuf::from(p).join("caelestia"))
            .unwrap_or_else(|_| home.join(".local/state/caelestia"));

        let cache = std::env::var("XDG_CACHE_HOME")
            .map(|p| PathBuf::from(p).join("caelestia"))
            .unwrap_or_else(|_| home.join(".cache/caelestia"));

        let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(format!("/tmp/caelestia-{}", user)));

        let pictures = std::env::var("XDG_PICTURES_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join("Pictures"));

        let videos = std::env::var("XDG_VIDEOS_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join("Videos"));

        let bin_dir = std::env::var("CAELESTIA_BIN_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join(".local/bin"));

        Self {
            home,
            config,
            data,
            state,
            cache,
            runtime_dir,
            pictures,
            videos,
            bin_dir,
        }
    }

    /// Path to scheme.json palette file
    pub fn scheme_path(&self) -> PathBuf {
        self.state.join("scheme.json")
    }

    /// Path to shell.json configuration file
    pub fn shell_config_path(&self) -> PathBuf {
        self.config.join("shell.json")
    }

    /// Path to wallpaper directory
    pub fn wallpapers_dir(&self) -> PathBuf {
        std::env::var("CAELESTIA_WALLPAPERS_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| self.pictures.join("Wallpapers"))
    }

    /// Path to user avatar icon (~/.face or ~/.face.icon)
    pub fn user_face_icon(&self) -> Option<PathBuf> {
        let face = self.home.join(".face");
        if face.exists() {
            return Some(face);
        }
        let face_icon = self.home.join(".face.icon");
        if face_icon.exists() {
            return Some(face_icon);
        }
        None
    }

    /// Shorten an absolute path replacing $HOME with '~'
    pub fn shorten_home<P: AsRef<Path>>(&self, path: P) -> String {
        let p = path.as_ref();
        if let Ok(rel) = p.strip_prefix(&self.home) {
            format!("~/{}", rel.display())
        } else {
            p.display().to_string()
        }
    }

    /// Expand '~' into an absolute path
    pub fn expand_home<S: AsRef<str>>(&self, path: S) -> PathBuf {
        let s = path.as_ref();
        if let Some(rest) = s.strip_prefix("~/") {
            self.home.join(rest)
        } else if s == "~" {
            self.home.clone()
        } else {
            PathBuf::from(s)
        }
    }
}
