//! Module used to separate the backend data from Bevy.
//! This was added in case Bevy needed to be changed later.

use std::path::PathBuf;
use std::time::Duration;
use bevy::prelude::Resource;
use crate::backend;

#[derive(Resource, Default)]
pub struct StemResources {
    pub vocals: StemResource,
    pub bass: StemResource,
    pub drums: StemResource,
    pub other: StemResource,
    pub current_tick: usize,
}

impl StemResources {
    pub fn from_data(data: backend::StemAppData, tick_len: Duration) -> Self {
        Self {
            vocals: StemResource {
                stem_type: TrackType::Vocals,
                stem_path: data.vocals.audio_file,
                db_track: data.vocals.track_db_per_tick,
                tick_len,
                is_active: true,
            },
            bass: StemResource {
                stem_type: TrackType::Bass,
                stem_path: data.bass.audio_file,
                db_track: data.bass.track_db_per_tick,
                tick_len,
                is_active: true,
            },
            drums: StemResource {
                stem_type: TrackType::Drums,
                stem_path: data.drums.audio_file,
                db_track: data.drums.track_db_per_tick,
                tick_len,
                is_active: true,
            },
            other: StemResource {
                stem_type: TrackType::Other,
                stem_path: data.other.audio_file,
                db_track: data.other.track_db_per_tick,
                tick_len,
                is_active: true,
            },
            current_tick: 0,
        }
    }
}

#[derive(Default)]
pub struct StemResource {
    pub stem_type: TrackType,
    pub stem_path: PathBuf,
    pub db_track: Vec<usize>,
    pub tick_len: Duration,
    pub is_active: bool
}

#[derive(Resource, Clone, Debug)]
pub struct AppStartSelections {
    pub tick_len: Duration,
    pub is_dummy_backend: bool
}

#[derive(Copy, Clone, Debug, Default)]
pub enum TrackType {
    #[default]
    Vocals,
    Drums,
    Bass,
    Other
}

