//! Krohnkite KWin Tiling Script Configuration & State Service
//! 100% parity with C++ `Caelestia::KrohnkiteConfig`.

use std::process::Command;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KrohnkiteGaps {
    pub between: i32,
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

impl Default for KrohnkiteGaps {
    fn default() -> Self {
        Self {
            between: 6,
            top: 6,
            bottom: 6,
            left: 6,
            right: 6,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KrohnkiteLayouts {
    pub binary_tree: bool,
    pub cascade: bool,
    pub columns: bool,
    pub floating: bool,
    pub monocle: bool,
    pub quarter: bool,
    pub spiral: bool,
    pub spread: bool,
    pub stacked: bool,
    pub stair: bool,
    pub three_column: bool,
    pub tile: bool,
}

impl Default for KrohnkiteLayouts {
    fn default() -> Self {
        Self {
            binary_tree: true,
            cascade: false,
            columns: true,
            floating: true,
            monocle: true,
            quarter: false,
            spiral: true,
            spread: true,
            stacked: false,
            stair: false,
            three_column: true,
            tile: true,
        }
    }
}

pub struct KrohnkiteService {
    pub gaps: Mutex<KrohnkiteGaps>,
    pub layouts: Mutex<KrohnkiteLayouts>,
    pub ignore_classes: Mutex<Vec<String>>,
}

impl Default for KrohnkiteService {
    fn default() -> Self {
        Self::new()
    }
}

impl KrohnkiteService {
    pub fn new() -> Self {
        let service = Self {
            gaps: Mutex::new(KrohnkiteGaps::default()),
            layouts: Mutex::new(KrohnkiteLayouts::default()),
            ignore_classes: Mutex::new(vec![
                "krunner".to_string(),
                "spectacle".to_string(),
                "caelestia-shell".to_string(),
            ]),
        };
        service.load_from_kwinrc();
        service
    }

    pub fn set_gaps(&self, gaps: KrohnkiteGaps) {
        if let Ok(mut g) = self.gaps.lock() {
            *g = gaps.clone();
        }
        self.save_key("screenGapBetween", &gaps.between.to_string());
        self.save_key("screenGapTop", &gaps.top.to_string());
        self.save_key("screenGapBottom", &gaps.bottom.to_string());
        self.save_key("screenGapLeft", &gaps.left.to_string());
        self.save_key("screenGapRight", &gaps.right.to_string());
        self.reconfigure();
    }

    pub fn set_layouts(&self, layouts: KrohnkiteLayouts) {
        if let Ok(mut l) = self.layouts.lock() {
            *l = layouts.clone();
        }
        self.save_key("binaryTreeLayoutEnabled", if layouts.binary_tree { "true" } else { "false" });
        self.save_key("floatingLayoutEnabled", if layouts.floating { "true" } else { "false" });
        self.save_key("monocleLayoutEnabled", if layouts.monocle { "true" } else { "false" });
        self.save_key("spiralLayoutEnabled", if layouts.spiral { "true" } else { "false" });
        self.save_key("tileLayoutEnabled", if layouts.tile { "true" } else { "false" });
        self.reconfigure();
    }

    fn save_key(&self, key: &str, value: &str) {
        let _ = Command::new("kwriteconfig6")
            .args(["--file", "kwinrc", "--group", "Script-krohnkite", "--key", key, value])
            .status();
    }

    fn load_from_kwinrc(&self) {
        if let Ok(out) = Command::new("kreadconfig6")
            .args(["--file", "kwinrc", "--group", "Script-krohnkite", "--key", "screenGapBetween", "--default", "6"])
            .output()
        {
            if let Ok(val) = String::from_utf8_lossy(&out.stdout).trim().parse::<i32>() {
                if let Ok(mut g) = self.gaps.lock() {
                    g.between = val;
                }
            }
        }
    }

    fn reconfigure(&self) {
        let _ = Command::new("qdbus6")
            .args(["org.kde.KWin", "/KWin", "reconfigure"])
            .status();
    }
}
