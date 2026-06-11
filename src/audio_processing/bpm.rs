//! Utility functions to track a "Beats per minute" value.
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bpm {
    pub base_value: usize,
}

impl Bpm {
    pub fn from_f32(value: f32) -> Self {
        Self {
            base_value: value.max(0.0) as usize,
        }
    }

    pub fn to_duration(&self) -> Duration {
        if self.base_value == 0 {
            return Duration::ZERO;
        }
        let secs_per_beat = 60.0 / (self.base_value as f32);
        Duration::from_secs_f32(secs_per_beat)
    }
}
