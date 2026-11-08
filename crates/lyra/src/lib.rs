//! Lyra — physical-modeling vocal synthesis engine.
//!
//! Every sound is produced by simulating airflow through a digital airway.

#![allow(dead_code)]

pub mod anatomy;
pub mod source;
pub mod airway;
pub mod gesture;

pub use anatomy::Body;
pub use source::Glottis;

/// A single voice instance.
#[derive(Debug, Clone)]
pub struct Voice {
    pub body: Body,
    pub glottis: Glottis,
    /// Current lung pressure (cmH₂O)
    pub lung_pressure: f32,
    /// Fundamental frequency target (Hz)
    pub f0: f32,
}

impl Voice {
    pub fn new_male() -> Self {
        Self {
            body: Body::male(),
            glottis: Glottis::default(),
            lung_pressure: 8.0,
            f0: 110.0,
        }
    }

    pub fn new_female() -> Self {
        Self {
            body: Body::female(),
            glottis: Glottis::default(),
            lung_pressure: 7.5,
            f0: 220.0,
        }
    }

    /// Process one block of audio (placeholder for now).
    pub fn process(&mut self, output: &mut [f32], sample_rate: f32) {
        // TODO: full FDTD + source + radiation
        let _ = (output, sample_rate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voice_creation() {
        let v = Voice::new_male();
        assert!((v.f0 - 110.0).abs() < 1e-3);
    }
}
