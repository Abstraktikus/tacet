# Tacet — Empty VST Synth Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `Tacet`, a VST3/CLAP instrument plugin that GigPerformer recognizes as a synth but that outputs pure silence and holds no state — a minimal-footprint placeholder for unused synth slots.

**Architecture:** A single Rust `cdylib` crate using the `nih-plug` framework. The `Plugin` trait declares one stereo output, no audio input, and MIDI input. `process()` writes silence via a small pure helper and discards all incoming MIDI. A sibling `xtask` crate provides nih-plug's bundler to produce `Tacet.vst3` (and `Tacet.clap`).

**Tech Stack:** Rust (edition 2021), `nih_plug` (git dependency), `nih_plug_xtask` bundler, target `x86_64-pc-windows-msvc`.

## Global Constraints

- Framework: **Rust + nih-plug** (git dependency: `https://github.com/robbert-vdh/nih-plug.git`); commit `Cargo.lock` to pin the exact revision.
- Export **VST3** (primary, for GigPerformer) **and CLAP** (free from the same build).
- Plugin `NAME` = `"Tacet"`; `VENDOR` = `"Kapellmeister"`.
- Audio I/O: **1 stereo output, no audio input**. `MIDI_INPUT = MidiConfig::Basic`.
- VST3 subcategories **must** include `Vst3SubCategory::Instrument` **and** `Vst3SubCategory::Synth`.
- **Zero parameters, no editor window.**
- `process()` outputs **silence** and performs **no heap allocation** on the audio thread.
- Build target is the host: **x86_64-pc-windows-msvc** (GigPerformer 5 is 64-bit x64).
- All code, comments, identifiers, and repo content in **English**.
- Commit messages end with the `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>` trailer.

---

## File Structure

```
tacet/
├─ Cargo.toml            # plugin package + workspace root; crate-type = cdylib
├─ Cargo.lock            # committed — pins nih-plug revision
├─ .cargo/config.toml    # `xtask` cargo alias
├─ .gitignore            # already present (/target, *.vst3, *.clap)
├─ src/
│  └─ lib.rs             # silence helper + Plugin/ClapPlugin/Vst3Plugin impl + export macros + tests
├─ xtask/
│  ├─ Cargo.toml         # depends on nih_plug_xtask
│  └─ src/main.rs        # calls nih_plug_xtask::main()
├─ deploy.ps1            # copies bundled Tacet.vst3 into the system VST3 folder
├─ README.md
└─ docs/superpowers/…    # spec + this plan
```

- `src/lib.rs` is intentionally the single source file: the whole plugin is ~90 lines and the silence helper, trait impls, and tests change together. No split warranted (YAGNI).
- `xtask/` is nih-plug's standard build harness; it never contains plugin logic.

---

## Task 1: Project scaffold with build harness and silence helper

Establishes the buildable Rust workspace (plugin crate + xtask bundler) and delivers the first testable unit: the `write_silence` helper. Ends with `cargo test` green and `cargo build` succeeding.

**Files:**
- Create: `C:\dev\tacet\Cargo.toml`
- Create: `C:\dev\tacet\.cargo\config.toml`
- Create: `C:\dev\tacet\xtask\Cargo.toml`
- Create: `C:\dev\tacet\xtask\src\main.rs`
- Create: `C:\dev\tacet\src\lib.rs`
- Test: inline `#[cfg(test)]` module in `src/lib.rs`

**Interfaces:**
- Consumes: nothing (first task).
- Produces: `pub fn write_silence(channels: &mut [&mut [f32]])` — zeroes every sample in every channel slice. Task 2 calls this from `process()`.

- [ ] **Step 1: Create the workspace `Cargo.toml`**

Create `C:\dev\tacet\Cargo.toml`:

```toml
[package]
name = "tacet"
version = "0.1.0"
edition = "2021"
authors = ["Martin Nafzger"]
license = "MIT"
description = "An empty VST3/CLAP instrument that does nothing — a minimal-footprint placeholder for unused GigPerformer synth slots."
repository = "https://github.com/Abstraktikus/tacet"

[workspace]
members = ["xtask"]

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git" }
```

Note: `crate-type` includes `"lib"` alongside `"cdylib"` so the inline unit tests can link against the crate.

