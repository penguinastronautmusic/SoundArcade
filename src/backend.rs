//! The backend module contains the core and shared data used by the main systems of the application.
//! 

use std::path::PathBuf;
use std::time::Duration;
use bevy::prelude::*;
use crate::audio_processing::db::calculate_db_levels;
use crate::audio_processing::error::AudioProcessingError;
use crate::audio_processing::stems::split_audio_into_stems;

#[derive(Clone)]
pub struct StemAppData {
    pub main_audio_file: PathBuf,
    pub vocals: Stem,
    pub bass: Stem,
    pub drums: Stem,
    pub other: Stem
}

#[derive(Clone)]
pub struct Stem {
    pub audio_file: PathBuf,
    pub track_db_per_tick: Vec<usize>,
}


pub struct AppInput {
    pub(crate) audio_file: PathBuf,
    pub(crate) output_dir: PathBuf,
    pub(crate) tick_len: Duration,
    pub(crate) is_dummy: bool,
}


/// Processes the audio
pub fn load_backend(app_input: AppInput) -> Result<StemAppData, AudioProcessingError> {
    info!("Loading backend from file {:?}", app_input.audio_file);

    if app_input.is_dummy {
        let backend_dummy = load_mock_backend_stems(app_input);
        info!("Successfully loaded dummy stem data.");
        return backend_dummy;
    }

    info!("...splitting audio into 4 stems...");
    let processed_audio = split_audio_into_stems(&app_input.audio_file, app_input.output_dir)?;

    info!("Split successful! Calculating db data per tick.");
    let vocal_db_levels = calculate_db_levels(&processed_audio.vocals, &app_input.tick_len)?;
    let bass_db_levels = calculate_db_levels(&processed_audio.bass, &app_input.tick_len)?;
    let drums_db_levels = calculate_db_levels(&processed_audio.drums, &app_input.tick_len)?;
    let other_db_levels = calculate_db_levels(&processed_audio.other, &app_input.tick_len)?;

    info!("Backend successfully loaded. Returning stem data.");
    Ok(StemAppData {
        main_audio_file: app_input.audio_file,
        vocals: Stem {
            audio_file: processed_audio.vocals,
            track_db_per_tick: vocal_db_levels,
        },
        bass: Stem {
            audio_file: processed_audio.bass,
            track_db_per_tick: bass_db_levels,
        },
        drums: Stem {
            audio_file: processed_audio.drums,
            track_db_per_tick: drums_db_levels,
        },
        other: Stem {
            audio_file: processed_audio.other,
            track_db_per_tick: other_db_levels,
        },
    })
}

/// Loads a dummy backend data from a randomized sinusoidal wave.
/// This helps to test the UI or previewing it.
pub fn load_mock_backend_stems(app_input: AppInput) -> Result<StemAppData, AudioProcessingError> {
    info!("Loading a mocked backend data without proper stem audio.");
    let total_seconds = 180;
    let ticks_per_second = 1.0 / app_input.tick_len.as_secs_f64();
    let total_ticks = (total_seconds as f64 * ticks_per_second).round() as usize;

    // Helper closure to generate pseudo-random sinusoidal dB values between 0 and 100
    let generate_sinusoidal_db = |phase_offset: f64, frequency: f64| -> Vec<usize> {
        (0..total_ticks)
            .map(|tick| {
                let t = tick as f64;
                // Base sine wave oscillating between -1.0 and 1.0
                let wave = (t * frequency + phase_offset).sin();

                // Deterministic pseudo-random noise based on the tick index
                let noise = ((t * 12.9898 + phase_offset).sin() * 43758.5453).fract();

                // Combine wave (scaled to 10-90) with small noise variance (+/- 5)
                let base_db = 50.0 + (wave * 40.0) + (noise * 10.0 - 5.0);

                // Clamp and cast to valid usize dB value
                base_db.clamp(0.0, 100.0) as usize
            })
            .collect()
    };

    Ok(StemAppData {
        main_audio_file: app_input.audio_file,
        vocals: Stem {
            audio_file: PathBuf::from("/dummy_directory/example_vocals.wav"),
            track_db_per_tick: generate_sinusoidal_db(0.0, 0.05)
        },
        bass: Stem {
            audio_file: PathBuf::from("/dummy_directory/example_bass.wav"),
            track_db_per_tick: generate_sinusoidal_db(2.0, 0.02)
        },
        drums: Stem {
            audio_file: PathBuf::from("/dummy_directory/example_drums.wav"),
            track_db_per_tick: generate_sinusoidal_db(4.0, 0.08)
        },
        other: Stem {
            audio_file: PathBuf::from("/dummy_directory/example_other.wav"),
            track_db_per_tick: generate_sinusoidal_db(1.5, 0.04)
        },
    })
}
