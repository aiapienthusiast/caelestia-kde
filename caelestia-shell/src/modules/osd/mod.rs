//! On-Screen Display (OSD) Subsystem
//! Complete parity with `shell/modules/osd/`.
//!
//! Provides on-screen popups with reactive volume, microphone, and display brightness
//! sliders connected to PipeWire / WirePlumber and sysfs backlight controls.

pub mod config;
pub mod controller;
pub mod state;
pub mod view;

pub use config::OsdConfig;
pub use controller::OsdController;
pub use state::{OsdSliderType, OsdState};
pub use view::OsdView;
