use bevy::audio::{AudioSink, AudioSinkPlayback};
use bevy::log::{info, warn};
use bevy::prelude::{Commands, Query, ResMut, Resource, With};
use crate::ui::process_audio::{BassStemAudio, DrumsStemAudio, OtherStemAudio, VocalsStemAudio};


#[derive(Resource, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub has_changed: bool
}


pub fn setup_audio_controller(mut commands: Commands) {
    // Starting Playback state to "Changed==True" because Bevy plays the audio on load.
    // See ProcessAudio for loading of the audio files.
    commands.insert_resource(PlaybackState { is_playing: false, has_changed: true });
    commands.insert_resource(VocalsState { is_active: false, has_changed: false });
    commands.insert_resource(DrumsState { is_active: false, has_changed: false });
    commands.insert_resource(BassState { is_active: false, has_changed: false });
    commands.insert_resource(OthersState { is_active: false, has_changed: false });
}

pub fn play_stop_system(
    mut playback_state: ResMut<PlaybackState>,
    vocals_music_controller: Query<&AudioSink, With<VocalsStemAudio>>,
    bass_music_controller: Query<&AudioSink, With<BassStemAudio>>,
    drums_music_controller: Query<&AudioSink, With<DrumsStemAudio>>,
    other_music_controller: Query<&AudioSink, With<OtherStemAudio>>) {
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
