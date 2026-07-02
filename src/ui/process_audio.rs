//! UI Components referencing the [`backend`] mod.
//!
//! This uses the [`backend::load_backend`] function to take in an audio file, divide into stems,
//! and process its DB levels.
//! 
//! The Midware system will then keep the data in memory for the other components to use.
//!
//! Note that the task is spun up in the background to allow for the UI to continue normally.

use std::path::PathBuf;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;
use crate::audio_processing::error::AudioProcessingError;
use crate::backend;
use crate::backend::{AppInput, StemAppData};
use crate::midware::{AppStartSelections, StemResources};
use crate::ui::app_state::AppState;
use crate::ui::error::ErrorTracker;
use crate::ui::file_handler::SelectedAudioFile;

#[derive(Component)]
pub(crate) struct AudioBackendTask(Task<Result<StemAppData, AudioProcessingError>>);

pub fn start_backend_processing(
    mut commands: Commands,
    audio_file: Res<SelectedAudioFile>,
    start_args: Res<AppStartSelections>
) {
    let file_path = audio_file.0.clone();
    info!("Processing audio file at path {:?}", file_path);

    let args = start_args.clone();
    let thread_pool = AsyncComputeTaskPool::get();

    info!("Starting audio processing thread pool. Args: {:?}", &args);

    let task = thread_pool.spawn(async move {
        Ok(backend::load_backend(AppInput { 
            audio_file: file_path,
            output_dir: PathBuf::from("assets/output/"),
            tick_len: args.tick_len,
            is_dummy: args.is_dummy_backend
        })?)
    });

    commands.spawn(AudioBackendTask(task));
}

#[derive(Component)]
pub(crate) struct VocalsStemAudio;
#[derive(Component)]
pub(crate) struct BassStemAudio;
#[derive(Component)]
pub(crate) struct DrumsStemAudio;
#[derive(Component)]
pub(crate) struct OtherStemAudio;

#[derive(Resource, Default)]
pub struct AudioLoadingQueue {
    pub handles: Vec<Handle<AudioSource>>,
    pub should_spawn: bool,
}

pub fn monitor_backend(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut AudioBackendTask)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut error_msg: ResMut<ErrorTracker>,
    app_start_selections: Res<AppStartSelections>,
    asset_server: Res<AssetServer>
) {
    for (entity, mut task) in &mut tasks {
        if let Some(backend_result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).despawn();

            match backend_result {
                Ok(data) => {
                    info!("Audio processing completed successfully. Loading audio into Bevy.");
                    let stem_resources = StemResources::from_data(data, app_start_selections.tick_len);

                    info!("Starting asset server stem loading processes asynchronously.");
                    let handles = vec![
                        asset_server.load(stem_resources.vocals.stem_path.clone()),
                        asset_server.load(stem_resources.bass.stem_path.clone()),
                        asset_server.load(stem_resources.drums.stem_path.clone()),
                        asset_server.load(stem_resources.other.stem_path.clone()),
                    ];

                    commands.insert_resource(AudioLoadingQueue{handles, should_spawn: true });

                    commands.insert_resource(stem_resources);
                    next_state.set(AppState::CoreApplication);
                }
                Err(err) => {
                    error_msg.message = format!("Error processing audio: {err}");
                    eprintln!("{}", &error_msg.message);
                    next_state.set(AppState::UnrecoverableError);
                }
            }
        }
    }
}
