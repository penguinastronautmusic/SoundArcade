//! UI Components referencing the [`backend`] mod.
//!
//! This uses the [`backend::load_backend`] function to take in an audio file, divide into stems,
//! and process its DB levels.
//!
//! Note that the task is spun up in the background to allow for the UI to continue normally.
//!

use std::path::PathBuf;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;
use crate::audio_processing::AudioProcessingError;
use crate::backend;
use crate::backend::{AppInput, StemAppData};
use crate::ui::app_state::AppState;
use crate::ui::error::ErrorTracker;
use crate::ui::file_handler::SelectedAudioFile;

#[derive(Component)]
pub(crate) struct AudioBackendTask(Task<Result<StemAppData, AudioProcessingError>>);

pub fn start_backend_processing(
    mut commands: Commands,
    audio_file: Res<SelectedAudioFile>
) {
    let file_path = audio_file.0.clone();
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move {
        Ok(backend::load_backend(AppInput { 
            audio_file: file_path,
            output_dir: PathBuf::from("output/"),
            tick_len: Default::default(),
        })?)
    });

    commands.spawn(AudioBackendTask(task));
}

pub fn monitor_backend(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut AudioBackendTask)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut error_msg: ResMut<ErrorTracker>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(backend_result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).despawn();

            match backend_result {
                Ok(_) => next_state.set(AppState::CoreApplication),
                Err(err) => {
                    error_msg.message = format!("Error processing audio: {err}");
                    eprintln!("{}", &error_msg.message);
                    next_state.set(AppState::UnrecoverableError);
                }
            }
        }
    }
}
