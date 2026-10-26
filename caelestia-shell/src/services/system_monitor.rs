use std::collections::VecDeque;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};

pub const HISTORY_MAX_SAMPLES: usize = 30;

#[derive(Debug, Clone, Default)]
pub struct PrivacyState {
    pub camera_active: bool,
    pub microphone_active: bool,
    pub screenshare_active: bool,
}

impl PrivacyState {
    pub fn in_use(&self) -> bool {
        self.camera_active || self.microphone_active || self.screenshare_active
    }
}

#[derive(Debug, Clone, Default)]
pub struct AccessibilityState {
    pub high_contrast: bool,
    pub large_text: bool,
    pub screen_reader: bool,
}

#[derive(Debug, Clone)]
pub struct SystemMonitorState {
    pub cpu_usage: f32,
    pub cpu_cores_usage: Vec<f32>,
    pub cpu_temp: f32,
    pub gpu_usage: f32,
    pub gpu_vram_used: u64,
    pub gpu_vram_total: u64,
    pub gpu_temp: f32,
    pub ram_used_bytes: u64,
    pub ram_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
    pub disk_used_bytes: u64,
    pub disk_total_bytes: u64,
    pub network_rx_bytes_sec: u64,
    pub network_tx_bytes_sec: u64,

    // Sparkline history buffers (0.0 to 1.0)
    pub cpu_history: VecDeque<f32>,
    pub ram_history: VecDeque<f32>,
    pub gpu_history: VecDeque<f32>,

    pub privacy: PrivacyState,
    pub accessibility: AccessibilityState,
}

impl Default for SystemMonitorState {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            cpu_cores_usage: Vec::new(),
            cpu_temp: 45.0,
            gpu_usage: 0.0,
            gpu_vram_used: 0,
            gpu_vram_total: 1024 * 1024 * 1024 * 8, // 8 GB default
            gpu_temp: 42.0,
            ram_used_bytes: 0,
            ram_total_bytes: 1,
            swap_used_bytes: 0,
            swap_total_bytes: 1,
            disk_used_bytes: 0,
            disk_total_bytes: 1,
            network_rx_bytes_sec: 0,
            network_tx_bytes_sec: 0,
            cpu_history: VecDeque::with_capacity(HISTORY_MAX_SAMPLES),
            ram_history: VecDeque::with_capacity(HISTORY_MAX_SAMPLES),
            gpu_history: VecDeque::with_capacity(HISTORY_MAX_SAMPLES),
            privacy: PrivacyState::default(),
            accessibility: AccessibilityState::default(),
        }
    }
}

pub struct SystemMonitorService {
    system: System,
    disks: Disks,
    networks: Networks,
    pub state: SystemMonitorState,
}

impl Default for SystemMonitorService {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemMonitorService {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();

        let mut service = Self {
            system,
            disks,
            networks,
            state: SystemMonitorState::default(),
        };
        service.refresh();
        service
    }

    /// Refresh all system hardware metrics
    pub fn refresh(&mut self) {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.disks.refresh();
        self.networks.refresh();

        // CPU
        let cpu_usage = self.system.global_cpu_usage();
        let cpu_cores: Vec<f32> = self.system.cpus().iter().map(|c| c.cpu_usage()).collect();

        // Memory
        let ram_used = self.system.used_memory();
        let ram_total = self.system.total_memory();
        let swap_used = self.system.used_swap();
        let swap_total = self.system.total_swap();

        // Disks
        let mut disk_used = 0u64;
        let mut disk_total = 0u64;
        for disk in self.disks.list() {
            let total = disk.total_space();
            let available = disk.available_space();
            disk_total += total;
            disk_used += total.saturating_sub(available);
        }

        // Network rates
        let mut rx_total = 0u64;
        let mut tx_total = 0u64;
        for (_interface, data) in &self.networks {
            rx_total += data.received();
            tx_total += data.transmitted();
        }

        // History
        let cpu_norm = (cpu_usage / 100.0).clamp(0.0, 1.0);
        let ram_norm = if ram_total > 0 {
            (ram_used as f32 / ram_total as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let gpu_norm = (self.state.gpu_usage / 100.0).clamp(0.0, 1.0);

        if self.state.cpu_history.len() >= HISTORY_MAX_SAMPLES {
            self.state.cpu_history.pop_front();
        }
        self.state.cpu_history.push_back(cpu_norm);

        if self.state.ram_history.len() >= HISTORY_MAX_SAMPLES {
            self.state.ram_history.pop_front();
        }
        self.state.ram_history.push_back(ram_norm);

        if self.state.gpu_history.len() >= HISTORY_MAX_SAMPLES {
            self.state.gpu_history.pop_front();
        }
        self.state.gpu_history.push_back(gpu_norm);

        self.state.cpu_usage = cpu_usage;
        self.state.cpu_cores_usage = cpu_cores;
        self.state.ram_used_bytes = ram_used;
        self.state.ram_total_bytes = ram_total;
        self.state.swap_used_bytes = swap_used;
        self.state.swap_total_bytes = swap_total;
        self.state.disk_used_bytes = disk_used;
        self.state.disk_total_bytes = disk_total;
        self.state.network_rx_bytes_sec = rx_total;
        self.state.network_tx_bytes_sec = tx_total;
    }

    /// RAM used in GiB formatted
    pub fn ram_used_gib(&self) -> f64 {
        self.state.ram_used_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    /// RAM total in GiB formatted
    pub fn ram_total_gib(&self) -> f64 {
        self.state.ram_total_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    /// Disk used percentage (0 to 100)
    pub fn disk_percent(&self) -> f32 {
        if self.state.disk_total_bytes > 0 {
            (self.state.disk_used_bytes as f32 / self.state.disk_total_bytes as f32) * 100.0
        } else {
            0.0
        }
    }
}
