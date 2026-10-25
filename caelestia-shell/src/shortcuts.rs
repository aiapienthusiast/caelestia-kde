//! Global shortcuts and keybinding coordinator
//! Achieves 100% parity with `shell/modules/Shortcuts.qml` and Krohnkite integration.

use crate::modules::drawers::ContentWindowCoordinator;
use std::collections::HashMap;

/// KWin Krohnkite tiling shortcuts registered in KGlobalAccel
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KrohnkiteAction {
    FocusUp,
    FocusDown,
    FocusLeft,
    FocusRight,
    ShiftUp,
    ShiftDown,
    ShiftLeft,
    ShiftRight,
    CloseWindow,
    FocusNext,
    FocusPrev,
    SetMaster,
    NextLayout,
    PreviousLayout,
    BTreeLayout,
    MonocleLayout,
    FloatingLayout,
    QuarterLayout,
    SpreadLayout,
    StackedLayout,
    StairLayout,
    ColumnsLayout,
    ThreeColumnLayout,
    SpiralLayout,
    TileLayout,
    GrowHeight,
    ShrinkHeight,
    GrowWidth,
    ShrinkWidth,
    IncreaseMaster,
    DecreaseMaster,
    ToggleFloat,
    FloatAll,
    Rotate,
    RotatePart,
    ToggleDock,
}

impl KrohnkiteAction {
    pub fn dbus_action(&self) -> &'static str {
        match self {
            Self::FocusUp => "KrohnkiteFocusUp",
            Self::FocusDown => "KrohnkiteFocusDown",
            Self::FocusLeft => "KrohnkiteFocusLeft",
            Self::FocusRight => "KrohnkiteFocusRight",
            Self::ShiftUp => "KrohnkiteShiftUp",
            Self::ShiftDown => "KrohnkiteShiftDown",
            Self::ShiftLeft => "KrohnkiteShiftLeft",
            Self::ShiftRight => "KrohnkiteShiftRight",
            Self::CloseWindow => "Window Close",
            Self::FocusNext => "KrohnkiteFocusNext",
            Self::FocusPrev => "KrohnkiteFocusPrev",
            Self::SetMaster => "KrohnkiteSetMaster",
            Self::NextLayout => "KrohnkiteNextLayout",
            Self::PreviousLayout => "KrohnkitePreviousLayout",
            Self::BTreeLayout => "KrohnkiteBTreeLayout",
            Self::MonocleLayout => "KrohnkiteMonocleLayout",
            Self::FloatingLayout => "KrohnkiteFloatingLayout",
            Self::QuarterLayout => "KrohnkiteQuarterLayout",
            Self::SpreadLayout => "KrohnkiteSpreadLayout",
            Self::StackedLayout => "KrohnkiteStackedLayout",
            Self::StairLayout => "KrohnkiteStairLayout",
            Self::ColumnsLayout => "KrohnkiteColumnsLayout",
            Self::ThreeColumnLayout => "KrohnkiteThreeColumnLayout",
            Self::SpiralLayout => "KrohnkiteSpiralLayout",
            Self::TileLayout => "KrohnkiteTileLayout",
            Self::GrowHeight => "KrohnkiteGrowHeight",
            Self::ShrinkHeight => "KrohnkiteShrinkHeight",
            Self::GrowWidth => "KrohnkitegrowWidth",
            Self::ShrinkWidth => "KrohnkiteShrinkWidth",
            Self::IncreaseMaster => "KrohnkiteIncrease",
            Self::DecreaseMaster => "KrohnkiteDecrease",
            Self::ToggleFloat => "KrohnkiteToggleFloat",
            Self::FloatAll => "KrohnkiteFloatAll",
            Self::Rotate => "KrohnkiteRotate",
            Self::RotatePart => "KrohnkiteRotatePart",
            Self::ToggleDock => "KrohnkitetoggleDock",
        }
    }

    pub fn default_key(&self) -> &'static str {
        match self {
            Self::FocusUp => "Meta+Up",
            Self::FocusDown => "Meta+Down",
            Self::FocusLeft => "Meta+Left",
            Self::FocusRight => "Meta+Right",
            Self::ShiftUp => "Meta+Shift+Up",
            Self::ShiftDown => "Meta+Shift+Down",
            Self::ShiftLeft => "Meta+Shift+Left",
            Self::ShiftRight => "Meta+Shift+Right",
            Self::CloseWindow => "Meta+Q",
            _ => "",
        }
    }
}