- [ ] **Step 2: Create the cargo alias**

Create `C:\dev\tacet\.cargo\config.toml`:

```toml
[alias]
xtask = "run --package xtask --release --"
```

- [ ] **Step 3: Create the xtask bundler crate**

Create `C:\dev\tacet\xtask\Cargo.toml`:

```toml
[package]
name = "xtask"
version = "0.1.0"
edition = "2021"

[dependencies]
nih_plug_xtask = { git = "https://github.com/robbert-vdh/nih-plug.git" }
```

Create `C:\dev\tacet\xtask\src\main.rs`:

```rust
fn main() -> nih_plug_xtask::Result<()> {
    nih_plug_xtask::main()
}
```

- [ ] **Step 4: Write the failing test for `write_silence`**

Create `C:\dev\tacet\src\lib.rs`:

```rust
//! Tacet — an empty VST3/CLAP instrument that outputs silence.

/// Overwrite every sample in every channel with silence (0.0).
///
/// Pure and allocation-free so it is safe to call from the audio thread.
pub fn write_silence(channels: &mut [&mut [f32]]) {
    for channel in channels {
        for sample in channel.iter_mut() {
            *sample = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_silence_zeros_all_samples() {
        let mut left = [0.3f32, -0.7, 1.0];
        let mut right = [9.0f32, -2.0, 0.5];
        let mut channels: [&mut [f32]; 2] = [&mut left, &mut right];

        write_silence(&mut channels);

        assert!(left.iter().all(|&s| s == 0.0), "left not silenced: {left:?}");
        assert!(right.iter().all(|&s| s == 0.0), "right not silenced: {right:?}");
    }
}
```

