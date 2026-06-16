//! UI system to spawn and handle play, stop and reset buttons.
//! This is meant to be used in the core system, when the song is playing.
use bevy::asset::Handle;
use bevy::camera::{Camera, Camera2d};
use bevy::color::Color;
use bevy::image::Image;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Component, Entity, GlobalTransform, MouseButton, Query, Res, ResMut, Resource, Sprite, Transform, Window, With};
use crate::ui::audio_controller::{BassState, DrumsState, OthersState, PlaybackState, VocalsState};

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct PauseButton;

#[derive(Resource)]
pub struct PlayImage(pub Handle<Image>);
#[derive(Resource)]
pub struct PauseImage(pub Handle<Image>);

pub fn setup_play_stop_reset_buttons(
    mut commands: Commands,
    play_image: Res<PlayImage>,
) {
    // Spawn Play Button
    commands.spawn((
        Sprite {
            image: play_image.0.clone(),
            color: Color::srgb(5.0, 5.0, 5.0),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        PlayButton,
    ));
}

pub fn initial_button_system(
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut play_button_query: Query<(Entity, &GlobalTransform), With<PlayButton>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut playback_state: ResMut<PlaybackState>,
    mut vocals_state: ResMut<VocalsState>,
    mut drums_state: ResMut<DrumsState>,
    mut bass_state: ResMut<BassState>,
    mut other_state: ResMut<OthersState>,
    pause_image: Res<PauseImage>,
) {
    let window = if let Ok(w) = window_query.single() { w } else { return };
    let (camera, camera_transform) = if let Ok(c) = camera_query.single() { c } else { return };

    let cursor_world_pos = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok());

    if let Some(world_position) = cursor_world_pos {
        for (entity, transform) in play_button_query.iter_mut() {
            let pos = transform.translation().truncate();
            let distance = world_position.distance(pos);

            // 50 click radius for the play button
            if distance < 50.0 && mouse_button_input.just_pressed(MouseButton::Left) {
                // Not interfering with "Has_Changed" because playing the song
                // already unmutes all tracks.
                vocals_state.is_active = true;
                drums_state.is_active = true;
                bass_state.is_active = true;
                other_state.is_active = true;
                playback_state.is_playing = true;
                playback_state.has_changed = true;

                // Spawn Pause Button
                commands.spawn((
                    Sprite {
                        image: pause_image.0.clone(),
                        color: Color::srgb(3.0, 3.0, 3.0),
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(540.0, 300.0, 10.0)),
                    PauseButton,
                ));

                commands.entity(entity).despawn();
                info!("Play button clicked.");
            }
        }
    }
}

pub fn play_pause_button_system(
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut pause_button_query: Query<(Entity, &GlobalTransform), With<PauseButton>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut playback_state: ResMut<PlaybackState>,
    play_image: Res<PlayImage>,
    pause_image: Res<PauseImage>,
) {
    if playback_state.has_changed {
        return;
    }
    
    let window = if let Ok(w) = window_query.single() { w } else { return };
    let (camera, camera_transform) = if let Ok(c) = camera_query.single() { c } else { return };

    let cursor_world_pos = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok());

    if let Some(world_position) = cursor_world_pos {
        for (entity, transform) in pause_button_query.iter_mut() {
            let pos = transform.translation().truncate();
            let distance = world_position.distance(pos);

            if distance < 50.0 && mouse_button_input.just_pressed(MouseButton::Left) {
                playback_state.is_playing = !playback_state.is_playing;
                playback_state.has_changed = true;
                
                let image = if playback_state.is_playing {
                    pause_image.0.clone()
                } else {
                    play_image.0.clone()
                };

                // Spawn Other (Play/Pause) Button
                commands.spawn((
                    Sprite {
                        image: image,
                        color: Color::srgb(3.0, 3.0, 3.0),
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(540.0, 300.0, 10.0)),
                    PauseButton,
                ));
                commands.entity(entity).despawn();
                info!("Pause/Replay button clicked.");
            }
        }
    }
}