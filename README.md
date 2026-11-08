# Lyra

**A next-generation physical-modeling musical TTS and vocal synthesizer.**

> Every sound is born from a simulated airway — lungs, vocal folds (or syrinx), branched vocal tract, lips and nose. No samples. No concatenative stitching. Pure physics, real-time, interactive, and musical.

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![WebAssembly](https://img.shields.io/badge/WASM-AudioWorklet-purple)](https://webassembly.org/)

**Live demo** (coming soon) · **Docs** · **Architecture**

---

## Vision

Lyra takes the best ideas of articulatory synthesis and pushes them further into a complete, powerful, interactive musical TTS instrument:

| Capability              | Description                                                                 |
|-------------------------|-----------------------------------------------------------------------------|
| **Realistic TTS**       | Full English + multi-language support with high-quality G2P, prosody, and coarticulation |
| **Musical Singing**     | Piano-roll editor, lyrics-to-gesture, expressive timing, vibrato, dynamics  |
| **Interactive Play**    | Real-time drag of tongue / lips / jaw / soft palate on a live anatomical model |
| **Beatbox & Percussion**| Every drum sound is an articulatory gesture (ejectives, clicks, bursts)    |
| **Bestiary**            | Human (male/female/child continuous morph) + songbird, mammal, reptile models |
| **Expressive Control**  | Emotion, style, breathiness, pressed voice, twang, growl, falsetto          |
| **Studio Features**     | Real-time spectrum, formant tracking, MIDI, multi-voice, effects chain, WAV export |
| **Beautiful UI**        | Dark cinematic design, 3D head, live pressure/airflow visualization, vector anatomy |

The engine is written in pure **Rust**, compiled to **WebAssembly** for the browser (AudioWorklet) and also runs natively for CLI, offline rendering and research.

---

## Architecture (high level)

```
┌─────────────────────────────────────────────────────────────┐
│  Frontend (TypeScript / React / Three.js / Canvas)          │
│  • Anatomical viewer (midsagittal + 3D)                     │
│  • Piano roll + lyrics                                      │
│  • Gestural score, consonant chart, spectrum                │
│  • MIDI, keyboard, pad controllers                          │
└──────────────────────┬──────────────────────────────────────┘
                       │ timestamped commands
┌──────────────────────▼──────────────────────────────────────┐
│  AudioWorklet  ←  Rust Engine (WASM)                        │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Voice Pool                                            │  │
│  │  • Gesture Scheduler → Articulator Dynamics           │  │
│  │  • Anatomy → Geometry → Area Functions                │  │
│  │  • Sources (glottis / syrinx / noise / clicks)        │  │
│  │  • Branched FDTD Transmission Line (airway)           │  │
│  │  • Radiation + close-mic chain                        │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘

Same engine used by:
• Browser (real-time)
• `lyra` CLI (offline high-quality renders, diagnostics, calibration)
• Future DAW plugins / native apps
```

### Core acoustic model

- **Branched staggered-grid FDTD** transmission line (handles variable lengths, DC pressure, nonlinear losses, moving walls, nasal tract, sinuses…)
- Self-oscillating or controlled **vocal folds** (one-mass + planned two-mass Ishizaka–Flanagan)
- **Syrinx** model for birds (two independent labia)
- Full articulatory gestures with coarticulation and prosody

---

## Project Structure

```
lyra-vocal-synth/
├── crates/
│   ├── lyra/              # Core engine (anatomy, sources, FDTD, gestures)
│   ├── lyra-cli/          # Offline renderer, diagnostics, calibration
│   └── lyra-wasm/         # WASM bindings for AudioWorklet
├── web/                   # Frontend (Vite + React + Three.js)
├── docs/                  # Architecture, design, research notes
├── tools/                 # Spectrogram, analysis scripts
├── LICENSE
└── README.md
```

---

## Roadmap

### Phase 0 – Foundation (current)
- [x] Repository & vision
- [ ] Cargo workspace + core crate skeleton
- [ ] Basic FDTD tube + simple glottis
- [ ] CLI that can render a sustained vowel
- [ ] Minimal web shell + AudioWorklet bridge

### Phase 1 – Human Voice
- [ ] Full articulatory model (tongue, lips, jaw, soft palate, larynx height)
- [ ] Male / female / child continuous morph
- [ ] English G2P + IPA → gesture score
- [ ] Real-time Play mode with live anatomy visualization

### Phase 2 – Musical & Expressive
- [ ] Piano-roll singing with lyrics
- [ ] Prosody, vibrato, dynamics, emotion parameters
- [ ] Beatbox / percussion gestures
- [ ] Multi-voice + simple effects

### Phase 3 – Power Features
- [ ] Bestiary (songbird, etc.)
- [ ] Advanced vocal fold models (two-mass, ventricular folds)
- [ ] Calibration pipeline against real recordings
- [ ] Studio-grade export & plugin targets

---

## Quick Start (when ready)

```bash
# Requirements: Rust (wasm32-unknown-unknown), Node/Bun

cargo build --release -p lyra-cli
cargo run --release -p lyra-cli -- render vowel a 440 --out a4.wav

cd web && bun install && bun run dev
```

---

## Design Principles

1. **Physics first** — every sound must be explainable by the airway model.
2. **Deterministic & real-time safe** — the engine has a strict contract for the AudioWorklet.
3. **Beautiful by default** — the UI is not an afterthought; it is part of the instrument.
4. **Research-friendly** — diagnostics, spectrograms, formant tracking, comparison tools.
5. **Open and ambitious** — MIT license, clear architecture docs, room to grow.

---

## Inspiration & Credits

Lyra stands on the shoulders of decades of articulatory synthesis research and is heavily inspired by the outstanding work of [Chimera Vocal Synth](https://github.com/jamesondh/chimera-vocal-synth) by Jameson Hodge — an exceptionally ambitious and beautiful project.

We aim to push the same physical-modeling philosophy even further in musical expressivity, multi-language support, studio integration and design polish.

---

## License

MIT © 2026 glowku / Lyra contributors

---

**Lyra** — *let the voice rise from the physics of air.*
