//! Utilities Subsystem
//! Complete parity with `shell/modules/utilities/`.
//!
//! Provides the slide-out utilities panel hosting:
//! - Keep Awake (Idle Inhibit)
//! - Screen Recorder & Screenshots history
//! - QuickShare nearby device transfers
//! - Paged Quick Toggles grid

pub mod capture;
pub mod config;
pub mod idle_inhibit;
pub mod quick_share;
pub mod state;
pub mod toggles;
pub mod view;

pub use capture::{CaptureCardModel, CaptureItem, CaptureMode};
pub use config::{QuickToggleItem, UtilitiesConfig};
pub use idle_inhibit::{IdleInhibitState, IdleInhibitor};
pub use quick_share::{NearbyDevice, QuickShareModel, QuickShareTransfer};
pub use state::UtilitiesState;
pub use toggles::QuickTogglesModel;
pub use view::UtilitiesView;
