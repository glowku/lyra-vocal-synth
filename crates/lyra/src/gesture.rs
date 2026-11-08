//! Gesture scheduling and articulator dynamics.
//!
//! Turns phonemes / notes / score events into continuous articulator trajectories.

/// A single articulatory gesture target.
#[derive(Debug, Clone)]
pub struct Gesture {
    pub start_time: f32,
    pub duration: f32,
    pub articulator: Articulator,
    pub target: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Articulator {
    Jaw,
    TongueBody,
    TongueTip,
    Lips,
    SoftPalate,
    LarynxHeight,
    GlottalTension,
}

/// Simple scheduler that will later drive the anatomy → area function pipeline.
#[derive(Debug, Default)]
pub struct GestureScheduler {
    pub gestures: Vec<Gesture>,
}

impl GestureScheduler {
    pub fn add(&mut self, g: Gesture) {
        self.gestures.push(g);
    }

    pub fn clear(&mut self) {
        self.gestures.clear();
    }
}
