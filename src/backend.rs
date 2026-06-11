//! The backend module contains the core and shared data used by the main systems of the application.
//! 

use std::path::PathBuf;
use std::time::Duration;
use crate::audio_processing::{calculate_db_levels, split_audio_into_stems, AudioProcessingError};

pub struct StemAppData {
    pub main_audio_file: PathBuf,
    pub vocals: Stem,
    pub bass: Stem,
    pub drums: Stem,
    pub other: Stem
}


pub struct Stem {
    pub audio_file: PathBuf,
    pub track_db_per_tick: Vec<usize>
}


pub struct AppInput {
    pub(crate) audio_file: PathBuf,
    pub(crate) output_dir: PathBuf,
    pub(crate) tick_len: Duration
}


pub fn load_backend(app_input: AppInput) -> Result<StemAppData, AudioProcessingError> {
    let processed_audio = split_audio_into_stems(&app_input.audio_file, app_input.output_dir)?;

    let vocals_db = calculate_db_levels(&processed_audio.vocals, app_input.tick_len)?;
    let bass_db = calculate_db_levels(&processed_audio.bass, app_input.tick_len)?;
    let drums_db = calculate_db_levels(&processed_audio.drums, app_input.tick_len)?;
    let other_db = calculate_db_levels(&processed_audio.other, app_input.tick_len)?;

    Ok(StemAppData {
        main_audio_file: app_input.audio_file,
        vocals: Stem {
            audio_file: processed_audio.vocals,
            track_db_per_tick: vocals_db
        },
        bass: Stem {
            audio_file: processed_audio.bass,
            track_db_per_tick: bass_db
        },
        drums: Stem {
            audio_file: processed_audio.drums,
            track_db_per_tick: drums_db
        },
        other: Stem {
            audio_file: processed_audio.other,
            track_db_per_tick: other_db
        },
    })
}
