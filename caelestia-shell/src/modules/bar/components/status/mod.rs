//! Bar Status Indicators
//! Parity with `shell/modules/bar/components/status/`.

pub mod battery;
pub mod bluetooth;
pub mod lock_status;

pub use battery::BatteryStatusWidget;
pub use bluetooth::BluetoothStatusWidget;
pub use lock_status::LockStatusWidget;
