//! QuickShare Subsystem (Local Device Discovery & Sharing)
//! Complete parity with `shell/modules/utilities/cards/QuickShareList.qml`,
//! `QuickShareDeviceSelector.qml`, and `QuickShareDeleteModal.qml`.

use serde::{Deserialize, Serialize};
use tracing::info;

/// Discovered nearby target device
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NearbyDevice {
    pub id: String,
    pub name: String,
    pub device_type: String, // "phone", "tablet", "laptop", "pc"
}

/// Active or completed QuickShare transfer task
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuickShareTransfer {
    pub id: String,
    pub filename: String,
    pub device_name: String,
    pub progress: f32, // 0.0 .. 1.0
    pub completed: bool,
    pub failed: bool,
}

/// QuickShare model managing discovery, transfer queue, and dialogs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuickShareModel {
    pub is_enabled: bool,
    pub device_name: String,
    pub devices: Vec<NearbyDevice>,
    pub transfers: Vec<QuickShareTransfer>,
    pub is_selector_open: bool,
    pub selected_device_id: Option<String>,
    pub pending_delete_transfer_id: Option<String>,
}

impl Default for QuickShareModel {
    fn default() -> Self {
        Self {
            is_enabled: true,
            device_name: "Caelestia Desktop".to_string(),
            devices: Vec::new(),
            transfers: Vec::new(),
            is_selector_open: false,
            selected_device_id: None,
            pending_delete_transfer_id: None,
        }
    }
}

impl QuickShareModel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Open target device selector modal
    pub fn open_device_selector(&mut self) {
        self.is_selector_open = true;
    }

    /// Close device selector modal
    pub fn close_device_selector(&mut self) {
        self.is_selector_open = false;
        self.selected_device_id = None;
    }

    /// Select nearby target device
    pub fn select_device(&mut self, device_id: &str) {
        self.selected_device_id = Some(device_id.to_string());
    }

    /// Send a file to target device
    pub fn send_file(&mut self, device_id: &str, filename: &str) -> Option<QuickShareTransfer> {
        let dev = self.devices.iter().find(|d| d.id == device_id)?;
        let id = format!("tx-{}", self.transfers.len() + 1);

        info!("Sending file {} to nearby device {}", filename, dev.name);

        let tx = QuickShareTransfer {
            id: id.clone(),
            filename: filename.to_string(),
            device_name: dev.name.clone(),
            progress: 0.1,
            completed: false,
            failed: false,
        };

        self.transfers.insert(0, tx.clone());
        self.is_selector_open = false;
        Some(tx)
    }

    /// Request confirmation to delete or cancel a transfer
    pub fn request_delete_transfer(&mut self, transfer_id: &str) {
        self.pending_delete_transfer_id = Some(transfer_id.to_string());
    }

    /// Confirm transfer deletion or abort
    pub fn confirm_delete_transfer(&mut self) -> Option<String> {
        let id = self.pending_delete_transfer_id.take()?;
        self.transfers.retain(|t| t.id != id);
        Some(id)
    }

    /// Cancel delete modal
    pub fn cancel_delete_modal(&mut self) {
        self.pending_delete_transfer_id = None;
    }
}
