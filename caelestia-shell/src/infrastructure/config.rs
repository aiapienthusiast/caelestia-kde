//! Configuration loader for Caelestia Shell settings (~/.config/caelestia/shell.json).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    #[serde(default = "default_transparency")]
    pub transparency: f32,
    #[serde(default = "default_blur")]
    pub blur: bool,
    #[serde(default = "default_position")]
    pub position: String,
}

fn default_transparency() -> f32 {
    0.70
}

fn default_blur() -> bool {
    true
}

fn default_position() -> String {
    "right".to_string()
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            transparency: default_transparency(),
            blur: default_blur(),
            position: default_position(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShellConfig {
    #[serde(default)]
    pub session: SessionConfig,
}

impl ShellConfig {
    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str::<ShellConfig>(&content) {
                return config;
            }
        }
        Self::default()
    }

    pub fn config_path() -> PathBuf {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/caelestia/shell.json")
        } else {
            PathBuf::from("/home/ladybug-me/.config/caelestia/shell.json")
        }
    }
}
