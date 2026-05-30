//! The UI mod contains bevy objects that represent the processes and resources of the
//! application.
//! 
//! They utilize Backend and other modules' functions to process audio, divide it into stems,
//! and render different elements on the screen.

pub(crate) mod file_handler;
pub(crate) mod app_state;
pub(crate) mod process_audio;
pub(crate) mod welcome_screen;
pub(crate) mod error;
pub(crate) mod test_design;
pub(crate) mod spinner;
