//! UI system to use the music, reponding to the states of the song and each stems.
//! Other UI components will modify the state of the audio, and this will respond to it
//! by starting the song, muting stems, etc.

use bevy::asset::AssetServer;
use bevy::audio::{AudioPlayer, AudioSink, AudioSinkPlayback};
use bevy::log::{info, warn};
use bevy::prelude::{Commands, Query, Res, ResMut, Resource, With};
use crate::ui::process_audio::{AudioLoadingQueue, BassStemAudio, DrumsStemAudio, OtherStemAudio, VocalsStemAudio};


#[derive(Resource, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub has_changed: bool,
}

pub fn setup_audio_controller(mut commands: Commands) {
    commands.insert_resource(PlaybackState { is_playing: false, has_changed: false });
    commands.insert_resource(VocalsState { is_active: false, has_changed: false });
    commands.insert_resource(DrumsState { is_active: false, has_changed: false });
    commands.insert_resource(BassState { is_active: false, has_changed: false });
    commands.insert_resource(OthersState { is_active: false, has_changed: false });
}

pub fn play_stop_system(
    mut playback_state: ResMut<PlaybackState>,
    loading_queue: ResMut<AudioLoadingQueue>,
    vocals_music_controller: Query<&AudioSink, With<VocalsStemAudio>>,
    bass_music_controller: Query<&AudioSink, With<BassStemAudio>>,
    drums_music_controller: Query<&AudioSink, With<DrumsStemAudio>>,
    other_music_controller: Query<&AudioSink, With<OtherStemAudio>>) {
    if loading_queue.should_spawn {
        // Ignores play/stop system if the audio is not loaded yet.
        return;
    }
    if playback_state.has_changed {
        info!("Play/Stop button action changed. Playing: {}", playback_state.is_playing);
        if let Ok(sink) = vocals_music_controller.single() {
            // There could be a problem with some stems, but assuming once vocals is loaded,
            // rest will be ok too.
            playback_state.has_changed = false;
            if playback_state.is_playing {
                info!("Playing audio.");
                sink.play();
            } else {
                info!("Pausing audio.");
                sink.pause();
            }
        } else {
            warn!("No audio sink found for Vocals.");
        }
        if let Ok(sink) = bass_music_controller.single() {
            if playback_state.is_playing {
                sink.play();
            } else {
                sink.pause();
            }
        } else {
            warn!("No audio sink found for bass.");
        }
        if let Ok(sink) = drums_music_controller.single() {
            if playback_state.is_playing {
                sink.play();
            } else {
                sink.pause();
            }
        } else {
            warn!("No audio sink found for drums.");
        }
        if let Ok(sink) = other_music_controller.single() {
            if playback_state.is_playing {
                sink.play();
            } else {
                sink.pause();
            }
        } else {
            warn!("No audio sink found for other.");
        }
    }
}


#[derive(Resource, Default)]
pub struct VocalsState {
    pub is_active: bool,
    pub has_changed: bool
}

pub fn check_vocals_mute_system(
    mut vocals: ResMut<VocalsState>,
    mut vocals_music_controller: Query<&mut AudioSink, With<VocalsStemAudio>>,
) {
    if vocals.has_changed {
        if let Ok(mut sink) = vocals_music_controller.single_mut() {
            sink.toggle_mute();
            vocals.has_changed = false;
        }
    }
}

#[derive(Resource, Default)]
pub struct BassState {
    pub is_active: bool,
    pub has_changed: bool
}

pub fn check_bass_mute_system(
    mut bass: ResMut<BassState>,
    mut bass_music_controller: Query<&mut AudioSink, With<BassStemAudio>>,
) {
    if bass.has_changed {
        if let Ok(mut sink) = bass_music_controller.single_mut() {
            sink.toggle_mute();
            bass.has_changed = false;
        }
    }
}

#[derive(Resource, Default)]
pub struct DrumsState {
    pub is_active: bool,
    pub has_changed: bool
}

pub fn check_drums_mute_system(
    mut drums: ResMut<DrumsState>,
    mut drums_music_controller: Query<&mut AudioSink, With<DrumsStemAudio>>,
) {
    if drums.has_changed {
        if let Ok(mut sink) = drums_music_controller.single_mut() {
            sink.toggle_mute();
            drums.has_changed = false;
        }
    }
}

#[derive(Resource, Default)]
pub struct OthersState {
    pub is_active: bool,
    pub has_changed: bool
}

pub fn check_other_mute_system(
    mut other: ResMut<OthersState>,
    mut other_music_controller: Query<&mut AudioSink, With<OtherStemAudio>>,
) {
    if other.has_changed {
        if let Ok(mut sink) = other_music_controller.single_mut() {
            sink.toggle_mute();
            other.has_changed = false;
        }
    }
}

pub fn check_and_spawn_stems_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_queue: ResMut<AudioLoadingQueue>,
    mut playback_state: ResMut<PlaybackState>,
) {
    if !loading_queue.should_spawn { return; }

    // Check if Bevy's background asset threads have finished loading every file
    let all_loaded = loading_queue.handles.iter().all(|handle| {
        asset_server.is_loaded_with_dependencies(handle)
    });

    if all_loaded && playback_state.is_playing {
        info!("All stems loaded successfully. Spawning audio players in sync!");

        // Stems are safe to pop out of the vector since we are consuming them
        let mut handles = std::mem::take(&mut loading_queue.handles);

        commands.spawn((AudioPlayer::new(handles.remove(0)), VocalsStemAudio));
        commands.spawn((AudioPlayer::new(handles.remove(0)), BassStemAudio));
        commands.spawn((AudioPlayer::new(handles.remove(0)), DrumsStemAudio));
        commands.spawn((AudioPlayer::new(handles.remove(0)), OtherStemAudio));

        loading_queue.should_spawn = false;
        playback_state.has_changed = false;  // Reset playback. Upon load, files are playing already.
    }
}
