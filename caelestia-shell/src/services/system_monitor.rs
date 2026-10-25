//! System Monitor Service
//! Parity with `cpu.cpp`, `memory.cpp`, `diskinfo.cpp`, `networkusage.cpp`.
//! Collects real-time CPU, RAM, Swap, and Disk metrics with history buffers for sparklines.

use std::collections::VecDeque;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

pub const HISTORY_MAX_SAMPLES: usize = 30;

#[derive(Debug, Clone)]
pub struct SystemMonitorState {
    pub cpu_usage: f32,
    pub cpu_cores_usage: Vec<f32>,
    pub ram_used_bytes: u64,
    pub ram_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
    pub disk_used_bytes: u64,
    pub disk_total_bytes: u64,

    // Sparkline history buffers (0.0 to 1.0)
    pub cpu_history: VecDeque<f32>,
    pub ram_history: VecDeque<f32>,
}

impl Default for SystemMonitorState {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            cpu_cores_usage: Vec::new(),
            ram_used_bytes: 0,
            ram_total_bytes: 1,
            swap_used_bytes: 0,
            swap_total_bytes: 1,
            disk_used_bytes: 0,
            disk_total_bytes: 1,
            cpu_history: VecDeque::with_capacity(HISTORY_MAX_SAMPLES),
            ram_history: VecDeque::with_capacity(HISTORY_MAX_SAMPLES),
        }
    }
}

pub struct SystemMonitorService {
    system: System,
    disks: Disks,
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

        let mut service = Self {
            system,
            disks,
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

        // History
        let cpu_norm = (cpu_usage / 100.0).clamp(0.0, 1.0);
        let ram_norm = if ram_total > 0 {
            (ram_used as f32 / ram_total as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };

        if self.state.cpu_history.len() >= HISTORY_MAX_SAMPLES {
            self.state.cpu_history.pop_front();
        }
        self.state.cpu_history.push_back(cpu_norm);

        if self.state.ram_history.len() >= HISTORY_MAX_SAMPLES {
            self.state.ram_history.pop_front();
        }
        self.state.ram_history.push_back(ram_norm);

        self.state.cpu_usage = cpu_usage;
        self.state.cpu_cores_usage = cpu_cores;
        self.state.ram_used_bytes = ram_used;
        self.state.ram_total_bytes = ram_total;
        self.state.swap_used_bytes = swap_used;
        self.state.swap_total_bytes = swap_total;
        self.state.disk_used_bytes = disk_used;
        self.state.disk_total_bytes = disk_total;
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
