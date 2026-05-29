use std::path::PathBuf;
use std::time::Duration;
use crate::audio_processing::{split_audio_into_stems, AudioProcessingError, AudioStems};

pub struct StemAppData {
    main_audio_file: PathBuf,
    vocals: Stem,
    bass: Stem,
    drums: Stem,
    other: Stem
}


pub struct Stem {
    audio_file: PathBuf,
    track_db_per_tick: Vec<usize>
}


pub struct AppInput {
    pub(crate) audio_file: PathBuf,
    pub(crate) output_dir: PathBuf,
    pub(crate) tick_len: Duration
}


pub fn load_backend(app_input: AppInput) -> Result<StemAppData, AudioProcessingError> {
    let processed_audio = split_audio_into_stems(&app_input.audio_file, app_input.output_dir)?;

    // TODO: DB per tick

    Ok(StemAppData {
        main_audio_file: app_input.audio_file,
        vocals: Stem {
            audio_file: processed_audio.vocals,
            track_db_per_tick: vec!()
        },
        bass: Stem {
            audio_file: processed_audio.bass,
            track_db_per_tick: vec!()
        },
        drums: Stem {
            audio_file: processed_audio.drums,
            track_db_per_tick: vec!()
        },
        other: Stem {
            audio_file: processed_audio.other,
            track_db_per_tick: vec!()
        },
    })
}
