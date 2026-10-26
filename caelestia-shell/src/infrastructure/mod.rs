pub mod config;
pub mod display_capabilities;
pub mod env;
pub mod geometry;
pub mod icons;
pub mod loader_diagnostics;
pub mod paths;
pub mod solar;
pub mod strings;
pub mod sysinfo;
pub mod theme;

pub use display_capabilities::{PanelCapabilitiesDetector, PanelCapability};
pub use env::{is_test_environment, set_test_mode};
pub use geometry::{EdgeAnchor, GeometryUtils, Rect};
pub use icons::IconResolver;
pub use loader_diagnostics::{
    DiagnosticCategory, DiagnosticEntry, DiagnosticStatus, LoaderDiagnostics,
};
pub use solar::{Coordinates, SolarCalculator, SolarTimes};
pub use strings::StringUtils;

