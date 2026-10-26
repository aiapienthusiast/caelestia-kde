//! Desktop background widgets subsystem
//! Ported with 100% parity from `shell/modules/background/desktopicons/widgets/`.

pub mod calendar;
pub mod clock;
pub mod lyrics;
pub mod media;
pub mod note;
pub mod shapes;
pub mod system;

pub use calendar::{CalendarEventItem, DesktopCalendarWidgetModel};
pub use clock::{ClockStyle, DesktopClockWidgetModel};
pub use lyrics::{DesktopLyricLine, DesktopLyricsWidgetModel};
pub use media::DesktopMediaWidgetModel;
pub use note::DesktopNoteWidgetModel;
pub use shapes::{DesktopShapesWidgetModel, ShapeBlobDef};
pub use system::DesktopSystemWidgetModel;
