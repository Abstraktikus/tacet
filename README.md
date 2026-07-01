# Tacet

An empty VST3/CLAP **instrument** that does exactly nothing: it declares itself as
a synthesizer, accepts MIDI input, and outputs silence. Its purpose is to fill
unused synth slots in GigPerformer (and other hosts) with a near-zero-footprint
placeholder instead of loading a full synth, saving RAM and CPU while keeping the
plugin block's handle and connections valid.

- **Vendor:** Kapellmeister
- **Formats:** VST3 (primary), CLAP
- **I/O:** 1 stereo output, no audio input, MIDI input (discarded)
- **Parameters / UI:** none

## Build

Requires the Rust toolchain (MSVC).

```bash
cargo xtask bundle tacet --release
# → target/bundled/Tacet.vst3 and Tacet.clap
```

## Deploy (Windows)

```powershell
# elevated PowerShell
.\deploy.ps1
```

Copies `Tacet.vst3` into `C:\Program Files\Common Files\VST3`. Edit `$VstDir` in
`deploy.ps1` for a different scan folder.

## Test

```bash
cargo test
```
