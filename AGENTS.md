# AGENTS.md — Working on Lyra

## Project goal

Build the most ambitious, beautiful and musically powerful physical-modeling vocal synthesizer / musical TTS possible, in pure Rust + modern web.

## Key rules

1. **Physics first** — every new sound feature must ultimately come from the airway model.
2. **Determinism & real-time safety** — the audio process callback must remain allocation-free and bounded.
3. **Same engine everywhere** — browser WASM, CLI and future plugins share the `lyra` crate.
4. **Document the why** — important design decisions go into `docs/`.
5. **Beautiful UI is part of the product** — treat the frontend with the same care as the engine.

## Current phase

Phase 0 — Foundation:
- Solid Cargo workspace
- Minimal working FDTD + simple glottis that can render a sustained vowel offline
- Basic AudioWorklet bridge + dark shell UI

## Useful commands

```bash
cargo test --workspace
cargo run -p lyra-cli -- info
cargo run -p lyra-cli -- render vowel a 220 --out a.wav
```

## Inspiration

The original Chimera project by Jameson Hodge is an exceptional reference for ambition, architecture and visual quality. We build on that spirit while aiming for even stronger musical and TTS capabilities.
