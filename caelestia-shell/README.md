# Caelestia Shell (Rust)

GPU-accelerated desktop shell for Caelestia KDE on Wayland, built with GPUI, layer-shell, native KWin frosted blur, and Material 3 design tokens.

---

## Highlights

- **Wayland Layer-Shell Native**: Anchored to display edge using `zwlr_layer_shell_v1` on `Overlay` layer.
- **Native KWin Plasma 6 Blur**: Uses `ext_background_effect_manager_v1` to request compositor-level dual-filter blur behind translucent surfaces with zero overhead.
- **Material 3 Dynamic Theme Engine**: Integrates directly with `~/.local/state/caelestia/scheme.json` and `~/.config/caelestia/shell.json`, supporting hot-reloading on wallpaper changes.
- **Soft-Body Spring Physics**: Organic leading-edge curvature and velocity deformation implementing Caelestia Blobs dynamics.
- **Zero Heavy Dependencies**: Self-contained minimal GPUI engine under `engine/crates/` without deadweight editor crates. Rebuilds in ~1 second.

---

## Architecture Overview

```
caelestia-shell/
├── Cargo.toml                  # Workspace manifest and package configuration
├── rustfmt.toml                # Formatting rules
├── .cargo/config.toml          # Build flags and link optimizations
├── engine/crates/              # Minimal, isolated GPUI engine
│   ├── gpui/                   # Core GPU rendering, event dispatcher & elements
│   ├── gpui_linux/             # Wayland layer-shell & ext_background_effect_manager_v1
│   ├── gpui_wgpu/              # WGPU hardware rasterization backend
│   └── ...                     # Core utility crates
└── src/
    ├── main.rs                 # CLI entry point, Wayland app initialization
    ├── domain/                 # Domain constants and action models
    │   ├── tokens.rs           # M3 rounding, spacing, padding, drawer dimensions
    │   └── session.rs          # Session actions (Logout, Shutdown, Hibernate, Reboot)
    ├── infrastructure/         # External systems & appearance
    │   ├── config.rs           # Caelestia shell configuration loader
    │   └── theme.rs            # M3 palette parser with perceptual luminance scaling
    ├── services/               # IPC and system services
    │   └── session.rs          # D-Bus (org.kde.Shutdown) and systemctl command runner
    └── modules/                # Desktop shell visual modules
        └── session/            # Session drawer module
            ├── drawer.rs       # GPUI component, hover trigger, and blur coordinator
            └── physics.rs      # Soft-body spring physics & curvature geometry
```

---

## Building and Running

### Prerequisites

- Rust toolchain (`cargo`, `rustc` 2024 edition)
- Wayland compositor (KDE Plasma 6 KWin Wayland recommended)
- Standard development libraries (`wayland-client`, `vulkan-loader`, `libxkbcommon`)

### Build

```bash
# Debug build
cargo build --package caelestia-shell

# Optimized release build
cargo build --release --package caelestia-shell
```

### Run

```bash
# Launch daemon (idle retracted at screen edge)
./target/debug/caelestia-shell

# Launch with drawer forced open for visual debugging
DRAWER_OPEN=1 ./target/debug/caelestia-shell
```

---

## Interaction & Controls

| Input | Action |
|---|---|
| **Mouse Hover (Right Edge)** | Reveal session drawer with soft-body spring animation |
| **Mouse Exit** | Retract drawer back into edge indicator |
| **Tab / Down / j / n** | Focus next session action |
| **Shift+Tab / Up / k / p** | Focus previous session action |
| **Enter / Space / Click** | Execute focused session action |
| **Escape** | Dismiss / retract drawer |
