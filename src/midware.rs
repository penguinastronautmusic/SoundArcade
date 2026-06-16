//! Module used to separate the backend data from Bevy.
//! This was added in case Bevy needed to be changed later.

use std::path::PathBuf;
use std::time::Duration;
use bevy::log::{debug};
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
                stem_path: set_dir_to_asset_folder(&data.vocals.audio_file),
                db_track: data.vocals.track_db_per_tick,
                tick_len,
            },
            bass: StemResource {
                stem_type: TrackType::Bass,
                stem_path: set_dir_to_asset_folder(&data.bass.audio_file),
                db_track: data.bass.track_db_per_tick,
                tick_len,
            },
            drums: StemResource {
                stem_type: TrackType::Drums,
                stem_path: set_dir_to_asset_folder(&data.drums.audio_file),
                db_track: data.drums.track_db_per_tick,
                tick_len,
            },
            other: StemResource {
                stem_type: TrackType::Other,
                stem_path: set_dir_to_asset_folder(&data.other.audio_file),
                db_track: data.other.track_db_per_tick,
                tick_len,
            },
            current_tick: 0,
        }
    }
}

/// Helper function to reset the path to the proper asset folder.
/// Bevy's asset server expects everything to already be in the "asset-folder", so no need
/// to keep the prefix, which was necessary to have the full relative path to the project.
fn set_dir_to_asset_folder(stem_path: &PathBuf) -> PathBuf {
    debug!("Setting stem path to asset folder: {:?}", stem_path);
    stem_path.strip_prefix("assets").unwrap().to_owned()
}

#[derive(Default)]
pub struct StemResource {
    pub stem_type: TrackType,
    pub stem_path: PathBuf,
    pub db_track: Vec<usize>,
    pub tick_len: Duration,
}

#[derive(Resource, Clone, Debug)]
pub struct AppStartSelections {
    pub tick_len: Duration,
    pub is_dummy_backend: bool
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum TrackType {
    #[default]
    Vocals,
    Drums,
    Bass,
    Other
}