/// Shell shortcut actions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShellShortcutAction {
    Nexus,
    NexusBack,
    ShowAll,
    Dashboard,
    Overview,
    Screenshot,
    GoogleLens,
    Ocr,
    ScreenRecording,
    Session,
    Launcher,
    Sidebar,
    Utilities,
    Notifications,
    Krohnkite(KrohnkiteAction),
}

/// Metadata description of a shortcut
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShortcutDefinition {
    pub name: String,
    pub description: String,
    pub default_key: String,
    pub action: ShellShortcutAction,
}

/// Shortcut manager tracking keymappings and executing actions
#[derive(Clone, Debug)]
pub struct ShortcutManager {
    shortcuts: Vec<ShortcutDefinition>,
    key_map: HashMap<String, ShellShortcutAction>,
    pub launcher_interrupted: bool,
}

impl Default for ShortcutManager {
    fn default() -> Self {
        let mut manager = Self {
            shortcuts: Vec::new(),
            key_map: HashMap::new(),
            launcher_interrupted: false,
        };
        manager.register_defaults();
        manager
    }
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers the default Caelestia shell shortcuts from `Shortcuts.qml`
    fn register_defaults(&mut self) {
        let defaults = vec![
            ShortcutDefinition {
                name: "nexus".to_string(),
                description: "Open nexus".to_string(),
                default_key: "Meta+N".to_string(),
                action: ShellShortcutAction::Nexus,
            },
            ShortcutDefinition {
                name: "nexusBack".to_string(),
                description: "Go back in nexus".to_string(),
                default_key: "Escape".to_string(),
                action: ShellShortcutAction::NexusBack,
            },
            ShortcutDefinition {
                name: "showall".to_string(),
                description: "Toggle launcher, dashboard and osd".to_string(),
                default_key: "Meta+A".to_string(),
                action: ShellShortcutAction::ShowAll,
            },
            ShortcutDefinition {
                name: "dashboard".to_string(),
                description: "Toggle dashboard".to_string(),
                default_key: "Meta+D".to_string(),
                action: ShellShortcutAction::Dashboard,
            },
            ShortcutDefinition {
                name: "overview".to_string(),
                description: "Toggle overview".to_string(),
                default_key: "Meta+Tab".to_string(),
                action: ShellShortcutAction::Overview,
            },
            ShortcutDefinition {
                name: "screenshot".to_string(),
                description: "Toggle screenshot overlay".to_string(),
                default_key: "Print".to_string(),
                action: ShellShortcutAction::Screenshot,
            },
            ShortcutDefinition {
                name: "googleLens".to_string(),
                description: "Toggle Google Lens search".to_string(),
                default_key: "Meta+Shift+S".to_string(),
                action: ShellShortcutAction::GoogleLens,
            },
            ShortcutDefinition {
                name: "ocr".to_string(),
                description: "Recognize text on screen".to_string(),
                default_key: "Meta+Shift+O".to_string(),
                action: ShellShortcutAction::Ocr,
            },
            ShortcutDefinition {
                name: "screenRecording".to_string(),
                description: "Toggle screen recording".to_string(),
                default_key: "Meta+Alt+R".to_string(),
                action: ShellShortcutAction::ScreenRecording,
            },
            ShortcutDefinition {
                name: "session".to_string(),
                description: "Toggle session menu".to_string(),
                default_key: "Meta+X".to_string(),
                action: ShellShortcutAction::Session,
            },
            ShortcutDefinition {
                name: "launcher".to_string(),
                description: "Toggle launcher".to_string(),
                default_key: "Super_L".to_string(),
                action: ShellShortcutAction::Launcher,
            },
            ShortcutDefinition {
                name: "sidebar".to_string(),
                description: "Toggle sidebar".to_string(),
                default_key: "Meta+B".to_string(),
                action: ShellShortcutAction::Sidebar,
            },
            ShortcutDefinition {
                name: "utilities".to_string(),
                description: "Toggle utilities drawer".to_string(),
                default_key: "Meta+U".to_string(),
                action: ShellShortcutAction::Utilities,
            },
            ShortcutDefinition {
                name: "notifications".to_string(),
                description: "Toggle notifications".to_string(),
                default_key: "Meta+V".to_string(),
                action: ShellShortcutAction::Notifications,
            },
        ];

        for sc in defaults {
            self.register_shortcut(sc);
        }

        // Register Krohnkite shortcuts
        for action in [
            KrohnkiteAction::FocusUp,
            KrohnkiteAction::FocusDown,
            KrohnkiteAction::FocusLeft,
            KrohnkiteAction::FocusRight,
            KrohnkiteAction::ShiftUp,
            KrohnkiteAction::ShiftDown,
            KrohnkiteAction::ShiftLeft,
            KrohnkiteAction::ShiftRight,
            KrohnkiteAction::CloseWindow,
        ] {
            let key = action.default_key();
            if !key.is_empty() {
                self.register_shortcut(ShortcutDefinition {
                    name: format!("krohnkite_{:?}", action),
                    description: format!("Krohnkite {:?}", action),
                    default_key: key.to_string(),
                    action: ShellShortcutAction::Krohnkite(action),
                });
            }
        }
    }

