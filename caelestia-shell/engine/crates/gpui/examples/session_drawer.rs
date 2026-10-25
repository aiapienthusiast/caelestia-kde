#![cfg_attr(target_family = "wasm", no_main)]

fn run_example() {
    #[cfg(all(target_os = "linux", feature = "wayland"))]
    example::main();

    #[cfg(not(all(target_os = "linux", feature = "wayland")))]
    panic!("This example requires the `wayland` feature on Linux.");
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(all(target_os = "linux", feature = "wayland"))]
mod example {
    use std::process::Command;
    use std::time::{Duration, Instant, SystemTime};
    use gpui::{
        div, layer_shell::*, point, prelude::*, px, rgba, size,
        App, Bounds, Context, CursorStyle, FocusHandle, FontWeight,
        InteractiveElement, IntoElement, KeyDownEvent, MouseButton,
        MouseDownEvent, MouseExitEvent, MouseMoveEvent, MouseUpEvent,
        Pixels, Render, Styled,
        Window, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions,
    };
    use gpui_platform::application;

    // =========================================================================
    // Material Design 3 Design Tokens (Identical to Caelestia KDE tokens.hpp)
    // =========================================================================
    #[allow(dead_code)]
    pub struct Tokens;
    #[allow(dead_code)]
    impl Tokens {
        pub const ROUNDING_SMALL: Pixels = px(8.0);            // Pressed radius
        pub const ROUNDING_MEDIUM: Pixels = px(12.0);
        pub const ROUNDING_LARGE: Pixels = px(16.0);
        pub const ROUNDING_LARGE_INCREASED: Pixels = px(20.0);  // Inactive button radius
        pub const ROUNDING_EXTRA_LARGE: Pixels = px(28.0);      // Leading blob curve / active focus
        pub const SPACING_EXTRA_SMALL: Pixels = px(4.0);
        pub const SPACING_LARGE: Pixels = px(16.0);
        pub const PADDING_LARGE: Pixels = px(16.0);
        pub const BUTTON_WIDTH: Pixels = px(80.0);              // Tokens.sizes.session.button
        pub const BUTTON_HEIGHT: Pixels = px(76.0);
    }

    // =========================================================================
    // Caelestia Colours Service (Parity with shell/services/Colours.qml)
    // =========================================================================
    #[derive(Clone, Debug)]
    pub struct CaelestiaColours {
        pub surface: u32,
        pub surface_container: u32,
        pub surface_container_high: u32,
        pub secondary_container: u32,
        pub on_surface: u32,
        pub on_secondary_container: u32,
        pub outline_variant: u32,
        pub primary: u32,
        pub mascot_bg: u32,
    }

