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

/// Overwrite every sample in every channel with silence (0.0).
///
/// Pure and allocation-free so it is safe to call from the audio thread.
pub(crate) fn write_silence(channels: &mut [&mut [f32]]) {
    for channel in channels {
        for sample in channel.iter_mut() {
            *sample = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
