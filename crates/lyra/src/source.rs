//! Sound sources: glottis, noise, clicks, future syrinx.

/// Simple one-mass glottis model placeholder.
/// Will be replaced by a more sophisticated self-oscillating model.
#[derive(Debug, Clone)]
pub struct Glottis {
    /// Open quotient (0–1)
    pub open_quotient: f32,
    /// Spectral tilt / brightness control
    pub tilt: f32,
    /// Pressed vs breathy (0 = breathy, 1 = pressed)
    pub pressed: f32,
    /// Current phase of the glottal cycle
    phase: f32,
}

impl Default for Glottis {
    fn default() -> Self {
        Self {
            open_quotient: 0.6,
            tilt: 0.5,
            pressed: 0.5,
            phase: 0.0,
        }
    }
}

impl Glottis {
    /// Generate one sample of glottal flow derivative (very crude LF-model-like).
    pub fn tick(&mut self, f0: f32, sample_rate: f32) -> f32 {
        if f0 <= 0.0 || sample_rate <= 0.0 {
            return 0.0;
        }
        let phase_inc = f0 / sample_rate;
        self.phase = (self.phase + phase_inc) % 1.0;

        // Simple raised-cosine-ish pulse (placeholder)
        let t = self.phase;
        let oq = self.open_quotient.clamp(0.2, 0.9);
        if t < oq {
            let x = t / oq;
            // crude derivative of a glottal pulse
            let flow = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * x).cos());
            flow * (1.0 + self.pressed)
        } else {
            0.0
        }
    }
}
