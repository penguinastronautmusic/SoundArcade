use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) enum AppState {

    Welcome,
    FileDialog,
    ProcessingAudio,
    CoreApplication,
    UnrecoverableError,
    #[default]
    TestComponentDesign
}
