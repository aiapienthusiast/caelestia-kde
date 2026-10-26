//! Caelestia Shell Daemon
//! GPU-accelerated desktop shell for Caelestia KDE on Wayland.

pub mod components;
pub mod domain;
pub mod infrastructure;
pub mod modules;
pub mod services;
pub use modules::shortcuts;


use gpui::{
    layer_shell::*, point, px, size, App, AppContext, Bounds,
    TitlebarOptions, WindowBackgroundAppearance, WindowBounds, WindowDecorations,
    WindowKind, WindowOptions,
};
use infrastructure::theme::CaelestiaColours;
use modules::background::BackgroundView;
use modules::bar::BarView;
use modules::dashboard::DashboardDrawer;
use modules::launcher::LauncherView;
use modules::nexus::NexusWindow;
use modules::notifications::NotificationsView;
use modules::osd::{OsdConfig, OsdState, OsdView};
use modules::overview::OverviewView;
use modules::screenshot::{
    ScreenshotOverlayView, SelectionMode, SelectionPhase, SelectionRegion,
};
use modules::session::SessionDrawer;
use modules::sidebar::SidebarView;
use modules::utilities::UtilitiesView;
use tracing::info;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Caelestia Shell daemon on Wayland Layer-Shell...");

    let app = gpui::Application::with_platform(gpui_linux::current_platform(false));

    app.run(|cx: &mut App| {
        cx.set_quit_mode(gpui::QuitMode::Explicit);

        let args: Vec<String> = std::env::args().collect();
        let colours = CaelestiaColours::load();

        let is_bar = std::env::var("BAR_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--bar" || arg == "bar");
        let is_osd = std::env::var("OSD_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--osd" || arg == "osd");
        let is_dashboard = std::env::var("DASHBOARD_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--dashboard" || arg == "dashboard");
        let is_nexus = std::env::var("NEXUS_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--nexus" || arg == "nexus");
        let is_session = std::env::var("SESSION_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--session" || arg == "session");
        let is_launcher = std::env::var("LAUNCHER_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--launcher" || arg == "launcher");
        let is_overview = std::env::var("OVERVIEW_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--overview" || arg == "overview");
        let is_sidebar = std::env::var("SIDEBAR_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--sidebar" || arg == "sidebar");
        let is_utilities = std::env::var("UTILITIES_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--utilities" || arg == "utilities");
        let is_screenshot = std::env::var("SCREENSHOT_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--screenshot" || arg == "screenshot");
        let is_notifications = std::env::var("NOTIFICATIONS_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--notifications" || arg == "notifications");
        let is_background = std::env::var("BACKGROUND_OPEN").is_ok()
            || args.iter().any(|arg| arg == "--background" || arg == "background");

        if is_bar {
            info!("Launching Status Bar Surface...");
            let bar_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: size(px(1920.), px(48.)),
                })),
                app_id: Some("caelestia-bar".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-bar".to_string(),
                    layer: Layer::Top,
                    anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
                    exclusive_zone: Some(px(48.)),
                    keyboard_interactivity: KeyboardInteractivity::None,
                    ..Default::default()
                }),
                ..Default::default()
            };
            match cx.open_window(bar_options, |_, cx| {
                cx.new(|_| BarView::new(colours))
            }) {
                Ok(handle) => info!("Bar window opened successfully: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Bar window: {:?}", err),
            }
        } else if is_osd {
            info!("Launching OSD Surface...");
            let osd_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(1920. - 100.), px(400.)),
                    size: size(px(100.), px(300.)),
                })),
                app_id: Some("caelestia-osd".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-osd".to_string(),
                    layer: Layer::Overlay,
                    anchor: Anchor::RIGHT,
                    exclusive_zone: None,
                    keyboard_interactivity: KeyboardInteractivity::None,
                    ..Default::default()
                }),
                ..Default::default()
            };
            match cx.open_window(osd_options, |_, cx| {
                cx.new(|_| OsdView::new(
                    OsdConfig::default(),
                    OsdState {
                        is_open: true,
                        offset_scale: 0.0,
                        volume: 0.75,
                        brightness: 0.60,
                        ..Default::default()
                    },
                    colours
                ))
            }) {
                Ok(handle) => info!("OSD window opened successfully: {:?}", handle),
                Err(err) => tracing::error!("Failed to open OSD window: {:?}", err),
            }
        } else if is_dashboard {
            info!("Launching Dashboard Multi-Tab Drawer...");
            let dash_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: size(px(1920.), px(1080.)),
                })),
                app_id: Some("caelestia-dashboard".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-drawers".to_string(),
                    layer: Layer::Overlay,
                    anchor: Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
                    margin: Some((px(0.), px(0.), px(0.), px(0.))),
                    exclusive_zone: None,
                    keyboard_interactivity: KeyboardInteractivity::OnDemand,
                    ..Default::default()
                }),
                ..Default::default()
            };

            match cx.open_window(dash_options, |_, cx| cx.new(DashboardDrawer::new)) {
                Ok(handle) => info!("Dashboard window handle opened successfully: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Dashboard window: {:?}", err),
            }
        } else if is_nexus {
            info!("Launching Nexus Control Center & Settings Standard Window...");
            let nexus_options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Caelestia Settings".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(120.), px(80.)),
                    size: size(px(1080.), px(720.)),
                })),
                app_id: Some("caelestia-nexus".to_string()),
                window_background: WindowBackgroundAppearance::Blurred,
                window_decorations: Some(WindowDecorations::Server),
                kind: WindowKind::Normal,
                ..Default::default()
            };

            match cx.open_window(nexus_options, |_, cx| cx.new(NexusWindow::new)) {
                Ok(handle) => info!("Nexus standard window opened successfully: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Nexus window: {:?}", err),
            }
        } else if is_launcher {
            info!("Launching Application Launcher Drawer...");
            let launcher_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: size(px(1920.), px(1080.)),
                })),
                app_id: Some("caelestia-launcher".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-launcher".to_string(),
                    layer: Layer::Overlay,
                    anchor: Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
                    keyboard_interactivity: KeyboardInteractivity::Exclusive,
                    ..Default::default()
                }),
                ..Default::default()
            };

            match cx.open_window(launcher_options, |_, cx| {
                cx.new(|cx| LauncherView::new(cx, colours))
            }) {
                Ok(handle) => info!("Launcher window opened: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Launcher window: {:?}", err),
            }
        } else if is_overview {
            info!("Launching Overview Canvas...");
            let overview_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: size(px(1920.), px(1080.)),
                })),
                app_id: Some("caelestia-overview".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-overview".to_string(),
                    layer: Layer::Overlay,
                    anchor: Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
                    keyboard_interactivity: KeyboardInteractivity::Exclusive,
                    ..Default::default()
                }),
                ..Default::default()
            };

            match cx.open_window(overview_options, |_, cx| {
                cx.new(|cx| OverviewView::new(cx, colours))
            }) {
                Ok(handle) => info!("Overview window opened: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Overview window: {:?}", err),
            }
        } else if is_sidebar {
            info!("Launching Sidebar Drawer...");
            let sidebar_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(1920. - 380.), px(0.)),
                    size: size(px(380.), px(1080.)),
                })),
                app_id: Some("caelestia-sidebar".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-sidebar".to_string(),
                    layer: Layer::Overlay,
                    anchor: Anchor::TOP | Anchor::BOTTOM | Anchor::RIGHT,
                    keyboard_interactivity: KeyboardInteractivity::OnDemand,
                    ..Default::default()
                }),
                ..Default::default()
            };

            match cx.open_window(sidebar_options, |_, cx| {
                cx.new(|cx| SidebarView::new(cx, colours, 1920.0))
            }) {
                Ok(handle) => info!("Sidebar window opened: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Sidebar window: {:?}", err),
            }
        } else if is_utilities {
            info!("Launching Utilities Drawer...");
            let util_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(1920. - 360.), px(48.)),
                    size: size(px(360.), px(600.)),
                })),
                app_id: Some("caelestia-utilities".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-utilities".to_string(),
                    layer: Layer::Overlay,
                    anchor: Anchor::TOP | Anchor::RIGHT,
                    keyboard_interactivity: KeyboardInteractivity::OnDemand,
                    ..Default::default()
                }),
                ..Default::default()
            };

            match cx.open_window(util_options, |_, cx| {
                cx.new(|cx| UtilitiesView::new(cx, colours))
            }) {
                Ok(handle) => info!("Utilities window opened: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Utilities window: {:?}", err),
            }
        } else if is_screenshot {
            info!("Launching Screenshot Overlay View...");
            let sc_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: size(px(1920.), px(1080.)),
                })),
                app_id: Some("caelestia-screenshot".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-screenshot".to_string(),
                    layer: Layer::Overlay,
                    anchor: Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
                    keyboard_interactivity: KeyboardInteractivity::Exclusive,
                    ..Default::default()
                }),
                ..Default::default()
            };

            match cx.open_window(sc_options, |_, cx| {
                cx.new(|cx| {
                    let mut view = ScreenshotOverlayView::new(cx, colours, 1920.0, 1080.0);
                    view.state.is_active = true;
                    view.state.phase = SelectionPhase::Post;
                    view.state.selection = Some(SelectionRegion {
                        mode: SelectionMode::RectCorners,
                        start_x: 480.0,
                        start_y: 320.0,
                        current_x: 1040.0,
                        current_y: 640.0,
                        snapped_window_address: None,
                    });
                    view
                })
            }) {
                Ok(handle) => info!("Screenshot window opened: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Screenshot window: {:?}", err),
            }
        } else if is_notifications {
            info!("Launching Notifications Popup View...");
            let notif_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(1920. - 400.), px(48.)),
                    size: size(px(380.), px(600.)),
                })),
                app_id: Some("caelestia-notifications".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-notifications".to_string(),
                    layer: Layer::Overlay,
                    anchor: Anchor::TOP | Anchor::RIGHT,
                    keyboard_interactivity: KeyboardInteractivity::None,
                    ..Default::default()
                }),
                ..Default::default()
            };

            match cx.open_window(notif_options, |_, cx| {
                cx.new(|_| NotificationsView::new(colours))
            }) {
                Ok(handle) => info!("Notifications window opened: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Notifications window: {:?}", err),
            }
        } else if is_session {
            info!("Launching Session Drawer...");
            let options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(1920. - 116.), px(280.)),
                    size: size(px(116.), px(520.)),
                })),
                app_id: Some("caelestia-session".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-session".to_string(),
                    layer: Layer::Overlay,
                    anchor: Anchor::RIGHT,
                    keyboard_interactivity: KeyboardInteractivity::OnDemand,
                    ..Default::default()
                }),
                ..Default::default()
            };

            match cx.open_window(options, |_, cx| cx.new(SessionDrawer::new)) {
                Ok(handle) => info!("Session Drawer window handle opened: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Session Drawer window: {:?}", err),
            }
        } else if is_background {
            info!("Launching Background Surface...");
            let bg_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: size(px(1920.), px(1080.)),
                })),
                app_id: Some("caelestia-background".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-background".to_string(),
                    layer: Layer::Background,
                    anchor: Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
                    exclusive_zone: None,
                    keyboard_interactivity: KeyboardInteractivity::None,
                    ..Default::default()
                }),
                ..Default::default()
            };

            match cx.open_window(bg_options, |_, cx| {
                cx.new(|cx| BackgroundView::new(cx, colours))
            }) {
                Ok(handle) => info!("Background window opened: {:?}", handle),
                Err(err) => tracing::error!("Failed to open Background window: {:?}", err),
            }
        } else {
            // Default: Unified daemon hosting the Bar and Desktop Background
            info!("Launching Caelestia Desktop Shell unified daemon...");

            // Background surface
            let bg_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: size(px(1920.), px(1080.)),
                })),
                app_id: Some("caelestia-background".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-background".to_string(),
                    layer: Layer::Background,
                    anchor: Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
                    exclusive_zone: None,
                    keyboard_interactivity: KeyboardInteractivity::None,
                    ..Default::default()
                }),
                ..Default::default()
            };
            let _ = cx.open_window(bg_options, |_, cx| {
                cx.new(|cx| BackgroundView::new(cx, colours.clone()))
            });

            // Top Status Bar
            let bar_options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: size(px(1920.), px(48.)),
                })),
                app_id: Some("caelestia-bar".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-bar".to_string(),
                    layer: Layer::Top,
                    anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
                    exclusive_zone: Some(px(48.)),
                    keyboard_interactivity: KeyboardInteractivity::None,
                    ..Default::default()
                }),
                ..Default::default()
            };
            let _ = cx.open_window(bar_options, |_, cx| {
                cx.new(|_| BarView::new(colours))
            });

            info!("Caelestia Shell unified daemon initialized successfully.");
        }
    });
}
