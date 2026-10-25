//! Bar Performance Meters
//! Parity with `shell/modules/bar/components/performance/`.

pub mod battery;
pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod network;
pub mod stat;
pub mod storage;

pub use battery::PerfBattery;
pub use cpu::PerfCpu;
pub use gpu::PerfGpu;
pub use memory::PerfMemory;
pub use network::PerfNetwork;
pub use stat::PerfStat;
pub use storage::PerfStorage;
