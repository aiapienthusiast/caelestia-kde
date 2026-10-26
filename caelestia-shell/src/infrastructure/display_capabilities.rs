//! Display & Panel Capabilities Detection
//! 100% native Rust parity with `shell/scripts/panel_capabilities.py`.
//! Directly parses raw EDID binary structures from `/sys/class/drm/*/edid`
//! to verify genuine hardware HDR (SMPTE ST 2084 / HLG) capabilities without external dependencies.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const EDID_HEADER: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];
const CTA_EXTENSION: u8 = 0x02;
const EXTENDED_TAG: u8 = 0x07;
const HDR_STATIC_METADATA: u8 = 0x06;
const HDR_TRANSFER_FUNCTIONS: u8 = 0x0C; // Bit 2: PQ (ST 2084), Bit 3: HLG

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelCapability {
    pub output_name: String,
    pub hdr_supported: bool,
    pub has_valid_edid: bool,
}

pub struct PanelCapabilitiesDetector;

impl PanelCapabilitiesDetector {
    /// Parses EDID bytes and detects if HDR (PQ / HLG) transfer functions are advertised
    pub fn parse_edid(data: &[u8]) -> bool {
        if data.len() < 128 || &data[..8] != EDID_HEADER {
            return false;
        }

        let extension_count = data[126] as usize;
        for i in 1..=extension_count {
            let offset = i * 128;
            if offset + 128 <= data.len() {
                let block = &data[offset..offset + 128];
                if block[0] == CTA_EXTENSION {
                    if Self::check_cta_hdr(block) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Checks CTA-861 data blocks for HDR static metadata
    fn check_cta_hdr(block: &[u8]) -> bool {
        if block.len() < 4 {
            return false;
        }

        let end = (block[2] as usize).min(block.len());
        let mut pos = 4;

        while pos < end {
            let header = block[pos];
            let tag = (header >> 5) & 0x07;
            let len = (header & 0x1F) as usize;
            let next_pos = pos + 1 + len;

            if next_pos <= end {
                let payload = &block[pos + 1..next_pos];
                if tag == EXTENDED_TAG && !payload.is_empty() && payload[0] == HDR_STATIC_METADATA {
                    // Check transfer functions in payload[1]
                    if payload.len() > 1 && (payload[1] & HDR_TRANSFER_FUNCTIONS) != 0 {
                        return true;
                    }
                }
            }
            pos = next_pos;
        }

        false
    }

    /// Parses an EDID from a file
    pub fn check_file(path: &Path) -> PanelCapability {
        let output_name = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .map(|name| {
                if name.starts_with("card") {
                    name.split_once('-').map(|x| x.1).unwrap_or(name)
                } else {
                    name
                }
            })
            .unwrap_or("Unknown")
            .to_string();

        if let Ok(bytes) = fs::read(path) {
            let has_valid_edid = bytes.len() >= 128 && &bytes[..8] == EDID_HEADER;
            let hdr_supported = Self::parse_edid(&bytes);
            PanelCapability {
                output_name,
                hdr_supported,
                has_valid_edid,
            }
        } else {
            PanelCapability {
                output_name,
                hdr_supported: false,
                has_valid_edid: false,
            }
        }
    }

    /// Discovers capabilities across all connectors in /sys/class/drm/
    pub fn scan_drm_connectors() -> Vec<PanelCapability> {
        let mut results = Vec::new();
        let drm_path = Path::new("/sys/class/drm");
        if let Ok(entries) = fs::read_dir(drm_path) {
            for entry in entries.flatten() {
                let edid_path = entry.path().join("edid");
                if edid_path.exists() {
                    results.push(Self::check_file(&edid_path));
                }
            }
        }
        results
    }
}
