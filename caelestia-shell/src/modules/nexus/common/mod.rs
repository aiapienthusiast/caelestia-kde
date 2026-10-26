//! Nexus Common Reusable Controls, Sub-dialogs and Detail Sections.
//! Parity with `shell/modules/nexus/common/`.

pub mod app_picker;
pub mod audio_devices;
pub mod dialogs;
pub mod key_capture;
pub mod network_detail;

pub use app_picker::{AppPickerRow, WindowPickerRow};
pub use audio_devices::AudioDeviceList;
pub use dialogs::{ListEditor, UninstallDialog, UpdateEntry, UpdateTimeline};
pub use key_capture::{KeyCaptureDialog, ShortcutRow};
pub use network_detail::{EthernetSection, Ipv4ConfigSection, NetworkList};