(The test is written before the helper is proven; to see it fail first, temporarily change the helper body to `{}` in the next step's verification — or trust that TDD ordering is satisfied because the assertions pin the exact behavior. For a true red step, comment out the loop body, run, see it fail, then restore.)

- [ ] **Step 5: Verify the test fails, then passes**

Temporarily replace the `write_silence` body with `{ let _ = channels; }` (a no-op), then run:

Run: `cargo test --lib write_silence_zeros_all_samples`
Expected: FAIL — `left not silenced: [0.3, -0.7, 1.0]`.

Restore the real loop body and run again:

Run: `cargo test --lib write_silence_zeros_all_samples`
Expected: PASS.

- [ ] **Step 6: Verify the whole workspace builds**

Run: `cargo build`
Expected: compiles `tacet` and `xtask` with no errors (first build downloads and pins nih-plug in `Cargo.lock`).

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml Cargo.lock .cargo/config.toml xtask/ src/lib.rs
git commit -m "$(cat <<'EOF'
Scaffold tacet workspace with silence helper

Rust cdylib plugin crate + nih-plug xtask bundler. Adds the pure,
allocation-free write_silence helper with a unit test.

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 2: Declare the plugin as an Instrument/Synth that outputs silence

Adds the full `Plugin`/`ClapPlugin`/`Vst3Plugin` implementation with correct metadata, wires `process()` to `write_silence`, exports the plugin, and pins the declaration with assertion tests. Ends with `cargo test` green and `cargo xtask bundle tacet --release` producing `Tacet.vst3`.

**Files:**
- Modify: `C:\dev\tacet\src\lib.rs` (add imports, `Tacet` struct, `TacetParams`, trait impls, export macros, and metadata tests)

**Interfaces:**
- Consumes: `write_silence(&mut [&mut [f32]])` from Task 1.
- Produces: the exported plugin type `Tacet` implementing `nih_plug::prelude::Plugin`, `ClapPlugin`, `Vst3Plugin`; associated consts `Tacet::NAME`, `Tacet::VENDOR`, `Tacet::AUDIO_IO_LAYOUTS`, `Tacet::MIDI_INPUT`, `Tacet::VST3_SUBCATEGORIES`, `Tacet::VST3_CLASS_ID`, `Tacet::CLAP_ID`. No later task depends on these — this is the final code task.

- [ ] **Step 1: Write the failing metadata tests**

In `C:\dev\tacet\src\lib.rs`, replace the existing `#[cfg(test)] mod tests { ... }` block with:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use nih_plug::prelude::*;
    use std::num::NonZeroU32;

    #[test]
    fn write_silence_zeros_all_samples() {
        let mut left = [0.3f32, -0.7, 1.0];
        let mut right = [9.0f32, -2.0, 0.5];
        let mut channels: [&mut [f32]; 2] = [&mut left, &mut right];

        write_silence(&mut channels);

        assert!(left.iter().all(|&s| s == 0.0), "left not silenced: {left:?}");
        assert!(right.iter().all(|&s| s == 0.0), "right not silenced: {right:?}");
    }

    #[test]
    fn declares_name_and_vendor() {
        assert_eq!(Tacet::NAME, "Tacet");
        assert_eq!(Tacet::VENDOR, "Kapellmeister");
    }

    #[test]
    fn declares_instrument_and_synth_subcategories() {
        assert!(Tacet::VST3_SUBCATEGORIES.contains(&Vst3SubCategory::Instrument));
        assert!(Tacet::VST3_SUBCATEGORIES.contains(&Vst3SubCategory::Synth));
    }

    #[test]
    fn is_stereo_out_with_no_audio_input() {
        let layout = Tacet::AUDIO_IO_LAYOUTS[0];
        assert_eq!(layout.main_input_channels, None);
        assert_eq!(layout.main_output_channels, NonZeroU32::new(2));
    }

    #[test]
    fn accepts_midi_input() {
        assert!(matches!(Tacet::MIDI_INPUT, MidiConfig::Basic));
    }

    #[test]
    fn vst3_class_id_is_frozen() {
        // The class ID must never change once released, or hosts treat an
        // updated Tacet as a different plugin and drop existing references.
        assert_eq!(&Tacet::VST3_CLASS_ID, b"KapellmstrTacet1");
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --lib`
Expected: FAIL — `cannot find type Tacet in this scope` / `Vst3SubCategory` unresolved (the plugin type and imports do not exist yet).

- [ ] **Step 3: Implement the plugin**

At the **top** of `C:\dev\tacet\src\lib.rs`, above `write_silence`, add the imports and implementation:

```rust
//! Tacet — an empty VST3/CLAP instrument that outputs silence.

use nih_plug::prelude::*;
use std::num::NonZeroU32;
use std::sync::Arc;

/// The plugin. Holds no audio state — it deliberately does nothing.
struct Tacet {
    params: Arc<TacetParams>,
}

/// Tacet exposes no parameters.
#[derive(Params, Default)]
struct TacetParams {}

impl Default for Tacet {
    fn default() -> Self {
        Self {
            params: Arc::new(TacetParams::default()),
        }
    }
}

impl Plugin for Tacet {
    const NAME: &'static str = "Tacet";
    const VENDOR: &'static str = "Kapellmeister";
    const URL: &'static str = "https://github.com/Abstraktikus/tacet";
    const EMAIL: &'static str = "martin.nafzger@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // One stereo output, no audio input — the shape of a synth.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    // Accept MIDI so hosts wire it up like a synth; the notes are discarded.
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = false;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Do nothing: emit silence and ignore all incoming MIDI events.
        write_silence(buffer.as_slice());
        ProcessStatus::Normal
    }
}

impl ClapPlugin for Tacet {
    const CLAP_ID: &'static str = "com.kapellmeister.tacet";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("An empty instrument that outputs silence — a placeholder for unused synth slots.");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::Instrument, ClapFeature::Synthesizer, ClapFeature::Stereo];
}

impl Vst3Plugin for Tacet {
    // Stable 16-byte identifier — must never change once released.
    const VST3_CLASS_ID: [u8; 16] = *b"KapellmstrTacet1";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(Tacet);
nih_export_vst3!(Tacet);
```

Keep the existing `write_silence` function (below the impl) and the `#[cfg(test)]` module unchanged from Step 1.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --lib`
Expected: PASS — all six tests green.

If `ClapFeature::Synthesizer` does not resolve, use `ClapFeature::Instrument` alone (the Instrument feature is the required one; Synthesizer is a refinement). Re-run.

- [ ] **Step 5: Bundle the plugin**

Run: `cargo xtask bundle tacet --release`
Expected: build succeeds; artifacts appear at `target/bundled/Tacet.vst3` and `target/bundled/Tacet.clap`.

Verify: `ls target/bundled` shows `Tacet.vst3`.

- [ ] **Step 6: Commit**

```bash
git add src/lib.rs Cargo.lock
git commit -m "$(cat <<'EOF'
Implement Tacet as a silent Instrument/Synth plugin

Declares VST3 Instrument|Synth subcategories, stereo out / no input,
MIDI input (discarded), zero parameters, no editor. process() emits
silence via write_silence. Exports VST3 and CLAP.

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 3: Deploy script, README, and GigPerformer verification

Delivers the deploy path into GigPerformer's VST3 folder plus a README, and a manual verification checklist confirming the plugin loads as a silent instrument.

**Files:**
- Create: `C:\dev\tacet\deploy.ps1`
- Create: `C:\dev\tacet\README.md`

**Interfaces:**
- Consumes: `target/bundled/Tacet.vst3` produced by Task 2.
- Produces: nothing consumed by later tasks (final task).

- [ ] **Step 1: Write the deploy script**

Create `C:\dev\tacet\deploy.ps1`:

```powershell
# Bundles Tacet in release mode and copies the VST3 into the system VST3 folder.
# Run from an ELEVATED PowerShell (writing to Program Files needs admin), or set
# $VstDir to a user-writable folder that GigPerformer is configured to scan.

$ErrorActionPreference = 'Stop'

$VstDir = 'C:\Program Files\Common Files\VST3'
$Source = Join-Path $PSScriptRoot 'target\bundled\Tacet.vst3'

Write-Host 'Bundling Tacet (release)...'
cargo xtask bundle tacet --release
if ($LASTEXITCODE -ne 0) { throw "bundle failed with exit code $LASTEXITCODE" }

if (-not (Test-Path $Source)) { throw "bundle not found at $Source" }
if (-not (Test-Path $VstDir)) { New-Item -ItemType Directory -Path $VstDir | Out-Null }

$Dest = Join-Path $VstDir 'Tacet.vst3'
if (Test-Path $Dest) { Remove-Item -Recurse -Force $Dest }
Copy-Item -Recurse -Force $Source $Dest

Write-Host "Deployed Tacet.vst3 to $VstDir"
```

- [ ] **Step 2: Run the deploy script**

Run (elevated PowerShell): `powershell -ExecutionPolicy Bypass -File .\deploy.ps1`
Expected: prints `Deployed Tacet.vst3 to C:\Program Files\Common Files\VST3`.

If not elevated / access denied: set `$VstDir` to a user VST3 folder GigPerformer scans (check GP → Options → VST plugins → scan paths), then re-run.

- [ ] **Step 3: Write the README**

Create `C:\dev\tacet\README.md`:

```markdown
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
```

- [ ] **Step 4: Commit**

```bash
git add deploy.ps1 README.md
git commit -m "$(cat <<'EOF'
Add deploy script and README

deploy.ps1 bundles and installs Tacet.vst3 into the system VST3 folder;
README documents purpose, build, deploy, and test.

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
EOF
)"
```

- [ ] **Step 5: Manual verification in GigPerformer (user)**

Rescan plugins in GigPerformer 5, then confirm:
- [ ] Tacet appears in the plugin list under manufacturer **Kapellmeister**, categorized as an **instrument/synth**.
- [ ] It can be inserted into a synth plugin block.
- [ ] Sending MIDI produces **no sound and no errors**.
- [ ] Its memory footprint is negligible compared to a real synth in the same slot.

- [ ] **Step 6 (optional): Validate with pluginval**

If [pluginval](https://github.com/Tracktion/pluginval) is available, run it against the bundled VST3 for an independent sanity check:

Run: `pluginval --validate "C:\Program Files\Common Files\VST3\Tacet.vst3"`
Expected: all tests pass (a silent instrument has no DSP to fail).

---

## Notes

- **VST3 class ID is frozen.** `*b"KapellmstrTacet1"` must never change once Tacet is distributed, or hosts will treat updated versions as a different plugin and drop existing block references.
- **The MIDI plugin reuses this scaffold.** The planned "MIDI Processing & Monitoring" plugin is a separate future project; it will start from this same Cargo/xtask/deploy structure, swapping the silent `process()` for MIDI-processing logic and adding an editor via `nih_plug_egui`/`nih_plug_vizia`.
