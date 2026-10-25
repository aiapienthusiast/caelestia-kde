//! Service Loader & Initialization Coordinator
//! Parity with `shell/modules/ServiceLoader.qml`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServicePhase {
    Immediate,
    Deferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellServiceId {
    ConfigMigrations,
    Notifications,
    IdleInhibitor,
    GameMode,
    Players,
    Brightness,
    Weather,
    WorkspaceTrackerGuard,
    PlasmaWidgets,
    TrayRegistry,
    Vpn,
}

#[derive(Debug, Clone, Default)]
pub struct ServiceLoader {
    initialized_services: Vec<ShellServiceId>,
}

impl ServiceLoader {
    pub fn new() -> Self {
        Self {
            initialized_services: Vec::new(),
        }
    }

    /// Immediate services loaded synchronously at startup.
    pub fn immediate_services() -> Vec<ShellServiceId> {
        vec![
            ShellServiceId::ConfigMigrations,
            ShellServiceId::Notifications,
        ]
    }

    /// Deferred services loaded after startup timer (250ms in QML).
    pub fn deferred_services(vpn_enabled: bool) -> Vec<ShellServiceId> {
        let mut services = vec![
            ShellServiceId::IdleInhibitor,
            ShellServiceId::GameMode,
            ShellServiceId::Players,
            ShellServiceId::Brightness,
            ShellServiceId::Weather,
            ShellServiceId::WorkspaceTrackerGuard,
            ShellServiceId::PlasmaWidgets,
            ShellServiceId::TrayRegistry,
        ];
        if vpn_enabled {
            services.push(ShellServiceId::Vpn);
        }
        services
    }

    pub fn mark_initialized(&mut self, service: ShellServiceId) {
        if !self.initialized_services.contains(&service) {
            self.initialized_services.push(service);
        }
    }

    pub fn is_initialized(&self, service: ShellServiceId) -> bool {
        self.initialized_services.contains(&service)
    }
}
