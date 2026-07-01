# Tacet — Empty VST3/CLAP Synth Shell

**Date:** 2026-07-01
**Status:** Approved design, pending implementation plan

## Purpose & Success Criteria

Tacet is an instrument-plugin shell that GigPerformer recognizes as a synthesizer
and can load into synth slots, but that does **exactly nothing** (outputs silence).

**Why:** GigPerformer rackspaces often expose more synth plugin blocks than a given
song actually uses. Loading a real Arturia/Roland/etc. synth into an unused slot
wastes RAM and CPU. Dropping Tacet into those slots instead fills the block with a
near-zero-footprint placeholder while keeping the block's handle and connections
valid.

**Done when:**
- `Tacet.vst3` builds from the Rust source.
- It loads in GigPerformer 5 and appears as an **instrument/synth**.
- It accepts MIDI input without error and produces **silence**.
- Its memory/CPU footprint is measurably smaller than a real synth in the same slot.

## Stack

- **Language/framework:** Rust + [`nih-plug`](https://github.com/robbert-vdh/nih-plug).
- **Export formats:** **VST3** (the format GigPerformer loads on Windows) and **CLAP**
  (produced for free by the same nih-plug build; useful if GP adds CLAP support and
  gives first-class MIDI note ports for the future MIDI plugin).
- **Rationale:** Reuses the existing, working Rust/Cargo/MSVC toolchain already used
  by Kapellmeister. No Visual Studio C++ workload, CMake, or JUCE required — nothing
  to install or buy. JUCE/C++ and the raw Steinberg VST3 SDK were considered but
  rejected because they introduce a new toolchain for no benefit on a "does nothing"
  plugin.
- **Repo:** Standalone repository `C:\dev\tacet`, its own git history, published to
  GitHub under `Abstraktikus/tacet`. All code, UI strings, and GitHub content in
  English.

## Plugin Declaration (the core requirement)

What makes a VST3 register as a synth in a host:

1. **Subcategory `Instrument | Synth`** (`Vst3SubCategory::Instrument`,
   `Vst3SubCategory::Synth`) — this is what makes the host list Tacet as an
   instrument and allow it in a synth slot.
2. **MIDI/event input bus** (`MidiConfig::Basic`) — the plugin can receive notes
   (and immediately discards them).
3. **Audio output bus** — one **stereo** output, **no** audio input.
4. **Zero parameters, no editor window** — for the minimal footprint that is the
   whole point of the project.
5. **Unique VST3 class ID and CLAP ID** — generated once and kept stable.

## Behavior

`process()`:
- Writes silence to the output buffer (or leaves it cleared).
- Discards all incoming MIDI events.
- Holds no state and performs **no allocation on the audio thread**.

## Project Structure & Build/Deploy

```
tacet/
├─ Cargo.toml            # nih-plug dependency, crate-type = ["cdylib"]
├─ src/lib.rs            # plugin implementation (~80 lines)
├─ xtask/                # nih-plug bundler (cargo xtask bundle)
├─ docs/
├─ README.md
└─ .gitignore
```

- **Build/bundle:** `cargo xtask bundle tacet --release` → produces `Tacet.vst3`
  (and `Tacet.clap`).
- **Deploy target:** `C:\Program Files\Common Files\VST3\` (the standard system VST3
  folder GigPerformer scans). A small deploy script mirrors the Kapellmeister
  `deploy:test` reflex. Note: writing to `Program Files` needs elevation; the user
  VST3 folder (`%LOCALAPPDATA%\Programs\Common\VST3`) is a non-elevated fallback if
  GP is configured to scan it.

## Testing

- **Unit test:** `process()` is guaranteed to output silence; bus layout and
  Instrument/Synth category metadata are correct.
- **VST3 validator:** Steinberg's `validator` (nih-plug can invoke it) must pass.
- **Manual (user):** load in GigPerformer 5, confirm it appears as an instrument,
  accepts MIDI, is silent, and check the footprint.

## Explicitly Out of Scope (YAGNI)

- No multi-out / configurable channel count.
- No parameters, no UI, no sound.
- No MIDI processing or monitoring.

The planned **MIDI Processing & Monitoring** plugin (a Rust reimplementation of logic
currently in `Global Rackspace.gpscript`) is a **separate future project** that will
reuse this same nih-plug scaffold (bundling, instrument declaration, MIDI bus, and
the optional editor window via `nih_plug_egui`/`nih_plug_vizia`). Building Tacet first
establishes that scaffold as a minimal "hello world".

## Notes / Open Items for Implementation

- Generate and record the stable VST3 class ID and CLAP ID during scaffolding.
- Decide the exact silence strategy (explicit zero-fill vs. relying on host-cleared
  buffers) — zero-fill is the safe default.
- Confirm GigPerformer 5's configured VST3 scan folder(s) on this machine so deploy
  targets the right path.
