//! Bar Component Widgets
//! 1:1 Complete Parity with `shell/modules/bar/components/`.

pub mod clock;
pub mod dock;
pub mod github;
pub mod github_store;
pub mod greeter;
pub mod kb_layout;
pub mod keep_awake;
pub mod media;
pub mod notes;
pub mod notifications;
pub mod os_icon;
pub mod performance;
pub mod performance_widget;
pub mod power;
pub mod screen_capture;
pub mod shortcuts;
pub mod show_desktop;
pub mod status;
pub mod status_icons;
pub mod tray;
pub mod tray_item;
pub mod update_indicator;
pub mod weather;
pub mod workspaces;

pub use clock::{BarClockModel, ClockSnapshot, ClockWidget};
pub use dock::{BarDockModel, DockSnapshot, DockWidget};
pub use github::GithubActivityWidget;
pub use github_store::{GithubStore, GithubStoreSnapshot};
pub use greeter::GreeterWidget;
pub use kb_layout::KbLayoutIndicatorWidget;
pub use keep_awake::KeepAwakeWidget;
pub use media::MediaWidget;
pub use notes::NotesWidget;
pub use notifications::NotificationsIndicatorWidget;
pub use os_icon::OsIconWidget;
pub use performance_widget::PerformanceWidget;
pub use power::PowerWidget;
pub use screen_capture::ScreenCaptureWidget;
pub use shortcuts::ShortcutsWidget;
pub use show_desktop::ShowDesktopWidget;
pub use status_icons::{BarStatusSnapshot, StatusIconsModel, StatusIconsWidget};
pub use tray::TrayWidget;
pub use tray_item::TrayItemWidget;
pub use update_indicator::UpdateIndicatorWidget;
pub use weather::WeatherWidget;
pub use workspaces::{WorkspacePill, WorkspacesModel, WorkspacesStripWidget};

