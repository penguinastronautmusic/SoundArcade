//! AppStates contains the main states of the app, in a very "Bevy-Style" application
//! control manner.
//!
//! See https://docs.rs/bevy/latest/bevy/state/index.html


use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) enum AppState {
    #[default]
    Welcome,
    FileDialog,
    ProcessingAudio,
    CoreApplication,
    UnrecoverableError,
    TestComponentDesign
}
