use std::path::PathBuf;
use bevy::color::Color;
use bevy::prelude::{default, BackgroundColor, Commands, Component, Entity, NextState, Node, Query, Res, ResMut, Time, Transform, Val, With};
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

#[derive(Component)]
pub(crate) struct LoadingSpinner;

// 1. Enter State: Start the backend processing task and draw the UI
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

    // Spawn your 2D text, 3D mesh, or UI node representing the loading spinner
    commands.spawn((
        Node { width: Val::Px(50.0), height: Val::Px(50.0), ..default() },
        BackgroundColor(Color::WHITE),
        LoadingSpinner,
    ));
}

// 2. Update Loop: Rotate the spinning visual element
pub fn animate_spinner(time: Res<Time>, mut query: Query<&mut Transform, With<LoadingSpinner>>) {
    for mut transform in &mut query {
        transform.rotate_z(-2.0 * time.delta_secs());
    }
}

// 3. Update Loop: Keep check on backend status
pub fn monitor_backend(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut AudioBackendTask)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut error_msg: ResMut<ErrorTracker>,
    spinner_query: Query<Entity, With<LoadingSpinner>>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(backend_result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).despawn();

            // Cleanup spinner visuals
            for spinner in &spinner_query {
                commands.entity(spinner).despawn();
            }

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
