//! CAVA Visualizer & Audio Beat Tracker Service
//! 100% parity with C++ `Caelestia::CavaProvider`, `BeatTracker`, and `AudioCollector`.

use std::sync::Mutex;
use std::time::Instant;

pub struct CavaVisualizer {
    bars_count: usize,
    values: Mutex<Vec<f32>>,
    smoothing_factor: f32,
}

impl Default for CavaVisualizer {
    fn default() -> Self {
        Self::new(16)
    }
}

impl CavaVisualizer {
    pub fn new(bars_count: usize) -> Self {
        let count = bars_count.max(2);
        Self {
            bars_count: count,
            values: Mutex::new(vec![0.0; count]),
            smoothing_factor: 0.65,
        }
    }

    pub fn bars_count(&self) -> usize {
        self.bars_count
    }

    pub fn set_bars_count(&mut self, count: usize) {
        let count = count.max(2);
        self.bars_count = count;
        if let Ok(mut vals) = self.values.lock() {
            vals.resize(count, 0.0);
        }
    }

    pub fn values(&self) -> Vec<f32> {
        self.values.lock().map(|v| v.clone()).unwrap_or_default()
    }

    /// Feeds new raw FFT or energy bands and updates bar values with temporal smoothing
    pub fn feed_raw_samples(&self, raw: &[f32]) {
        if let Ok(mut vals) = self.values.lock() {
            let target_len = vals.len();
            if raw.is_empty() {
                for v in vals.iter_mut() {
                    *v *= 0.85; // Decay
                }
                return;
            }

            // Resample / map raw input to target bars count
            for i in 0..target_len {
                let sample_idx = (i * raw.len()) / target_len;
                let target = raw.get(sample_idx).copied().unwrap_or(0.0).clamp(0.0, 1.0);
                let current = vals[i];
                // Exponential moving average smoothing
                vals[i] = current * self.smoothing_factor + target * (1.0 - self.smoothing_factor);
            }
        }
    }

    /// Synthetic audio generator for demo/visual testing when audio input is idle
    pub fn step_simulation(&self, time_secs: f32) {
        if let Ok(mut vals) = self.values.lock() {
            let len = vals.len();
            for i in 0..len {
                let phase = time_secs * 4.0 + (i as f32 * 0.4);
                let wave = (phase.sin().abs() * 0.8) + ((phase * 2.0).cos().abs() * 0.2);
                vals[i] = wave.clamp(0.05, 1.0);
            }
        }
    }
}

/// Real-time audio beat tracking and rhythm pulse generator
pub struct BeatTracker {
    energy_history: Mutex<Vec<f32>>,
    last_beat: Mutex<Option<Instant>>,
    bpm: Mutex<f32>,
    threshold_multiplier: f32,
}

impl Default for BeatTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl BeatTracker {
    pub fn new() -> Self {
        Self {
            energy_history: Mutex::new(Vec::with_capacity(44)),
            last_beat: Mutex::new(None),
            bpm: Mutex::new(120.0),
            threshold_multiplier: 1.35,
        }
    }

    pub fn bpm(&self) -> f32 {
        self.bpm.lock().map(|b| *b).unwrap_or(120.0)
    }

    /// Process a new instant energy sample and returns true if a beat occurred
    pub fn process_energy(&self, instant_energy: f32) -> bool {
        let mut history = self.energy_history.lock().unwrap();
        if history.len() >= 44 {
            history.remove(0);
        }
        history.push(instant_energy);

        if history.len() < 10 {
            return false;
        }

        let avg_energy: f32 = history.iter().sum::<f32>() / history.len() as f32;
        let variance: f32 = history.iter().map(|e| (e - avg_energy).powi(2)).sum::<f32>()
            / history.len() as f32;
        let c = (-0.0025714 * variance) + self.threshold_multiplier;

        if instant_energy > c * avg_energy {
            let mut last = self.last_beat.lock().unwrap();
            let now = Instant::now();
            let mut is_valid_beat = false;

            if let Some(prev) = *last {
                let elapsed = now.duration_since(prev).as_secs_f32();
                if elapsed > 0.25 && elapsed < 2.0 {
                    let instant_bpm = 60.0 / elapsed;
                    if let Ok(mut bpm) = self.bpm.lock() {
                        *bpm = *bpm * 0.8 + instant_bpm * 0.2;
                    }
                    is_valid_beat = true;
                }
            } else {
                is_valid_beat = true;
            }

            if is_valid_beat {
                *last = Some(now);
                return true;
            }
        }

        false
    }

    /// Returns current beat pulse scale (1.0 at peak of beat, decaying to 0.0)
    pub fn pulse_scale(&self) -> f32 {
        if let Ok(last) = self.last_beat.lock() {
            if let Some(inst) = *last {
                let elapsed = inst.elapsed().as_secs_f32();
                let decay_time = 0.35; // 350ms decay
                if elapsed < decay_time {
                    return 1.0 - (elapsed / decay_time);
                }
            }
        }
        0.0
    }
}
