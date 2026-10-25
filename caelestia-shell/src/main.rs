//! Caelestia Shell Daemon
//! GPU-accelerated desktop shell for Caelestia KDE on Wayland.

pub mod components;
mod domain;
mod infrastructure;
mod modules;
mod services;

use domain::tokens::DRAWER_WIDTH;
use gpui::{
    layer_shell::*, point, px, size, App, AppContext, Bounds,
    WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions,
};
use modules::session::SessionDrawer;
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

        let options = WindowOptions {
            titlebar: None,
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(0.), px(0.)),
                size: size(px(DRAWER_WIDTH), px(500.)),
            })),
            app_id: Some("caelestia-shell".to_string()),
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace: "caelestia-session".to_string(),
                layer: Layer::Overlay,
                anchor: Anchor::RIGHT,
                margin: Some((px(0.), px(0.), px(0.), px(0.))),
                exclusive_zone: None,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                ..Default::default()
            }),
            ..Default::default()
        };

        match cx.open_window(options, |_, cx| cx.new(SessionDrawer::new)) {
            Ok(handle) => info!("Session Drawer window handle opened: {:?}", handle),
            Err(err) => tracing::error!("Failed to open Session Drawer window: {:?}", err),
        }
    });
}
