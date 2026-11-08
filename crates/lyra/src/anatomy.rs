//! Anatomical body parameters and continuous morphing.

/// Continuous body parameters (can morph between male / female / child).
#[derive(Debug, Clone, Copy)]
pub struct Body {
    /// Overall scale factor (1.0 = adult male reference)
    pub scale: f32,
    /// Vocal tract length factor
    pub tract_length: f32,
    /// Glottal size / mass factor
    pub glottal_scale: f32,
    /// Formant shift tendency (higher = brighter)
    pub brightness: f32,
}

impl Body {
    pub fn male() -> Self {
        Self {
            scale: 1.0,
            tract_length: 1.0,
            glottal_scale: 1.0,
            brightness: 0.45,
        }
    }

    pub fn female() -> Self {
        Self {
            scale: 0.89,
            tract_length: 0.85,
            glottal_scale: 0.75,
            brightness: 0.65,
        }
    }

    pub fn child() -> Self {
        Self {
            scale: 0.72,
            tract_length: 0.70,
            glottal_scale: 0.55,
            brightness: 0.80,
        }
    }

    /// Linear morph between two bodies (t in [0, 1]).
    pub fn morph(a: &Self, b: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            scale: a.scale + (b.scale - a.scale) * t,
            tract_length: a.tract_length + (b.tract_length - a.tract_length) * t,
            glottal_scale: a.glottal_scale + (b.glottal_scale - a.glottal_scale) * t,
            brightness: a.brightness + (b.brightness - a.brightness) * t,
        }
    }
}
