# Architecture — Lyra Vocal Synth

## Design goals

1. **Physical fidelity first** — every sample is the result of an airway simulation.
2. **Real-time deterministic** — the engine must be safe for AudioWorklet (no allocations in the process callback, fixed control rate).
3. **Same code everywhere** — browser, CLI, future plugins share the exact same Rust core.
4. **Musical & expressive** — not just TTS; a true instrument.

## High-level data flow

```
Score / Text / MIDI / Direct articulator control
        ↓
Gesture Scheduler  →  Articulator Dynamics (control rate ~ 100–400 Hz)
        ↓
Anatomy → Geometry → Area Functions (oral + nasal)
        ↓
Sources (glottis / syrinx / turbulence / clicks)
        ↓
Branched FDTD Transmission Line (sample rate)
        ↓
Radiation + close-mic / effects chain
        ↓
Audio output
```

## Planned modules

| Module       | Responsibility                                      |
|--------------|-----------------------------------------------------|
| `anatomy`    | Body morph, articulator positions → geometry        |
| `source`     | Glottis (1-mass → 2-mass), syrinx, noise, clicks    |
| `airway`     | FDTD network, losses, branching, moving walls       |
| `gesture`    | Phoneme/score → continuous trajectories + prosody   |
| `prosody`    | Pitch, duration, intensity, emotion curves          |
| `g2p`        | Text → IPA / phonemes (English first, then more)    |

## Control rate vs audio rate

- Control rate: every 64–128 samples. Area functions and source parameters are interpolated per sample.
- All heavy geometry work stays at control rate.

## Real-time contract

- No heap allocation inside `process()`.
- Bounded CPU (worst-case measured).
- Deterministic given the same commands and seed.

## Future extensions

- Two-mass vocal folds + ventricular folds
- Full nasal tract + sinuses + piriform fossae
- Creature models (songbird dual syrinx, etc.)
- In-engine EQ / reverb / limiter for studio use
