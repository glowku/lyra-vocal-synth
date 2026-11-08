//! Branched FDTD transmission-line airway model.
//!
//! This will become a full staggered-grid finite-difference time-domain
//! simulation of the vocal tract (oral + nasal + sinuses).

/// Placeholder for the future full FDTD network.
#[derive(Debug, Clone)]
pub struct Airway {
    /// Current area function (m²) along the tract (very coarse for now)
    pub areas: Vec<f32>,
    /// Number of sections
    pub n_sections: usize,
}

impl Airway {
    pub fn new(n_sections: usize) -> Self {
        let mut areas = vec![0.0005; n_sections]; // ~5 cm² default
        // Simple vowel-ish shape (placeholder)
        if n_sections > 4 {
            areas[n_sections / 3] = 0.0002;
            areas[2 * n_sections / 3] = 0.0010;
        }
        Self { areas, n_sections }
    }

    pub fn set_vowel_shape(&mut self, f1_approx: f32, f2_approx: f32) {
        // Extremely crude mapping for future development
        let _ = (f1_approx, f2_approx);
    }
}
