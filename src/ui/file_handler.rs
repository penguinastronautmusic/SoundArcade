use std::path::PathBuf;
use bevy::prelude::{Commands, Component, Entity, NextState, Query, ResMut, Resource};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;
use rfd::AsyncFileDialog;
use crate::ui::app_state::AppState;

#[derive(Component)]
pub(crate) struct FilePickTask(Task<Option<PathBuf>>);

pub(crate) fn trigger_file_dialog(mut commands: Commands) {
    let thread_pool = AsyncComputeTaskPool::get();

    // Spawn the dialog onto the async thread pool
    let task = thread_pool.spawn(async move {
        let file_handle = AsyncFileDialog::new()
            .add_filter("audio", &["mp3", "wav", "ogg"])
            .pick_file()
            .await;

        file_handle.map(|f| f.path().to_path_buf())
    });

    commands.spawn(FilePickTask(task));
}

#[derive(Resource)]
pub struct SelectedAudioFile(pub PathBuf);

pub(crate) fn poll_file_dialog(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut FilePickTask)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).despawn();

            if let Some(path) = result {
                commands.insert_resource(SelectedAudioFile(path));
                next_state.set(AppState::ProcessingAudio);
            } else {
                next_state.set(AppState::Welcome);
            }
        }
    }
}
