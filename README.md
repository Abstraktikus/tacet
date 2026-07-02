# Tacet

> **Status: v0.1.0.** Verified in a live GigPerformer 5 rig — loads as an
> instrument, accepts MIDI, and is silent. Early release; no installer yet.

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

## Related projects

- **[GigPerformer](https://www.gigperformer.com/)** — the host Tacet is built for.
- **[Kapellmeister](https://github.com/Abstraktikus/kapellmeister)** — on-stage
  companion display for a GigPerformer rig (chord sheets, song/part status, timeline).
- **[GigPerformer Global Rackspace script](https://github.com/Abstraktikus/GigPerformer)** —
  the OSC-streaming GPScript that feeds Kapellmeister.

## Build

Requires the Rust toolchain (MSVC). Build for the **host's architecture** — a VST3
must match the host it loads into. GigPerformer 5 on Windows is x64:

```bash
cargo xtask bundle tacet --release --target x86_64-pc-windows-msvc
# → target/bundled/Tacet.vst3 and Tacet.clap
```

On a native ARM64 host, use `--target aarch64-pc-windows-msvc` instead (or omit
`--target` to build for the native toolchain).

## Install (Windows)

A VST3 is a **bundle folder** (`Tacet.vst3`), not a single file, and needs **no
installer** — you just place the bundle where your host scans for plugins, then
rescan. Get `Tacet.vst3` from a
[release](https://github.com/Abstraktikus/tacet/releases) (unzip it) or by building
it (see above). The bundle ships both x64 and ARM64 slices; the host picks its own.

### Option A — no admin rights (recommended for GigPerformer)

You never touch `Program Files`, so no elevation is needed:

1. Put the `Tacet.vst3` folder anywhere you can write, e.g.
   `%USERPROFILE%\...\GigPerformer\Tacet\Tacet.vst3`.
2. In GigPerformer, add that folder as an extra VST3 scan path
   (*Options → General → VST plug-in paths / folders*), then **Rescan** plug-ins.
3. Tacet appears under vendor **Kapellmeister** as an Instrument/Synth.

### Option B — system-wide (needs admin)

```powershell
# elevated PowerShell (writing to Program Files needs admin)
.\deploy.ps1
```

Builds x64 and copies `Tacet.vst3` into `C:\Program Files\Common Files\VST3`, the
folder GigPerformer scans by default. Override with `-Target` / `-VstDir` for a
different architecture or scan folder.

## Test

```bash
cargo test
```
