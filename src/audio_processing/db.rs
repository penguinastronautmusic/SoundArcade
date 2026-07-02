//! Module for calculating the decibel (dB) levels of an audio file.
//! This should be used for each stems.

use std::path::PathBuf;
use std::time::Duration;
use bevy::log::{debug, info, warn};
use crate::audio_processing::error::AudioProcessingError;

/// Calculates the decibel (dB) levels of an audio file in WAV format over a specified time interval.
/// Uses the [hound](https://docs.rs/hound/latest/hound/) crate.
///
/// This function processes an audio file and computes the dB levels for each interval of the
/// specified `tick_len` duration. It supports mono and stereo audio channels and averages
/// multichannel audio into a single mono-like analysis for dB level measurement.
///
/// # Parameters
///
/// - `path`: A reference to the `PathBuf` representing the path to the input WAV audio file.
/// - `tick_len`: A `Duration` specifying the length of each time interval (tick) for which
///   the dB levels should be calculated.
///
/// # Returns
///
/// - `Ok(Vec<usize>)`: A vector of scaled dB levels for each tick, where each value
///   is a scaled intensity between 0 (silence or -60 dB) and 100 (maximum 0 dB).
/// - `Err(AudioProcessingError)`: Returns an error if there is an issue, such as
///   an invalid audio file format or file read failure.
///
/// # Supported Formats
///
/// - Input audio must be in WAV format.
/// - Both `Float` and `Int` sample formats are supported. Integer samples are normalized to the
///   range [-1.0, 1.0] based on the sample's maximum possible value.
///
/// # Example
///
/// ```rust
/// use std::path::PathBuf;
/// use std::time::Duration;
/// use crate::calculate_db_levels; // Adjust import path as needed
///
/// let path = PathBuf::from("audio.wav");
/// let tick_len = Duration::from_millis(100);
///
/// match calculate_db_levels(&path, tick_len) {
///     Ok(levels) => {
///         for (i, level) in levels.iter().enumerate() {
///             println!("Tick {}: dB Level {}", i, level);
///         }
///     }
///     Err(e) => eprintln!("Error processing audio: {:?}", e),
/// }
/// ```
///
/// # Errors
///
/// - Returns an `AudioProcessingError` if:
///   - The WAV file cannot be opened or read.
///   - The file format is unsupported or invalid.
pub fn calculate_db_levels(path: &PathBuf, tick_len: &Duration) -> Result<Vec<usize>, AudioProcessingError> {
    info!("Calculating DB levels of {:?}...", path);
    let mut reader = hound::WavReader::open(path)
        .map_err(|e| AudioProcessingError::InvalidFile(format!("Failed to open WAV file: {}", e)))?;

    let spec = reader.spec();
    let samples_per_tick = (spec.sample_rate as f64 * tick_len.as_secs_f64()) as usize;
    let num_channels = spec.channels as usize;

    let all_samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect(),
        hound::SampleFormat::Int => {
            let max_val = (1 << (spec.bits_per_sample - 1)) as f32;
            reader.samples::<i32>().map(|s| s.unwrap_or(0) as f32 / max_val).collect()
        }
    };

    let mut db_levels = Vec::new();

    let samples_to_process = samples_per_tick * num_channels;

    if samples_to_process == 0 {
        warn!("No samples to process. Returning an empty track.");
        return Ok(vec![]);
    }

    for chunk in all_samples.chunks(samples_to_process) {
        let mut sum_sq = 0.0;
        for &sample in chunk {
            sum_sq += sample * sample;
        }
        let rms = (sum_sq / chunk.len() as f32).sqrt();

        let db = 20.0 * (rms.max(0.00001)).log10();

        let scaled_db = ((db + 60.0) * (100.0 / 60.0)).clamp(0.0, 100.0) as usize;
        db_levels.push(scaled_db);
    }

    debug!("Successfully parsed {:?} chunks.", db_levels.len());
    Ok(db_levels)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_calculation_logic() {
        let temp_dir = std::env::temp_dir();
        let wav_path = temp_dir.join("test_silence.wav");

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        {
            let mut writer = hound::WavWriter::create(&wav_path, spec).unwrap();
            // 1 second of silence
            for _ in 0..44100 {
                writer.write_sample(0i16).unwrap();
            }
            // 1 second of "noise" (alternating max/min for high RMS)
            for _ in 0..44100 {
                writer.write_sample(10000i16).unwrap();
            }
            writer.finalize().unwrap();
        }

        let tick_len = Duration::from_millis(1000); // 1-second ticks
        let levels = calculate_db_levels(&wav_path, &tick_len).unwrap();

        assert_eq!(levels.len(), 2);
        assert_eq!(levels[0], 0); // Silence should be 0 (scaled)
        assert!(levels[1] > 0);   // Noise should be > 0

        let _ = std::fs::remove_file(wav_path);
    }
}