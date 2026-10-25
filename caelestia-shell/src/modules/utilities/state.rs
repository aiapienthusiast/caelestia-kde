//! Utilities State and Coordinator
//! Complete parity with `shell/modules/utilities/Wrapper.qml` and `Content.qml`.

use super::capture::CaptureCardModel;
use super::config::UtilitiesConfig;
use super::idle_inhibit::IdleInhibitor;
use super::quick_share::QuickShareModel;
use super::toggles::QuickTogglesModel;

/// Master state coordinator for the Utilities overlay drawer
pub struct UtilitiesState {
    pub is_open: bool,
    pub idle_inhibit: IdleInhibitor,
    pub capture: CaptureCardModel,
    pub quick_share: QuickShareModel,
    pub toggles: QuickTogglesModel,
    pub sidebar_attached: bool,
}

impl UtilitiesState {
    pub fn new(config: &UtilitiesConfig) -> Self {
        Self {
            is_open: false,
            idle_inhibit: IdleInhibitor::new(),
            capture: CaptureCardModel::new(),
            quick_share: QuickShareModel::new(),
            toggles: QuickTogglesModel::new(config.quick_toggles.clone(), config.quick_toggles_per_page),
            sidebar_attached: false,
        }
    }

    /// Open utilities drawer
    pub fn open(&mut self) {
        self.is_open = true;
    }

    /// Close utilities drawer
    pub fn close(&mut self) {
        self.is_open = false;
        self.capture.pending_delete_path = None;
        self.quick_share.close_device_selector();
        self.quick_share.pending_delete_transfer_id = None;
    }

    /// Toggle utilities drawer visibility
    pub fn toggle(&mut self) {
        if self.is_open {
            self.close();
        } else {
            self.open();
        }
    }
}
