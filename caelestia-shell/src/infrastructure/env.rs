//! Environment detection and test mode isolation.
//! Ensures tests never execute destructive system calls (poweroff, reboot,
//! nmcli, wpctl, bluetoothctl, qdbus6, brightnessctl, gpu-screen-recorder).

use std::sync::atomic::{AtomicBool, Ordering};

static TEST_MODE_FORCE: AtomicBool = AtomicBool::new(false);

/// Explicitly force test/dry-run mode for mock executions
pub fn set_test_mode(enabled: bool) {
    TEST_MODE_FORCE.store(enabled, Ordering::SeqCst);
}

/// Returns true if the current execution is within a test runner or dry-run context
pub fn is_test_environment() -> bool {
    if TEST_MODE_FORCE.load(Ordering::Relaxed) {
        return true;
    }
    if cfg!(test) {
        return true;
    }
    if std::env::var("CARGO_MANIFEST_DIR").is_ok() {
        return true;
    }
    if std::env::var("CAELESTIA_TEST").is_ok() || std::env::var("CAELESTIA_DRY_RUN").is_ok() {
        return true;
    }
    if let Ok(exe) = std::env::current_exe() {
        let path = exe.to_string_lossy();
        if path.contains("/deps/") || path.contains("test") {
            return true;
        }
    }
    false
}
