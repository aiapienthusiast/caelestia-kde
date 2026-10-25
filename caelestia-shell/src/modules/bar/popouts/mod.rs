//! Bar Popouts Subsystem
//! Complete 1:1 parity with `shell/modules/bar/popouts/`.

pub mod audio;
pub mod battery;
pub mod bluetooth;
pub mod calendar;
pub mod dock_context;
pub mod github;
pub mod greeter;
pub mod kb_layout;
pub mod keep_awake;
pub mod lock_status;
pub mod media;
pub mod network;
pub mod night_light;
pub mod notes;
pub mod notifications;
pub mod performance;
pub mod peripheral_battery;
pub mod power;
pub mod privacy;
pub mod screen_capture;
pub mod shortcuts;
pub mod tray_menu;
pub mod updates;
pub mod weather;
pub mod wireless_password;
pub mod wrapper;

pub use audio::AudioPopout;
pub use battery::BatteryPopout;
pub use bluetooth::BluetoothPopout;
pub use calendar::CalendarPopout;
pub use dock_context::DockContextPopout;
pub use github::GithubPopout;
pub use greeter::GreeterPopout;
pub use kb_layout::KbLayoutPopout;
pub use keep_awake::KeepAwakePopout;
pub use lock_status::LockStatusPopout;
pub use media::MediaPopout;
pub use network::NetworkPopout;
pub use night_light::NightLightPopout;
pub use notes::NotesPopout;
pub use notifications::NotificationsPopout;
pub use performance::PerformancePopout;
pub use peripheral_battery::PeripheralBatteryPopout;
pub use power::PowerPopout;
pub use privacy::PrivacyPopout;
pub use screen_capture::ScreenCapturePopout;
pub use shortcuts::ShortcutsPopout;
pub use tray_menu::TrayMenuPopout;
pub use updates::UpdatesPopout;
pub use weather::WeatherPopout;
pub use wireless_password::WirelessPasswordPopout;
pub use wrapper::PopoutWrapper;

use crate::infrastructure::theme::CaelestiaColours;
use gpui::Div;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PopoutType {
    Audio,
    Battery,
    Bluetooth,
    Calendar,
    DockContext,
    Github,
    Greeter,
    KbLayout,
    KeepAwake,
    LockStatus,
    Media,
    Network,
    NightLight,
    Notes,
    Notifications,
    Performance,
    PeripheralBattery,
    Power,
    Privacy,
    ScreenCapture,
    Shortcuts,
    TrayMenu,
    Updates,
    Weather,
    WirelessPassword,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BarPopoutsState {
    pub current: Option<PopoutType>,
    pub current_center: f32,
    pub is_open: bool,
}

impl BarPopoutsState {
    pub fn open(&mut self, popout: PopoutType, center: f32) {
        self.current = Some(popout);
        self.current_center = center;
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.current = None;
        self.is_open = false;
    }

    pub fn toggle(&mut self, popout: PopoutType, center: f32) {
        if self.is_open && self.current == Some(popout) {
            self.close();
        } else {
            self.open(popout, center);
        }
    }
}

pub struct BarPopouts;

impl BarPopouts {
    /// Dispatches to the corresponding dedicated popout component.
    pub fn render(
        popout_type: PopoutType,
        center_x: f32,
        c: &CaelestiaColours,
    ) -> Div {
        match popout_type {
            PopoutType::Audio => AudioPopout::render(center_x, c),
            PopoutType::Battery => BatteryPopout::render(center_x, c),
            PopoutType::Bluetooth => BluetoothPopout::render(center_x, c),
            PopoutType::Calendar => CalendarPopout::render(center_x, c),
            PopoutType::DockContext => DockContextPopout::render(center_x, c),
            PopoutType::Github => GithubPopout::render(center_x, c),
            PopoutType::Greeter => GreeterPopout::render(center_x, c),
            PopoutType::KbLayout => KbLayoutPopout::render(center_x, c),
            PopoutType::KeepAwake => KeepAwakePopout::render(center_x, c),
            PopoutType::LockStatus => LockStatusPopout::render(center_x, c),
            PopoutType::Media => MediaPopout::render(center_x, c),
            PopoutType::Network => NetworkPopout::render(center_x, c),
            PopoutType::NightLight => NightLightPopout::render(center_x, c),
            PopoutType::Notes => NotesPopout::render(center_x, c),
            PopoutType::Notifications => NotificationsPopout::render(center_x, c),
            PopoutType::Performance => PerformancePopout::render(center_x, c),
            PopoutType::PeripheralBattery => PeripheralBatteryPopout::render(center_x, c),
            PopoutType::Power => PowerPopout::render(center_x, c),
            PopoutType::Privacy => PrivacyPopout::render(center_x, c),
            PopoutType::ScreenCapture => ScreenCapturePopout::render(center_x, c),
            PopoutType::Shortcuts => ShortcutsPopout::render(center_x, c),
            PopoutType::TrayMenu => TrayMenuPopout::render(center_x, c),
            PopoutType::Updates => UpdatesPopout::render(center_x, c),
            PopoutType::Weather => WeatherPopout::render(center_x, c),
            PopoutType::WirelessPassword => WirelessPasswordPopout::render(center_x, c),
        }
    }
}