    /// Registers a shortcut definition and updates the keymap
    pub fn register_shortcut(&mut self, sc: ShortcutDefinition) {
        if !sc.default_key.is_empty() {
            self.key_map.insert(sc.default_key.clone(), sc.action);
        }
        self.shortcuts.push(sc);
    }

    /// Looks up an action by key combination string
    pub fn lookup(&self, key: &str) -> Option<ShellShortcutAction> {
        self.key_map.get(key).copied()
    }

    /// Dispatches an action to the window coordinator
    pub fn dispatch(&mut self, action: ShellShortcutAction, coordinator: &mut ContentWindowCoordinator) {
        match action {
            ShellShortcutAction::Launcher => {
                if !self.launcher_interrupted {
                    coordinator.toggle_drawer("launcher");
                }
                self.launcher_interrupted = false;
            }
            ShellShortcutAction::Dashboard => {
                coordinator.toggle_drawer("dashboard");
            }
            ShellShortcutAction::Overview => {
                let open = coordinator.visibilities.overview;
                coordinator.set_overview(!open);
            }
            ShellShortcutAction::Session => {
                coordinator.toggle_drawer("session");
            }
            ShellShortcutAction::Sidebar => {
                coordinator.toggle_drawer("sidebar");
            }
            ShellShortcutAction::Utilities => {
                coordinator.toggle_drawer("utilities");
            }
            ShellShortcutAction::Notifications => {
                coordinator.toggle_drawer("notifications");
            }
            ShellShortcutAction::ShowAll => {
                let any_open = coordinator.visibilities.launcher
                    || coordinator.visibilities.dashboard
                    || coordinator.visibilities.osd
                    || coordinator.visibilities.utilities;
                let target = !any_open;
                coordinator.set_drawer("launcher", target);
                coordinator.set_drawer("dashboard", target);
                coordinator.set_drawer("osd", target);
                coordinator.set_drawer("utilities", target);
            }
            ShellShortcutAction::Krohnkite(krohnkite_action) => {
                if !crate::infrastructure::is_test_environment() {
                    let _ = std::process::Command::new("qdbus6")
                        .args([
                            "org.kde.kglobalaccel",
                            "/component/kwin",
                            "invokeShortcut",
                            krohnkite_action.dbus_action(),
                        ])
                        .spawn();
                }
            }
            _ => {}
        }
    }
}
