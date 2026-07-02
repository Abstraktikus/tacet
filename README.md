# Tacet

> **Status: pre-alpha.** It builds, loads, and is silent, but has not yet been
> validated in a live GigPerformer rig.

An empty VST3/CLAP **instrument** that does exactly nothing: it declares itself as
a synthesizer, accepts MIDI input, and outputs silence. Its purpose is to fill
unused synth slots in GigPerformer (and other hosts) with a near-zero-footprint
placeholder instead of loading a full synth, saving RAM and CPU while keeping the
plugin block's handle and connections valid.

- **Vendor:** Kapellmeister
- **Formats:** VST3 (primary), CLAP
- **I/O:** 1 stereo output, no audio input, MIDI input (discarded)
- **Parameters / UI:** none

## Why this exists

[GigPerformer](https://www.gigperformer.com/) rackspaces often expose more synth
plugin blocks than a given song actually uses. Loading a real (Arturia, Roland, …)
synth into an unused slot wastes RAM and CPU. Dropping **Tacet** into those slots
instead keeps the block's handle and connections valid at a near-zero footprint.

Tacet is part of the **Kapellmeister** toolset — a companion controller for
GigPerformer — and shares its `Kapellmeister` vendor brand with the planned future
MIDI-processing plugin, which will reuse this same [`nih-plug`](https://github.com/robbert-vdh/nih-plug)
scaffold.

## Build

Requires the Rust toolchain (MSVC). Build for the **host's architecture** — a VST3
must match the host it loads into. GigPerformer 5 on Windows is x64:

```bash
cargo xtask bundle tacet --release --target x86_64-pc-windows-msvc
# → target/bundled/Tacet.vst3 and Tacet.clap
```

On a native ARM64 host, use `--target aarch64-pc-windows-msvc` instead (or omit
`--target` to build for the native toolchain).

## Deploy (Windows)

```powershell
# elevated PowerShell (writing to Program Files needs admin)
.\deploy.ps1
```

Builds x64 and copies `Tacet.vst3` into `C:\Program Files\Common Files\VST3` (the
folder GigPerformer scans). Override with `-Target` / `-VstDir` for a different
architecture or a user-writable scan folder.

## Test

```bash
cargo test
```