    fn parse_hex_color(s: &str, alpha: f32) -> u32 {
        let clean = s.trim().trim_start_matches('#');
        if clean.len() >= 6 {
            let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0);
            let a = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
            ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
        } else {
            0x000000ff
        }
    }

    fn alter_colour(hex: &str, alpha: f32, light: bool, transp_base: f32, layer: i32) -> u32 {
        let clean = hex.trim().trim_start_matches('#');
        if clean.len() < 6 {
            return 0x000000ff;
        }
        let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0) as f32 / 255.0;

        let luminance = (0.299 * r * r + 0.587 * g * g + 0.114 * b * b).sqrt();
        if luminance <= 0.0 {
            return parse_hex_color(hex, alpha);
        }

        let layer_sign = if !light || layer == 1 { 1.0 } else { -(layer as f32) / 2.0 };
        let light_mul = if light { 0.2 } else { 0.3 };
        let wall_factor = if light { if layer == 1 { 3.0 } else { 1.0 } } else { 2.5 };
        let offset = layer_sign * light_mul * (1.0 - transp_base) * (1.0 + 0.25 * wall_factor);
        let scale = (luminance + offset) / luminance;

        let r_out = (r * scale).clamp(0.0, 1.0);
        let g_out = (g * scale).clamp(0.0, 1.0);
        let b_out = (b * scale).clamp(0.0, 1.0);

        let r_byte = (r_out * 255.0).round() as u8;
        let g_byte = (g_out * 255.0).round() as u8;
        let b_byte = (b_out * 255.0).round() as u8;
        let a_byte = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;

        ((r_byte as u32) << 24) | ((g_byte as u32) << 16) | ((b_byte as u32) << 8) | (a_byte as u32)
    }

    impl Default for CaelestiaColours {
        fn default() -> Self {
            Self {
                surface: 0x1a111094,
                surface_container: 0x271d1c80,
                surface_container_high: 0x322826a0,
                secondary_container: 0x5d3f3be6,
                on_surface: 0xf1dedcff,
                on_secondary_container: 0xffdad5ff,
                outline_variant: 0x53434140,
                primary: 0xffb4aaff,
                mascot_bg: 0x140c0b80,
            }
        }
    }

    impl CaelestiaColours {
        pub fn load() -> Self {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let scheme_path = format!("{}/.local/state/caelestia/scheme.json", home);
            let shell_path = format!("{}/.config/caelestia/shell.json", home);

            let mut transp_base = 0.58f32;
            let mut transp_layers = 0.50f32;

            if let Ok(shell_str) = std::fs::read_to_string(&shell_path) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&shell_str) {
                    if let Some(appearance) = val.get("appearance") {
                        if let Some(tr) = appearance.get("transparency") {
                            if let Some(b) = tr.get("base").and_then(|v| v.as_f64()) {
                                transp_base = b as f32;
                            }
                            if let Some(l) = tr.get("layers").and_then(|v| v.as_f64()) {
                                transp_layers = l as f32;
                            }
                        }
                    }
                }
            }

            let scheme_str = match std::fs::read_to_string(&scheme_path) {
                Ok(s) => s,
                Err(_) => return Self::default(),
            };

            let scheme: serde_json::Value = match serde_json::from_str(&scheme_str) {
                Ok(v) => v,
                Err(_) => return Self::default(),
            };

            let is_light = scheme.get("mode").and_then(|m| m.as_str()) == Some("light");
            let colours = match scheme.get("colours") {
                Some(c) => c,
                None => return Self::default(),
            };

            let get_hex = |key: &str, fallback: &str| -> String {
                colours
                    .get(key)
                    .and_then(|v| v.as_str())
                    .unwrap_or(fallback)
                    .to_string()
            };

            let surface_hex = get_hex("surface", "1a1110");
            let surface_container_hex = get_hex("surfaceContainer", "271d1c");
            let surface_container_high_hex = get_hex("surfaceContainerHigh", "322826");
            let secondary_container_hex = get_hex("secondaryContainer", "5d3f3b");
            let on_surface_hex = get_hex("onSurface", "f1dedc");
            let on_secondary_container_hex = get_hex("onSecondaryContainer", "ffdad5");
            let outline_variant_hex = get_hex("outlineVariant", "534341");
            let primary_hex = get_hex("primary", "ffb4aa");
            let mascot_hex = get_hex("surfaceContainerLowest", "140c0b");

            Self {
                surface: parse_hex_color(&surface_hex, transp_base),
                surface_container: alter_colour(&surface_container_hex, transp_layers, is_light, transp_base, 1),
                surface_container_high: alter_colour(&surface_container_high_hex, (transp_layers + 0.15).min(1.0), is_light, transp_base, 1),
                secondary_container: parse_hex_color(&secondary_container_hex, 0.90),
                on_surface: parse_hex_color(&on_surface_hex, 1.0),
                on_secondary_container: parse_hex_color(&on_secondary_container_hex, 1.0),
                outline_variant: parse_hex_color(&outline_variant_hex, 0.25),
                primary: parse_hex_color(&primary_hex, 1.0),
                mascot_bg: parse_hex_color(&mascot_hex, transp_layers),
            }
        }
    }

    #[derive(Clone)]
    struct SessionAction {
        id: usize,
        label: &'static str,
        icon: &'static str,
        command: Vec<&'static str>,
    }

    pub struct SessionDrawer {
        focus_handle: FocusHandle,
        actions: Vec<SessionAction>,
        focused_index: usize,
        pressed_index: Option<usize>,
        hovered_index: Option<usize>,
        mascot_pulse: f32,

        // Edge Hover & Blobs Physics State (matching Caelestia Blobs)
        is_hovered: bool,
        open_progress: f32, // 0.0 = tucked into right edge, 1.0 = fully open
        velocity: f32,
        deform_x: f32,      // Soft-body horizontal stretch factor
        deform_y: f32,      // Soft-body vertical compression factor

        colours: CaelestiaColours,
        last_scheme_check: Instant,
        last_scheme_mtime: Option<SystemTime>,
    }

    impl SessionDrawer {
        pub fn new(cx: &mut Context<Self>) -> Self {
            let focus_handle = cx.focus_handle();
            let actions = vec![
                SessionAction {
                    id: 0,
                    label: "Log out",
                    icon: "󰍃",
                    command: vec!["sh", "-c", "qdbus6 org.kde.Shutdown /Shutdown org.kde.Shutdown.logout 2>/dev/null"],
                },
                SessionAction {
                    id: 1,
                    label: "Shut down",
                    icon: "⏻",
                    command: vec!["systemctl", "poweroff"],
                },
                SessionAction {
                    id: 2,
                    label: "Hibernate",
                    icon: "󰒲",
                    command: vec!["systemctl", "hibernate"],
                },
                SessionAction {
                    id: 3,
                    label: "Restart",
                    icon: "󰑓",
                    command: vec!["systemctl", "reboot"],
                },
            ];

            // 60 FPS Animation & Soft-Body Physics Loop
            cx.spawn(async move |this, cx| {
                let mut step: f32 = 0.0;
                let dt = 0.016; // 16ms (~60 FPS)
                loop {
                    cx.background_executor().timer(std::time::Duration::from_millis(16)).await;
                    step += 0.05;
                    let pulse = (step.sin() + 1.0) / 2.0;
                    let _ = this.update(cx, |drawer, cx| {
                        drawer.mascot_pulse = pulse;
                        drawer.tick_physics(dt);
                        cx.notify();
                    });
                }
            }).detach();

            let colours = CaelestiaColours::load();
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let scheme_path = format!("{}/.local/state/caelestia/scheme.json", home);
            let last_scheme_mtime = std::fs::metadata(&scheme_path).ok().and_then(|m| m.modified().ok());

            Self {
                focus_handle,
                actions,
                focused_index: 0,
                pressed_index: None,
                hovered_index: None,
                mascot_pulse: 0.5,
                is_hovered: std::env::var("DRAWER_OPEN").is_ok(),
                open_progress: if std::env::var("DRAWER_OPEN").is_ok() { 1.0 } else { 0.0 },
                velocity: 0.0,
                deform_x: 1.0,
                deform_y: 1.0,
                colours,
                last_scheme_check: Instant::now(),
                last_scheme_mtime,
            }
        }

        /// Spring & inertia physics directly implementing Caelestia Blobs (blobrect.cpp:112-125)
        fn tick_physics(&mut self, dt: f32) {
            let target = if self.is_hovered { 1.0 } else { 0.0 };
            let diff = target - self.open_progress;

            // Spring constants matching Caelestia Blobs stiffness & damping
            let stiffness = 170.0;
            let damping = 17.0;
            let force = diff * stiffness;
            self.velocity += (force - self.velocity * damping) * dt;
            self.open_progress += self.velocity * dt;
            self.open_progress = self.open_progress.clamp(0.0, 1.0);

            // Calculate organic stretch & curvature from velocity (deformAmount = 0.2)
            let speed = self.velocity.abs();
            let target_stretch = 1.0 + (speed * 0.14).min(0.35);
            let target_compress = 1.0 / target_stretch;

            self.deform_x += (target_stretch - self.deform_x) * (dt * 14.0).min(1.0);
            self.deform_y += (target_compress - self.deform_y) * (dt * 14.0).min(1.0);

            // Dynamically reload scheme.json when changed
            if self.last_scheme_check.elapsed() > Duration::from_millis(500) {
                self.last_scheme_check = Instant::now();
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                let scheme_path = format!("{}/.local/state/caelestia/scheme.json", home);
                if let Ok(metadata) = std::fs::metadata(&scheme_path) {
                    if let Ok(mtime) = metadata.modified() {
                        if self.last_scheme_mtime != Some(mtime) {
                            self.last_scheme_mtime = Some(mtime);
                            self.colours = CaelestiaColours::load();
                        }
                    }
                }
            }
        }

        fn execute_action(&self, index: usize) {
            if let Some(action) = self.actions.get(index) {
                println!("[Caelestia-RS] Executing session action: {} ({:?})", action.label, action.command);
                if !action.command.is_empty() {
                    let prog = action.command[0];
                    let args = &action.command[1..];
                    let _ = Command::new(prog).args(args).spawn();
                }
            }
        }

        fn focus_next(&mut self, cx: &mut Context<Self>) {
            self.focused_index = (self.focused_index + 1) % self.actions.len();
            cx.notify();
        }

        fn focus_prev(&mut self, cx: &mut Context<Self>) {
            if self.focused_index == 0 {
                self.focused_index = self.actions.len() - 1;
            } else {
                self.focused_index -= 1;
            }
            cx.notify();
        }

        fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
            let key = event.keystroke.key.as_str();
            let shift = event.keystroke.modifiers.shift;
            let ctrl = event.keystroke.modifiers.control;

            match key {
                "down" | "tab" if !shift => self.focus_next(cx),
                "up" | "tab" if shift => self.focus_prev(cx),
                "j" | "n" if ctrl => self.focus_next(cx),
                "k" | "p" if ctrl => self.focus_prev(cx),
                "enter" | "space" => {
                    self.execute_action(self.focused_index);
                }
                "escape" => {
                    // Tucking away or closing
                    if self.is_hovered {
                        self.is_hovered = false;
                        cx.notify();
                    } else {
                        println!("[Caelestia-RS] Escape pressed. Exiting.");
                        std::process::exit(0);
                    }
                }
                _ => {}
            }
        }
    }

    impl Render for SessionDrawer {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let target_appearance = if self.open_progress > 0.001 {
                WindowBackgroundAppearance::Blurred
            } else {
                WindowBackgroundAppearance::Transparent
            };
            window.set_background_appearance(target_appearance);

            let focused_idx = self.focused_index;
            let pressed_idx = self.pressed_index;
            let hovered_idx = self.hovered_index;
            let mascot_scale = 0.95 + 0.1 * self.mascot_pulse;

            // Outer dimensions and offsets
            let drawer_width = 116.0;
            // Fully slides off-screen to the right when progress is 0.0
            let slide_offset = px((1.0 - self.open_progress) * drawer_width);

            // Dynamic curvature implementing Caelestia Blobs soft-body physics:
            // - Starts at 0px when tucked inside the screen edge
            // - Curves dynamically during opening up to 48px with peak spring velocity
            // - Settles smoothly to M3 Tokens::ROUNDING_EXTRA_LARGE (28.0px) at rest
            let base_curve = 28.0;
            let speed = self.velocity.abs();
            let velocity_bulge = (speed * 4.5).min(20.0);
            let progress_ramp = (self.open_progress * 1.8).min(1.0);
            let left_curve_radius = px((base_curve * progress_ramp + velocity_bulge).clamp(0.0, 50.0));

            // Soft-body horizontal stretch from blobrect.cpp deformMatrix
            let stretch_x = px(((self.deform_x - 1.0) * 16.0).max(0.0));

            let c = &self.colours;

            let render_button = |action: &SessionAction, is_focused: bool, is_pressed: bool, is_hovered: bool| {
                let radius = if is_pressed {
                    Tokens::ROUNDING_SMALL
                } else if is_focused {
                    Tokens::ROUNDING_EXTRA_LARGE
                } else {
                    Tokens::ROUNDING_LARGE_INCREASED
                };

                let bg_color = if is_focused {
                    rgba(c.secondary_container)
                } else if is_hovered {
                    rgba(c.surface_container_high)
                } else {
                    rgba(c.surface_container)
                };

                let text_color = if is_focused {
                    rgba(c.on_secondary_container)
                } else {
                    rgba(c.on_surface)
                };

                let action_id = action.id;

                div()
                    .id(action_id)
                    .w(Tokens::BUTTON_WIDTH)
                    .h(Tokens::BUTTON_HEIGHT)
                    .bg(bg_color)
                    .rounded(radius)
                    .border_1()
                    .border_color(if is_focused { rgba(c.primary) } else { rgba(c.outline_variant) })
                    .cursor(CursorStyle::PointingHand)
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(Tokens::SPACING_EXTRA_SMALL)
                    .on_mouse_down(MouseButton::Left, cx.listener(move |drawer, _: &MouseDownEvent, _window, cx| {
                        drawer.pressed_index = Some(action_id);
                        drawer.focused_index = action_id;
                        cx.notify();
                    }))
                    .on_mouse_up(MouseButton::Left, cx.listener(move |drawer, _: &MouseUpEvent, _window, cx| {
                        if drawer.pressed_index == Some(action_id) {
                            drawer.pressed_index = None;
                            drawer.execute_action(action_id);
                            cx.notify();
                        }
                    }))
                    .on_hover(cx.listener(move |drawer, hovered: &bool, _window, cx| {
                        drawer.hovered_index = if *hovered { Some(action_id) } else { None };
                        cx.notify();
                    }))
                    // Material Icon
                    .child(
                        div()
                            .text_size(px(24.0))
                            .text_color(text_color)
                            .font_weight(FontWeight::BOLD)
                            .child(action.icon)
                    )
                    // StyledText label (Tokens.font.label.small -> 11px)
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(text_color)
                            .font_weight(if is_focused { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                            .child(action.label)
                    )
            };

            // Window root container
            div()
                .id("edge-drawer-container")
                .size_full()
                .relative()
                .track_focus(&self.focus_handle)
                .on_key_down(cx.listener(|drawer, event: &KeyDownEvent, _window, cx| {
                    drawer.handle_key_down(event, cx);
                }))
                // Keep open while mouse is active inside the drawer; open when entering right edge
                .on_mouse_move(cx.listener(|drawer, event: &MouseMoveEvent, _window, cx| {
                    let x = event.position.x;
                    if drawer.open_progress > 0.05 || x >= px(88.0) {
                        if !drawer.is_hovered {
                            drawer.is_hovered = true;
                            cx.notify();
                        }
                    }
                }))
                // Retract when mouse exits the drawer window area
                .on_mouse_exit(cx.listener(|drawer, _event: &MouseExitEvent, _window, cx| {
                    if drawer.is_hovered {
                        drawer.is_hovered = false;
                        cx.notify();
                    }
                }))
                .child(
                    // 1. Sliding & Curving Blob Drawer Body
                    div()
                        .id("sliding-drawer-body")
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .w(px(drawer_width))
                        // Slide translation: moves off-screen to the right when closed,
                        // with dynamic jelly stretch along the X axis during opening
                        .left(slide_offset - stretch_x)
                        .opacity(self.open_progress.clamp(0.0, 1.0))
                        .when(self.open_progress <= 0.001, |el| el.invisible())
                        .p(Tokens::PADDING_LARGE)
                        .pr(px(16.0)) // Attached flush to the right edge
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(Tokens::SPACING_LARGE)
                        // Caelestia Blobs SDF Background Layer:
                        // - Flush with right screen edge: rounded_tr = 0, rounded_br = 0, border_r = 0
                        // - Leading edge curves dynamically during opening animation: rounded_tl & rounded_bl
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .bg(rgba(c.surface))
                                .rounded_tl(left_curve_radius)
                                .rounded_bl(left_curve_radius)
                                .rounded_tr(px(0.0)) // Attached flush to right screen edge
                                .rounded_br(px(0.0)) // Attached flush to right screen edge
                                .border_1()
                                .border_r_0()        // Flush with right screen edge
                                .border_color(rgba(c.outline_variant))
                                .shadow_lg()
                        )
                        // Content Layer (fades in as drawer opens, matching Wrapper.qml opacity)
                        .child(
                            div()
                                .opacity((self.open_progress * 1.2).min(1.0))
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap(Tokens::SPACING_LARGE)
                                // Top buttons: Log out, Shut down
                                .child(render_button(&self.actions[0], focused_idx == 0, pressed_idx == Some(0), hovered_idx == Some(0)))
                                .child(render_button(&self.actions[1], focused_idx == 1, pressed_idx == Some(1), hovered_idx == Some(1)))
                                // Mascot Centerpiece (Dino 80x80 container matching Content.qml)
                                .child(
                                    div()
                                        .w(Tokens::BUTTON_WIDTH)
                                        .h(Tokens::BUTTON_WIDTH)
                                        .rounded(Tokens::ROUNDING_LARGE_INCREASED)
                                        .bg(rgba(c.mascot_bg))
                                        .border_1()
                                        .border_color(rgba(c.outline_variant))
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .justify_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_size(px(26.0 * mascot_scale))
                                                .child("🦖")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(9.0))
                                                .text_color(rgba(c.on_surface))
                                                .font_weight(FontWeight::BOLD)
                                                .child("CAELESTIA")
                                        )
                                )
                                // Bottom buttons: Hibernate, Restart
                                .child(render_button(&self.actions[2], focused_idx == 2, pressed_idx == Some(2), hovered_idx == Some(2)))
                                .child(render_button(&self.actions[3], focused_idx == 3, pressed_idx == Some(3), hovered_idx == Some(3)))
                        )
                )
                .child(
                    // 2. Right-edge Hover Trigger Strip:
                    // Thin 16px edge trigger zone along the extreme right edge of the screen.
                    // Hovering here triggers the opening spring animation.
                    div()
                        .id("right-edge-hover-trigger")
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .right_0()
                        .w(px(28.0))
                        .cursor(CursorStyle::PointingHand)
                        .on_hover(cx.listener(|drawer, hovered: &bool, _window, cx| {
                            if *hovered && !drawer.is_hovered {
                                drawer.is_hovered = true;
                                cx.notify();
                            }
                        }))
                        // Subtle 2px edge indicator pill when closed
                        .child(
                            div()
                                .absolute()
                                .top(px(220.0))
                                .right_0()
                                .w(px(3.0))
                                .h(px(60.0))
                                .rounded_l(px(2.0))
                                .bg(rgba(c.primary))
                                .opacity((1.0 - self.open_progress * 2.0).clamp(0.0, 0.7))
                        )
                )
        }
    }

    pub fn main() {
        println!("[Caelestia-RS] Launching Edge-Attached Session Drawer on Wayland Layer-Shell...");
        application().run(|cx: &mut App| {
            cx.set_quit_mode(gpui::QuitMode::Explicit);
            println!("[Caelestia-RS] Initialized: Drawer is hidden on the right edge. Hover to reveal!");

            let options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: size(px(116.), px(500.)),
                })),
                app_id: Some("caelestia-session-drawer".to_string()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "caelestia-session".to_string(),
                    layer: Layer::Overlay,
                    anchor: Anchor::RIGHT,
                    margin: Some((px(0.), px(0.), px(0.), px(0.))), // 0px margin touches the right screen edge
                    exclusive_zone: None,
                    keyboard_interactivity: KeyboardInteractivity::OnDemand,
                    ..Default::default()
                }),
                ..Default::default()
            };

            let res = cx.open_window(options, |_, cx| {
                cx.new(SessionDrawer::new)
            });

            match res {
                Ok(handle) => println!("[Caelestia-RS] Window handle opened: {:?}", handle),
                Err(err) => eprintln!("[Caelestia-RS] Failed to open window: {:?}", err),
            }
        });
    }
}
