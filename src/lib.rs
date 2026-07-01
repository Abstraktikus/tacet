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
